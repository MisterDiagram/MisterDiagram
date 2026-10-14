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
fn instances_keep_overrides_and_changed_inherited_caches() {
    let mut d = skeleton::document(false);
    let mut s = diagramcraft_model::Shape { id: 1, master: Some(2), shape_type: Some("Shape".into()), ..Default::default() };
    s.sheet.cells.push(Cell { name: "BeginX".into(), value: Some("2".into()), ..Default::default() });
    d.pages[0].shapes.push(s);
    let a = import(&write(&d).unwrap().bytes).unwrap();
    let cells = &a.pages[0].shapes[0].sheet.cells;
    assert_eq!(cells.len(), 4);
    for (name, value) in [("PinX", "2"), ("Width", "0"), ("LocPinX", "0")] {
        let cell = cells.iter().find(|c| c.name == name).unwrap();
        assert_eq!(cell.formula.as_deref(), Some("Inh"));
        assert_eq!(cell.value.as_deref(), Some(value));
    }
    assert!(!cells.iter().any(|c| c.name == "EndX"));
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

// Source V text is a serialization contract, independently of the evaluator's value.
fn raw_cells(root: &XmlElement) -> Vec<(String, Option<String>, Option<String>, Option<String>, Option<String>)> {
    let mut cells = Vec::new();
    let mut stack = vec![(String::new(), root)];
    while let Some((path, node)) = stack.pop() {
        let path = format!("{path}/{}:{:?}:{:?}:{:?}", node.name, node.attr("ID"), node.attr("N"), node.attr("IX"));
        if node.is_visio("Cell") {
            cells.push((
                path.clone(),
                node.attr("V").map(str::to_string),
                node.attr("F").map(str::to_string),
                node.attr("U").map(str::to_string),
                node.attr("E").map(str::to_string),
            ));
        }
        for child in node.elements() {
            stack.push((path.clone(), child));
        }
    }
    cells.sort();
    cells
}
#[test]
fn unchanged_arrow_and_probe_preserve_every_local_cache_and_unit() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    if !root.exists() {
        eprintln!("skip: private corpus/ is absent");
        return;
    }
    for file in ["generated/arrow/begin-00.vsdx", "generated/basic/0001-probe.vsdx", "ms-shipped/1033/SITACC_M.vssx"] {
        let before = diagramcraft_vsdx::read(root.join(file)).unwrap();
        let after = import(&write(&before).unwrap().bytes).unwrap();
        for (path, p) in &before.parts {
            if let Some(xml) = &p.xml {
                assert_eq!(raw_cells(xml), raw_cells(after.parts[path].xml.as_ref().unwrap()), "{file}: {path}");
            }
        }
    }
}
#[test]
fn changed_dependency_recalculates_only_affected_caches() {
    let mut d = skeleton::document(false);
    let mut s = skeleton::rectangle(1, 1., 1., 2., 1., "").unwrap();
    s.sheet.cells.extend([
        Cell { name: "Affected".into(), formula: Some("Width*2".into()), value: Some("4.0000".into()), ..Default::default() },
        Cell { name: "Unrelated".into(), formula: Some("Height*2".into()), value: Some("2.0000".into()), ..Default::default() },
    ]);
    d.pages[0].shapes.push(s);
    let mut d = import(&write(&d).unwrap().bytes).unwrap();
    // Preserve an existing textual precision that differs from n.to_string().
    let xml = &mut d.parts.get_mut("visio/pages/page1.xml").unwrap().xml;
    fn rewrite(e: &mut XmlElement) {
        if e.is_visio("Cell") && e.attr("N") == Some("Unrelated") {
            e.attributes.insert("V".into(), "2.0000".into());
        }
        for child in &mut e.children {
            if let XmlNode::Element(e) = child {
                rewrite(e);
            }
        }
    }
    rewrite(xml.as_mut().unwrap());
    let s = &mut d.pages[0].shapes[0];
    s.sheet.cells.iter_mut().find(|c| c.name == "Unrelated").unwrap().value = Some("2.0000".into());
    s.sheet.cells.iter_mut().find(|c| c.name == "Width").unwrap().value = Some("3".into());
    let after = import(&write(&d).unwrap().bytes).unwrap();
    assert_eq!(after.pages[0].shapes[0].sheet.cell("Affected").unwrap().value.as_deref(), Some("6"));
    assert_eq!(after.pages[0].shapes[0].sheet.cell("Unrelated").unwrap().value.as_deref(), Some("2.0000"));
}
#[test]
fn authored_cache_types_do_not_leak_evaluator_ids_or_empty_zeroes() {
    let mut d = skeleton::document(false);
    let mut s = skeleton::rectangle(1, 1., 1., 2., 1., "").unwrap();
    s.sheet.cells.extend([
        Cell { name: "Comment".into(), formula: Some("\"\"".into()), value: Some("wrong".into()), ..Default::default() },
        Cell { name: "TextBkgnd".into(), formula: Some("26".into()), value: Some("wrong".into()), ..Default::default() },
    ]);
    s.sheet.sections.push(Section {
        name: "Character".into(),
        rows: vec![Row {
            index: Some(0),
            cells: vec![
                Cell { name: "LangID".into(), formula: Some("1033".into()), ..Default::default() },
                Cell { name: "Font".into(), formula: Some("FONT(\"Calibri\")".into()), ..Default::default() },
                Cell { name: "Color".into(), formula: Some("0".into()), ..Default::default() },
            ],
            ..Default::default()
        }],
        ..Default::default()
    });
    d.colors.insert(25, "#ffffff".into());
    d.pages[0].shapes.push(s);
    let after = import(&write(&d).unwrap().bytes).unwrap();
    let s = &after.pages[0].shapes[0].sheet;
    assert_eq!(s.cell("Comment").unwrap().value.as_deref(), Some(""));
    assert_eq!(s.cell("TextBkgnd").unwrap().value.as_deref(), Some("#ffffff"));
    let cells = &s.sections.iter().find(|s| s.name == "Character").unwrap().rows[0].cells;
    assert_eq!(cells[0].value.as_deref(), Some("en-US"));
    assert_eq!(cells[1].value.as_deref(), Some("0"));
    assert_eq!(cells[2].value.as_deref(), Some("#000000"));
    assert!(after.parts["visio/document.xml"].xml.as_ref().unwrap().child("FaceNames").unwrap().elements().all(|f| f.attr("ID").is_none()));
}

