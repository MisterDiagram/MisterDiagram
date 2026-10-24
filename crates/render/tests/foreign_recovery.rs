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
