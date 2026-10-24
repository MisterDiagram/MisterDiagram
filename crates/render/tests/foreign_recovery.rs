//! Original garbage parts and original generated pixels exercise foreign-object recovery.
use misterdiagram_model::{Cell, Document, Part, Relationship, XmlElement, XmlNode, skeleton};
use misterdiagram_render::{
    raster,
    scene::{self, Op, Options},
};
use std::io::Cursor;
fn document(icon: bool, picture: bool) -> Document {
    let mut doc = skeleton::document(false);
    let mut shape = skeleton::rectangle(7, 2., 2., 2., 2., "").unwrap();
    shape.name_u = "Original object".into();
    shape.shape_type = Some("Foreign".into());
    shape.sheet.sections.clear();
    for (name, v) in [("ImgWidth", "2"), ("ImgHeight", "2")] {
        shape.sheet.cells.push(Cell { name: name.into(), value: Some(v.into()), ..Default::default() });
    }
    let ns = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
    shape.foreign_data = Some(XmlElement {
        name: "ForeignData".into(),
        namespace: skeleton::VISIO_NS.into(),
        attributes: [
            ("ForeignType".into(), "Object".into()),
            ("ObjectType".into(), "17".into()),
            ("ObjectWidth".into(), "1.25".into()),
            ("ObjectHeight".into(), "0.75".into()),
            ("ShowAsIcon".into(), if icon { "1" } else { "0" }.into()),
        ]
        .into(),
        children: vec![XmlNode::Element(XmlElement {
            name: "Rel".into(),
            namespace: skeleton::VISIO_NS.into(),
            attributes: [("r:id".into(), "opaque".into())].into(),
            attribute_namespaces: [("r:id".into(), ns.into())].into(),
            ..Default::default()
        })],
        ..Default::default()
    });
    doc.pages[0].shapes = vec![shape].into();
    doc.parts.entry(doc.pages[0].part.clone()).or_default().relationships.push(Relationship {
        id: "opaque".into(),
        type_uri: format!("{ns}/oleObject"),
        target: "visio/embeddings/original.bin".into(),
        external: false,
    });
    let mut object = Part { bytes: vec![1, 3, 5, 7, 9].into(), ..Default::default() };
    if picture {
        object.relationships.push(Relationship {
            id: "preview".into(),
            type_uri: format!("{ns}/image"),
            target: "visio/media/original.png".into(),
            external: false,
        });
        let image = image::RgbaImage::from_pixel(2, 3, image::Rgba([20, 80, 140, 255]));
        let mut buffer = Cursor::new(Vec::new());
        image.write_to(&mut buffer, image::ImageFormat::Png).unwrap();
        doc.parts.insert("visio/media/original.png".into(), Part { bytes: buffer.into_inner().into(), ..Default::default() });
    }
    doc.parts.insert("visio/embeddings/original.bin".into(), object);
    doc
}
#[test]
fn undecodable_foreign_data_uses_only_its_picture_or_sized_placeholder() {
    for icon in [false, true] {
        for picture in [false, true] {
            let doc = document(icon, picture);
            assert!(scene::validate_foreign_parts(&doc, misterdiagram_vsdx::dump::Selection::All).is_err());
            assert!(scene::validate_foreign_parts(&doc, misterdiagram_vsdx::dump::Selection::Document).is_ok());
            let error = scene::page(&doc, 0, Options::default()).unwrap_err();
            assert!(error.contains("shape 7") && error.contains("ObjectType=17"), "{error}");
            let options = Options { cached_fallback: true, ..Default::default() };
            let scene = scene::page(&doc, 0, options).unwrap();
            assert!(
                scene.warnings.iter().any(|w| w.contains("shape 7") && w.contains("Original object") && w.contains("ObjectType=17")),
                "{:?}",
                scene.warnings
            );
            assert_eq!(scene.ops.iter().filter(|op| matches!(op, Op::Image { .. })).count(), usize::from(picture));
            let word = if !picture {
                "placeholder"
            } else if icon {
                "cached icon"
            } else {
                "cached picture"
            };
            assert!(scene.warnings.iter().any(|w| w.contains(word)), "{:?}", scene.warnings);
            if !picture {
                use kurbo::Shape;
                assert!(scene.ops.iter().any(|op| match op {
                    Op::Stroke { path, .. } => {
                        let r = path.bounding_box();
                        (r.width() - 1.25).abs() < 1e-6 && (r.height() - 0.75).abs() < 1e-6
                    }
                    _ => false,
                }));
            }
            assert!(raster::render(&scene, options).is_ok());
        }
    }
}
#[test]
fn malformed_extents_and_external_fallback_do_not_escape_the_drawing() {
    let mut doc = document(true, true);
    let object = doc.parts.get_mut("visio/embeddings/original.bin").unwrap();
    object.relationships[0].external = true;
    let foreign = doc.pages[0].shapes[0].foreign_data.as_mut().unwrap();
    foreign.attributes.insert("ObjectWidth".into(), "NaN".into());
    foreign.attributes.insert("ObjectHeight".into(), "1e999".into());
    let scene = scene::page(&doc, 0, Options { cached_fallback: true, ..Default::default() }).unwrap();
    assert!(!scene.ops.iter().any(|op| matches!(op, Op::Image { .. })));
    assert!(scene.warnings.iter().any(|w| w.contains("placeholder")));
}