#[test]
fn writer_text_metrics_are_internal_inches_without_rescaling_width_references() {
    struct FixedMetrics;
    impl diagramcraft_model::text::TextHost for FixedMetrics {
        fn measure(&self, _: &diagramcraft_model::text::TextBody, _: Option<f64>) -> Result<(f64, f64), String> {
            Ok((72., 36.))
        }
    }
    let mut d = skeleton::document(false);
    d.pages[0].sheet.cells.iter_mut().find(|c| c.name == "DrawingScale").unwrap().value = Some("100".into());
    d.pages[0].sheet.cells.iter_mut().find(|c| c.name == "PageScale").unwrap().value = Some("1".into());
    let mut s = skeleton::rectangle(1, 1., 1., 2., 1., "A").unwrap();
    for (name, formula) in [("TxtWidth", "TEXTWIDTH(TheText)"), ("TxtHeight", "TEXTHEIGHT(TheText,TxtWidth)"), ("TxtPinX", "Width/2")] {
        s.sheet.cells.retain(|c| c.name != name);
        s.sheet.cells.push(Cell { name: name.into(), formula: Some(formula.into()), ..Default::default() });
    }
    d.pages[0].shapes.push(s);
    let after = import(&diagramcraft_vsdx::write_with_hosts(&d, None, Some(&FixedMetrics)).unwrap().bytes).unwrap();
    let s = &after.pages[0].shapes[0].sheet;
    assert_eq!(s.cell("TxtWidth").unwrap().value.as_deref(), Some("1"));
    assert_eq!(s.cell("TxtHeight").unwrap().value.as_deref(), Some("0.5"));
    assert_eq!(s.cell("TxtPinX").unwrap().value.as_deref(), Some("1"));
}
#[test]
fn inherited_local_cache_is_not_replaced_by_the_provider() {
    let mut d = skeleton::document(false);
    let mut s = diagramcraft_model::Shape { id: 1, master: Some(2), shape_type: Some("Shape".into()), ..Default::default() };
    s.sheet.cells.push(Cell {
        name: "Width".into(),
        value: Some("999.00".into()),
        formula: Some("Inh".into()),
        units: Some("IN".into()),
        ..Default::default()
    });
    d.pages[0].shapes.push(s);
    let mut d = import(&write(&d).unwrap().bytes).unwrap();
    let c = d.pages[0].shapes[0].sheet.cells.iter_mut().find(|c| c.name == "Width").unwrap();
    c.value = Some("999.00".into());
    let e = d.parts.get_mut("visio/pages/page1.xml").unwrap().xml.as_mut().unwrap();
    for n in &mut e.children {
        if let XmlNode::Element(shapes) = n {
            for n in &mut shapes.children {
                if let XmlNode::Element(shape) = n {
                    for n in &mut shape.children {
                        if let XmlNode::Element(c) = n
                            && c.attr("N") == Some("Width")
                        {
                            c.attributes.insert("V".into(), "999.00".into());
                        }
                    }
                }
            }
        }
    }
    let a = import(&write(&d).unwrap().bytes).unwrap();
    let c = a.pages[0].shapes[0].sheet.cell("Width").unwrap();
    assert_eq!(c.value.as_deref(), Some("999.00"));
    assert_eq!(c.formula.as_deref(), Some("Inh"));
    assert_eq!(c.units.as_deref(), Some("IN"));
}

#[test]
fn no_ix_connection_row_uses_row_position_for_both_coordinates() {
    let mut d = skeleton::document(false);
    let mut s = skeleton::rectangle(1, 1., 1., 2., 3., "").unwrap();
    s.sheet.sections.push(Section {
        name: "Connection".into(),
        rows: vec![Row {
            row_type: Some("Connection".into()),
            cells: vec![
                Cell { name: "X".into(), formula: Some("Width*0.5".into()), value: Some("999".into()), ..Default::default() },
                Cell { name: "Y".into(), formula: Some("Height*0.5".into()), value: Some("999".into()), ..Default::default() },
            ],
            ..Default::default()
        }],
        ..Default::default()
    });
    d.pages[0].shapes.push(s);
    let a = import(&write(&d).unwrap().bytes).unwrap();
    let row = &a.pages[0].shapes[0].sheet.sections.iter().find(|s| s.name == "Connection").unwrap().rows[0];
    assert_eq!(row.cells[0].value.as_deref(), Some("1"));
    assert_eq!(row.cells[1].value.as_deref(), Some("1.5"));
}

#[test]
fn row_type_and_palette_edits_dirty_dependent_caches() {
    let mut d = skeleton::document(false);
    d.colors.insert(25, "#110000".into());
    let mut s = skeleton::rectangle(1, 1., 1., 2., 1., "").unwrap();
    for (n, f) in [("PathMeasure", "PATHLENGTH(Geometry1.Path)"), ("RedMeasure", "RED(25)"), ("RedDependent", "RedMeasure+1")] {
        s.sheet.cells.push(Cell { name: n.into(), formula: Some(f.into()), ..Default::default() });
    }
    d.pages[0].shapes.push(s);
    let mut d = import(&write(&d).unwrap().bytes).unwrap();
    let old = d.pages[0].shapes[0].sheet.cell("PathMeasure").unwrap().value.clone();
    assert_eq!(d.pages[0].shapes[0].sheet.cell("RedMeasure").unwrap().value.as_deref(), Some("17"));
    d.colors.insert(25, "#220000".into());
    let palette_only = import(&write(&d).unwrap().bytes).unwrap();
    assert_eq!(palette_only.pages[0].shapes[0].sheet.cell("RedMeasure").unwrap().value.as_deref(), Some("34"));
    assert_eq!(palette_only.pages[0].shapes[0].sheet.cell("RedDependent").unwrap().value.as_deref(), Some("35"));
    assert_eq!(palette_only.pages[0].shapes[0].sheet.cell("PathMeasure").unwrap().value, old);
    d.colors.insert(25, "#110000".into());
    d.pages[0].shapes[0].sheet.sections.iter_mut().find(|s| s.name == "Geometry").unwrap().rows[1].row_type = Some("MoveTo".into());
    let a = import(&write(&d).unwrap().bytes).unwrap();
    let sheet = &a.pages[0].shapes[0].sheet;
    assert_ne!(sheet.cell("PathMeasure").unwrap().value, old);
    assert_eq!(sheet.cell("RedMeasure").unwrap().value.as_deref(), Some("17"));
    assert_eq!(sheet.cell("RedDependent").unwrap().value.as_deref(), Some("18"));
}