#[test]
fn corrupt_first_preview_does_not_hide_a_later_valid_picture() {
    let mut doc = document(false, true);
    let object = doc.parts.get_mut("visio/embeddings/original.bin").unwrap();
    let mut broken = object.relationships[0].clone();
    broken.id = "broken-first".into();
    broken.target = "visio/media/missing.png".into();
    object.relationships.insert(0, broken);
    let result = scene::page(&doc, 0, Options { cached_fallback: true, ..Default::default() }).unwrap();
    assert!(result.ops.iter().any(|op| matches!(op, Op::Image { .. })));
    assert!(result.warnings.iter().any(|w| w.contains("cached picture")));
}
#[test]
fn hostile_preview_fanout_stops_with_a_named_placeholder() {
    let mut doc = document(false, true);
    let object = doc.parts.get_mut("visio/embeddings/original.bin").unwrap();
    let mut broken = object.relationships[0].clone();
    broken.target = "visio/media/missing.png".into();
    for i in 0..33 {
        broken.id = format!("broken-{i}");
        object.relationships.insert(0, broken.clone());
    }
    let result = scene::page(&doc, 0, Options { cached_fallback: true, ..Default::default() }).unwrap();
    assert!(!result.ops.iter().any(|op| matches!(op, Op::Image { .. })));
    assert!(result.warnings.iter().any(|w| w.contains("shape 7") && w.contains("placeholder") && w.contains("preview recovery limit")));
}

#[test]
fn empty_metafile_preview_uses_the_next_picture_or_placeholder() {
    let mut emf = vec![0u8; 116];
    for (offset, value) in [
        (0, 1u32),
        (4, 88),
        (16, 96),
        (20, 96),
        (32, 2540),
        (36, 2540),
        (40, 0x464d4520),
        (44, 0x10000),
        (48, 116),
        (52, 3),
        (56, 1),
        (72, 96),
        (76, 96),
        (80, 25),
        (84, 25),
        (88, 9999),
        (92, 8),
        (96, 14),
        (100, 20),
    ] {
        emf[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    for later_picture in [false, true] {
        let mut doc = document(false, later_picture);
        doc.parts.insert("visio/media/empty.emf".into(), Part { bytes: emf.clone().into(), ..Default::default() });
        doc.parts.get_mut("visio/embeddings/original.bin").unwrap().relationships.insert(
            0,
            Relationship {
                id: "empty".into(),
                type_uri: "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image".into(),
                target: "visio/media/empty.emf".into(),
                external: false,
            },
        );
        let result = scene::page(&doc, 0, Options { cached_fallback: true, ..Default::default() }).unwrap();
        assert_eq!(result.ops.iter().any(|op| matches!(op, Op::Image { .. })), later_picture);
        assert!(result.warnings.iter().any(|w| w.contains(if later_picture { "cached picture" } else { "placeholder" })), "{:?}", result.warnings);
    }
}

#[test]
fn foreign_recovery_preserves_primary_metafile_resource_refusals() {
    fn record(bytes: &mut Vec<u8>, kind: u32, values: &[u32]) {
        bytes.extend(kind.to_le_bytes());
        bytes.extend((8 + values.len() as u32 * 4).to_le_bytes());
        for value in values {
            bytes.extend(value.to_le_bytes());
        }
    }
    // Original dotted zigzag: its expansion exceeds the operation cap
    // without allocating the expanded path. This is a resource refusal, not an
    // unknown embedded-object format.
    let mut emf = vec![0u8; 88];
    record(&mut emf, 95, &[1, 0, 0, 0, 0, 2, 0, 0, 0xff, 0, 0]);
    record(&mut emf, 37, &[1]);
    let mut points = vec![0, 0, 96, 96, 40];
    for i in 0..40 {
        points.extend([if i % 2 == 0 { 0 } else { 100_000 }, i * 10]);
    }
    record(&mut emf, 4, &points);
    record(&mut emf, 14, &[0, 0, 20]);
    let size = emf.len() as u32;
    for (offset, value) in [
        (0, 1u32),
        (4, 88),
        (16, 96),
        (20, 96),
        (32, 2540),
        (36, 2540),
        (40, 0x464d4520),
        (44, 0x10000),
        (48, size),
        (52, 5),
        (56, 2),
        (72, 96),
        (76, 96),
        (80, 25),
        (84, 25),
    ] {
        emf[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    for kind in ["EnhMetaFile", "Object"] {
        let mut doc = document(false, true);
        doc.pages[0].shapes[0].foreign_data.as_mut().unwrap().attributes.insert("ForeignType".into(), kind.into());
        doc.parts.get_mut("visio/embeddings/original.bin").unwrap().bytes = emf.clone().into();
        let strict = scene::page(&doc, 0, Options::default()).unwrap_err();
        assert!(strict.contains("metafile operation limit"), "{strict}");
        let error = scene::page(&doc, 0, Options { cached_fallback: true, ..Default::default() }).unwrap_err();
        assert!(error.contains("shape 7") && error.contains("metafile operation limit"), "{error}");
    }
}

#[test]
fn foreign_recovery_preserves_primary_image_resource_refusals() {
    // Original BMP header declares a width beyond the renderer's allocation cap.
    let mut bmp = vec![0u8; 54];
    bmp[..2].copy_from_slice(b"BM");
    for (offset, value) in [(2, 54u32), (10, 54), (14, 40), (18, 20_000), (22, 1)] {
        bmp[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    bmp[26..28].copy_from_slice(&1u16.to_le_bytes());
    bmp[28..30].copy_from_slice(&24u16.to_le_bytes());
    assert!(raster::decode(&bmp).unwrap_err().contains("image pixel limit"));
    let mut doc = document(false, true);
    doc.parts.get_mut("visio/embeddings/original.bin").unwrap().bytes = bmp.into();
    let error = scene::page(&doc, 0, Options { cached_fallback: true, ..Default::default() }).unwrap_err();
    assert!(error.contains("shape 7") && error.contains("image pixel limit"), "{error}");
}