#[test]
fn resized_basic_rectangle_reproduces_inherited_instance_cache_set() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    if !root.exists() {
        eprintln!("skip: private corpus/ is absent");
        return;
    }
    let source = diagramcraft_vsdx::read(root.join("generated/master/basic-rect.vsdx")).unwrap();
    fn caches(d: &diagramcraft_model::Document) -> std::collections::BTreeMap<String, (Option<String>, Option<String>, Option<String>)> {
        let e = diagramcraft_vsdx::evaluate::Evaluator::new(d).unwrap();
        e.source_named_cells(diagramcraft_vsdx::evaluate::Scope::PageShape { page: 0, shape: 1 })
            .unwrap()
            .into_iter()
            .map(|n| (n.name, (n.cell.value.clone(), n.cell.formula.clone(), n.cell.units.clone())))
            .collect()
    }
    let expected = caches(&source);
    for rebuild in [false, true] {
        let mut d = source.clone();
        if rebuild {
            let s = &mut d.pages[0].shapes[0].sheet;
            s.cells.retain(|c| c.formula.as_deref() != Some("Inh"));
            for s in &mut s.sections {
                for r in &mut s.rows {
                    r.cells.retain(|c| c.formula.as_deref() != Some("Inh"));
                }
            }
        }
        let a = import(&write(&d).unwrap().bytes).unwrap();
        let actual = caches(&a);
        assert_eq!(actual.keys().collect::<Vec<_>>(), expected.keys().collect::<Vec<_>>(), "rebuild={rebuild}");
        for (name, (v, f, u)) in &expected {
            let (av, af, au) = &actual[name];
            assert_eq!(af, f, "{name}");
            assert_eq!(au, u, "{name}");
            match (v.as_deref().and_then(|s| s.parse::<f64>().ok()), av.as_deref().and_then(|s| s.parse::<f64>().ok())) {
                (Some(a), Some(b)) => assert!((a - b).abs() <= 1e-6, "{name}: {a} != {b}"),
                _ => assert_eq!(av, v, "{name}"),
            }
        }
    }
}

#[test]
fn inherited_rows_keep_effective_indices_and_named_identity() {
    let mut d = skeleton::document(false);
    let master = &mut d.masters[0].shapes[0];
    master.sheet.sections = vec![
        Section {
            name: "Connections".into(),
            rows: (0..2)
                .map(|i| Row {
                    cells: vec![Cell {
                        name: "X".into(),
                        value: Some((i + 1).to_string()),
                        formula: Some(format!("Width*{}", i + 1)),
                        ..Default::default()
                    }],
                    ..Default::default()
                })
                .collect(),
            ..Default::default()
        },
        Section {
            name: "User".into(),
            rows: vec![Row {
                name: Some("Named".into()),
                index: Some(7),
                cells: vec![Cell { name: "Value".into(), value: Some("1".into()), formula: Some("Width".into()), ..Default::default() }],
                ..Default::default()
            }],
            ..Default::default()
        },
    ];
    let mut instance = diagramcraft_model::Shape { id: 1, master: Some(2), ..Default::default() };
    instance.sheet.cells.push(Cell { name: "Width".into(), value: Some("2".into()), ..Default::default() });
    instance.sheet.sections.push(Section {
        name: "User".into(),
        rows: vec![Row {
            name: Some("Named".into()),
            cells: vec![Cell { name: "Prompt".into(), value: Some("keep".into()), ..Default::default() }],
            ..Default::default()
        }],
        ..Default::default()
    });
    d.pages[0].shapes.push(instance);
    let a = import(&write(&d).unwrap().bytes).unwrap();
    let sections = &a.pages[0].shapes[0].sheet.sections;
    let connections = sections.iter().find(|s| s.name == "Connections").unwrap();
    assert_eq!(connections.rows.len(), 2);
    for (i, row) in connections.rows.iter().enumerate() {
        assert_eq!(row.index, Some(i as u32));
        assert_eq!(row.cells[0].value.as_deref(), Some(((i + 1) * 2).to_string().as_str()));
    }
    let user = sections.iter().find(|s| s.name == "User").unwrap();
    assert_eq!(user.rows.len(), 1);
    assert_eq!(user.rows[0].cells.len(), 2);
}

#[test]
fn inherited_string_expansion_is_rejected_at_the_sheet_byte_bound() {
    let mut d = skeleton::document(false);
    d.masters[0].shapes[0].sheet.sections = vec![Section {
        name: "User".into(),
        rows: (0..300)
            .map(|i| Row {
                name: Some(format!("Large{i}")),
                cells: vec![Cell {
                    name: "Value".into(),
                    value: Some(String::new()),
                    formula: Some("IF(Width>1,User.Shared,\"\")".into()),
                    ..Default::default()
                }],
                ..Default::default()
            })
            .collect(),
        ..Default::default()
    }];
    let mut s = diagramcraft_model::Shape { id: 1, master: Some(2), ..Default::default() };
    s.sheet.cells.push(Cell { name: "Width".into(), value: Some("2".into()), ..Default::default() });
    d.pages[0].shapes.push(s);
    d.masters[0].shapes[0].sheet.sections[0].rows.push(Row {
        name: Some("Shared".into()),
        cells: vec![Cell { name: "Value".into(), value: Some("a".repeat(65536)), units: Some("STR".into()), ..Default::default() }],
        ..Default::default()
    });
    let error = write(&d).expect_err("expanded instance must be rejected");
    assert!(error.to_string().contains("instance sheet byte limit"), "{error}");
}

#[test]
fn equivalent_inherited_colors_and_font_ordinals_are_not_materialized() {
    let mut d = skeleton::document(false);
    d.masters[0].shapes[0].sheet.cells.push(Cell {
        name: "LineColor".into(),
        value: Some("#cc0000".into()),
        formula: Some("RGB(204,0,0)".into()),
        ..Default::default()
    });
    d.masters[0].shapes[0].sheet.sections.push(Section {
        name: "Character".into(),
        rows: vec![Row {
            index: Some(0),
            cells: vec![Cell { name: "Font".into(), value: Some("0".into()), formula: Some("FONT(\"Calibri\")".into()), ..Default::default() }],
            ..Default::default()
        }],
        ..Default::default()
    });
    d.pages[0].shapes.push(diagramcraft_model::Shape { id: 1, master: Some(2), ..Default::default() });
    let a = import(&write(&d).unwrap().bytes).unwrap();
    let sheet = &a.pages[0].shapes[0].sheet;
    assert!(sheet.cell("LineColor").is_none());
    assert!(!sheet.sections.iter().any(|s| s.name == "Character"));
}

#[test]
fn inherited_text_caches_compare_as_text_and_named_fonts_as_faces() {
    let mut d = skeleton::document(false);
    d.masters[0].shapes[0].sheet.sections = vec![
        Section {
            name: "User".into(),
            rows: [("NumericText", "01", "1"), ("HexText", "#abc", "#ABC")]
                .into_iter()
                .map(|(name, before, after)| Row {
                    name: Some(name.into()),
                    cells: vec![Cell {
                        name: "Value".into(),
                        value: Some(before.into()),
                        units: Some("STR".into()),
                        formula: Some(format!("IF(Width>1,\"{after}\",\"{before}\")")),
                        ..Default::default()
                    }],
                    ..Default::default()
                })
                .collect(),
            ..Default::default()
        },
        Section {
            name: "Character".into(),
            rows: vec![Row {
                index: Some(0),
                cells: vec![Cell {
                    name: "Font".into(),
                    value: Some("Calibri".into()),
                    formula: Some("FONT(\"Calibri\")".into()),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        },
    ];
    let mut s = diagramcraft_model::Shape { id: 1, master: Some(2), ..Default::default() };
    s.sheet.cells.push(Cell { name: "Width".into(), value: Some("2".into()), ..Default::default() });
    d.pages[0].shapes.push(s);
    let a = import(&write(&d).unwrap().bytes).unwrap();
    let sheet = &a.pages[0].shapes[0].sheet;
    let user = sheet.sections.iter().find(|s| s.name == "User").unwrap();
    for (name, value) in [("NumericText", "1"), ("HexText", "#ABC")] {
        let row = user.rows.iter().find(|r| r.name.as_deref() == Some(name)).unwrap();
        assert_eq!(row.cells[0].value.as_deref(), Some(value));
        assert_eq!(row.cells[0].formula.as_deref(), Some("Inh"));
    }
    assert!(!sheet.sections.iter().any(|s| s.name == "Character"));
}

#[test]
fn authored_master_content_rejects_reserved_ids() {
    for id in 0..5 {
        let mut d = skeleton::document(false);
        d.masters[0].shapes[0].id = id;
        assert!(write(&d).is_err(), "reserved master content ID {id}");
    }
    let d = skeleton::document(false);
    assert_eq!(import(&write(&d).unwrap().bytes).unwrap().masters[0].shapes[0].id, 5);
}
#[test]
fn original_visio_master_content_id_four_is_preserved() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    if !root.exists() {
        eprintln!("skip: private corpus/ is absent");
        return;
    }
    let d = diagramcraft_vsdx::read(root.join("ms-shipped/1033/PHONE_CONN_U.vstx")).unwrap();
    let a = import(&write(&d).unwrap().bytes).unwrap();
    let count = |d: &diagramcraft_model::Document| {
        let mut stack: Vec<_> = d.masters.iter().flat_map(|m| &m.shapes).collect();
        let mut count = 0;
        while let Some(s) = stack.pop() {
            count += usize::from(s.id == 4);
            stack.extend(&s.shapes);
        }
        count
    };
    assert!(count(&d) > 0);
    assert_eq!(count(&d), count(&a));
}

#[test]
fn authored_and_edited_literal_cache_types_are_serialized_by_type() {
    let mut d = skeleton::document(false);
    let mut shape = skeleton::rectangle(1, 0., 0., 1., 1., "").unwrap();
    for (name, value) in [("TextBkgnd", "2"), ("LineColor", "0"), ("Comment", "")] {
        shape.sheet.cells.push(Cell { name: name.into(), value: Some(value.into()), ..Default::default() });
    }
    shape.sheet.sections.push(Section {
        name: "Character".into(),
        rows: vec![Row {
            index: Some(0),
            cells: vec![
                Cell { name: "LangID".into(), value: Some("1033".into()), ..Default::default() },
                Cell { name: "Font".into(), value: Some("0".into()), ..Default::default() },
            ],
            ..Default::default()
        }],
        ..Default::default()
    });
    d.pages[0].shapes.push(shape);
    let mut a = import(&write(&d).unwrap().bytes).unwrap();
    let sheet = &mut a.pages[0].shapes[0].sheet;
    assert_eq!(sheet.cell("TextBkgnd").unwrap().value.as_deref(), Some("#ffffff"));
    assert_eq!(sheet.cell("LineColor").unwrap().value.as_deref(), Some("#000000"));
    assert_eq!(sheet.cell("Comment").unwrap().value.as_deref(), Some(""));
    let row = &mut sheet.sections.iter_mut().find(|s| s.name == "Character").unwrap().rows[0];
    assert_eq!(row.cells[0].value.as_deref(), Some("en-US"));
    assert_eq!(row.cells[1].value.as_deref(), Some("0"));
    row.cells[0].value = Some("2057".into());
    row.cells[1].value = Some("Calibri".into());
    sheet.cells.iter_mut().find(|c| c.name == "LineColor").unwrap().value = Some("1".into());
    let b = import(&write(&a).unwrap().bytes).unwrap();
    let sheet = &b.pages[0].shapes[0].sheet;
    assert_eq!(sheet.cell("LineColor").unwrap().value.as_deref(), Some("#ffffff"));
    let row = &sheet.sections.iter().find(|s| s.name == "Character").unwrap().rows[0];
    assert_eq!(row.cells[0].value.as_deref(), Some("en-GB"));
    assert_eq!(row.cells[1].value.as_deref(), Some("Calibri"));
}

#[test]
fn extension_xml_cannot_authorize_a_reserved_master_shape_id() {
    let mut d = import(&write(&skeleton::document(false)).unwrap().bytes).unwrap();
    let part = d.parts.get_mut("visio/masters/master1.xml").unwrap();
    let mut unrelated = XmlElement { name: "Extension".into(), namespace: "urn:diagramcraft:test".into(), ..Default::default() };
    unrelated.children.push(XmlNode::Element(XmlElement {
        name: "Shape".into(),
        namespace: skeleton::VISIO_NS.into(),
        attributes: std::collections::BTreeMap::from([("ID".into(), "4".into())]),
        ..Default::default()
    }));
    part.xml.as_mut().unwrap().children.push(XmlNode::Element(unrelated));
    d.masters[0].shapes[0].id = 4;
    assert!(write(&d).expect_err("extension is not source master content").to_string().contains("must start at 5"));
}

#[test]
fn edited_source_tree_cannot_authorize_reserved_master_id() {
    let mut d = import(&write(&skeleton::document(false)).unwrap().bytes).unwrap();
    let part = d.parts.get_mut("visio/masters/master1.xml").unwrap();
    fn alter(e: &mut XmlElement) {
        if e.is_visio("Shape") {
            e.attributes.insert("ID".into(), "4".into());
        }
        for child in &mut e.children {
            if let XmlNode::Element(e) = child {
                alter(e);
            }
        }
    }
    alter(part.xml.as_mut().unwrap());
    d.masters[0].shapes[0].id = 4;
    assert!(write(&d).is_err());
}

#[test]
fn background_page_link_and_last_page_removal_are_complete() {
    let mut d = skeleton::document(true);
    let mut back = skeleton::page(1, "Background", true);
    back.attributes.insert("Background".into(), "1".into());
    d.pages[0].attributes.insert("BackPage".into(), "1".into());
    d.pages.push(back);
    let a = import(&write(&d).unwrap().bytes).unwrap();
    assert!(
        a.parts["visio/pages/page1.xml"]
            .relationships
            .iter()
            .any(|r| r.type_uri == "http://schemas.microsoft.com/visio/2010/relationships/page" && r.target == "visio/pages/page2.xml")
    );
    let mut d = import(&write(&skeleton::document(false)).unwrap().bytes).unwrap();
    assert!(d.parts.contains_key("visio/pages/page1.xml"));
    d.pages.clear();
    let a = import(&write(&d).unwrap().bytes).unwrap();
    assert!(a.pages.is_empty());
    assert!(!a.parts.contains_key("visio/pages/page1.xml"));
    assert!(!a.parts.contains_key("visio/pages/_rels/pages.xml.rels"));
    assert!(!a.parts["visio/pages/pages.xml"].relationships.iter().any(|r| r.type_uri.ends_with("/page")));
}
#[test]
fn connect_to_missing_page_or_master_shape_is_rejected() {
    let mut d = skeleton::document(false);
    d.pages[0].shapes.push(skeleton::rectangle(1, 0., 0., 1., 1., "").unwrap());
    d.pages[0].connects.push(diagramcraft_model::Connect { from_sheet: 1, to_sheet: 99, ..Default::default() });
    assert!(write(&d).expect_err("missing page target").to_string().contains("Connect references missing sheet"));
    d.pages[0].connects.clear();
    d.masters[0].connects.push(diagramcraft_model::Connect { from_sheet: 5, to_sheet: 99, ..Default::default() });
    assert!(write(&d).expect_err("missing master target").to_string().contains("Connect references missing sheet"));
}

#[test]
fn connect_to_enclosing_page_sheet_is_valid_but_deleted_shape_is_not() {
    let mut d = skeleton::document(false);
    d.pages[0].shapes.push(skeleton::rectangle(1, 0., 0., 1., 1., "").unwrap());
    d.pages[0].connects.push(diagramcraft_model::Connect {
        from_sheet: 1,
        to_sheet: 0,
        from_cell: Some("Controls.Row_1".into()),
        to_cell: Some("User.VPX".into()),
        ..Default::default()
    });
    let a = import(&write(&d).unwrap().bytes).unwrap();
    assert_eq!(a.pages[0].connects[0].to_sheet, 0);
    d.pages[0].shapes[0].sheet.attributes.insert("Del".into(), "1".into());
    assert!(write(&d).is_err());
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    if !root.exists() {
        eprintln!("skip: private corpus/ is absent");
        return;
    }
    let d = diagramcraft_vsdx::read(root.join("ms-shipped/1033/BLOCKP_M.vssx")).unwrap();
    let a = import(&write(&d).unwrap().bytes).unwrap();
    let count = |d: &diagramcraft_model::Document| d.masters.iter().flat_map(|m| &m.connects).filter(|c| c.to_sheet == 0).count();
    assert_eq!(count(&d), 18);
    assert_eq!(count(&a), 18);
}

#[test]
fn background_cycles_reject_and_removed_link_prunes_relationship_part() {
    let mut d = skeleton::document(false);
    d.pages.push(skeleton::page(1, "Back", false));
    d.pages[0].attributes.insert("BackPage".into(), "1".into());
    d.pages[1].attributes.insert("BackPage".into(), "0".into());
    assert!(write(&d).unwrap_err().to_string().contains("background page cycle"));
    d.pages[1].attributes.remove("BackPage");
    let mut d = import(&write(&d).unwrap().bytes).unwrap();
    assert!(d.parts.contains_key("visio/pages/_rels/page1.xml.rels"));
    d.pages[0].attributes.remove("BackPage");
    let a = import(&write(&d).unwrap().bytes).unwrap();
    assert!(!a.parts.contains_key("visio/pages/_rels/page1.xml.rels"));
}

#[test]
fn unchanged_metadata_keeps_original_bytes_and_edits_are_serialized() {
    let d = import(&write(&skeleton::document(false)).unwrap().bytes).unwrap();
    let a = import(&write(&d).unwrap().bytes).unwrap();
    for name in ["docProps/core.xml", "docProps/app.xml", "docProps/custom.xml", "visio/windows.xml"] {
        assert_eq!(d.parts[name].bytes, a.parts[name].bytes, "{name}");
    }
    let mut edited = d.clone();
    let xml = edited.parts.get_mut("docProps/custom.xml").unwrap().xml.as_mut().unwrap();
    xml.attributes.insert("edited".into(), "true".into());
    let a = import(&write(&edited).unwrap().bytes).unwrap();
    assert_ne!(d.parts["docProps/custom.xml"].bytes, a.parts["docProps/custom.xml"].bytes);
    assert_eq!(a.parts["docProps/custom.xml"].xml.as_ref().unwrap().attr("edited"), Some("true"));
}

#[test]
fn probe_resave_has_no_literal_differences_in_acceptance_reporter() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    if !root.join("corpus").exists() {
        eprintln!("skip: private corpus/ is absent");
        return;
    }
    let source = root.join("corpus/generated/basic/0001-probe.vsdx");
    let d = diagramcraft_vsdx::read(&source).unwrap();
    let directory = root.join("target/writer-acceptance-tests");
    std::fs::create_dir_all(&directory).unwrap();
    let output = directory.join(format!("probe-{}.vsdx", std::process::id()));
    std::fs::write(&output, write(&d).unwrap().bytes).unwrap();
    let result=std::process::Command::new("python3").current_dir(&root).arg("-c").arg(
        "import sys;sys.path.insert(0,'tools/oracle');import accept_report as r;_,a=r.package_cells(sys.argv[1]);_,b=r.package_cells(sys.argv[2]);all_d=r.cell_diffs(a,b,set());kinds={str(k):v.get('kind') for k,v in (a|b).items()};d=[x for x in all_d if kinds.get(x[0])!='opaque'];print('probe literal/structural differences:',len(d),d[:20],'; separately measured byte-hash differences:',len(all_d)-len(d));sys.exit(bool(d))"
    ).arg(&source).arg(&output).output().unwrap();
    let _ = std::fs::remove_file(output);
    assert!(result.status.success(), "{}{}", String::from_utf8_lossy(&result.stdout), String::from_utf8_lossy(&result.stderr));
}

#[test]
fn unchanged_opc_wrappers_keep_original_bytes() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    if !root.exists() {
        eprintln!("skip: private corpus/ is absent");
        return;
    }
    for name in ["generated/basic/0001-probe.vsdx", "generated/arrow/begin-00.vsdx"] {
        let before = diagramcraft_vsdx::read(root.join(name)).unwrap();
        let after = import(&write(&before).unwrap().bytes).unwrap();
        let mut count = 0;
        for (name, part) in &before.parts {
            if name == "[Content_Types].xml" || name.ends_with(".rels") {
                assert_eq!(part.bytes, after.parts[name].bytes, "{name}");
                count += 1;
            }
        }
        assert!(count >= 4);
    }
}

/// Authored inherited strings use empty defaults, not COM's numeric ResultIU zero.
/// Witness: scratch__dynamic-glue saved Comment and ShapeKeywords caches.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
#[test]
fn authored_inherited_empty_strings_do_not_serialize_com_zero() {
    let d = skeleton::document(false);
    let after = import(&write(&d).unwrap().bytes).unwrap();
    for name in ["Comment", "ShapeKeywords"] {
        let c = after.masters[0].shapes[0].sheet.cell(name).unwrap();
        assert_eq!(c.formula.as_deref(), Some("Inh"));
        assert_eq!(c.value.as_deref(), Some(""), "{name}");
    }
    let mut d = skeleton::document(false);
    for c in &mut d.masters[0].shapes[0].sheet.cells {
        if matches!(c.name.as_str(), "Comment" | "ShapeKeywords") {
            c.formula = None;
            c.value = Some("0".into());
        }
    }
    let after = import(&write(&d).unwrap().bytes).unwrap();
    for name in ["Comment", "ShapeKeywords"] {
        assert_eq!(after.masters[0].shapes[0].sheet.cell(name).unwrap().value.as_deref(), Some("0"));
    }
}
