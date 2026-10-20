//! Contributor-original reproducers for the three named M5.12 gaps.
//! No vendor package data or artwork is included.
use misterdiagram_geom::{BezPath, Rect};
use misterdiagram_model::{Cell, Document, Part, Relationship, XmlElement, XmlNode, skeleton};
use misterdiagram_render::scene::{self, Op, Paint, Scene};

fn master(text: &str) -> Document {
    let mut doc = skeleton::document(false);
    doc.masters[0].shapes = vec![skeleton::rectangle(5, 0., 0., 2., 1., text).unwrap()].into();
    doc
}

/// REF is an error-producing function, not evidence for substituting a cached angle.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/ref-function
#[test]
fn unresolved_ref_in_text_angle_is_a_named_render_gap() {
    let mut doc = master("Original text");
    doc.masters[0].shapes[0].sheet.cells.push(Cell {
        name: "TxtAngle".into(),
        value: Some("0".into()),
        formula: Some("IF(REF(),3.14159,0)".into()),
        ..Default::default()
    });
    let bytes = misterdiagram_vsdx::write_as(&doc, misterdiagram_vsdx::SaveKind::Stencil).unwrap().bytes;
    let reopened = misterdiagram_vsdx::import(&bytes).unwrap();
    let error = scene::master(&reopened, 0, scene::Options::default()).unwrap_err();
    assert!(error.contains("#NAME? REF"), "{error}");
}

/// Non-finite coordinates must never reach a rasterizer, even on a small canvas.
/// This models the failing scene condition, not the unresolved path-generation cause.
#[test]
fn nonfinite_path_is_rejected_without_raising_the_coordinate_cap() {
    for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 1_000_001.] {
        let mut path = BezPath::new();
        path.move_to((0., 0.));
        path.curve_to((invalid, 0.2), (0.5, 0.4), (1., 1.));
        let scene = Scene {
            warnings: vec![],
            ops: vec![Op::Path { path, paint: Paint::Solid(misterdiagram_color::Rgba::BLACK) }],
            bounds: Rect::new(0., 0., 2., 1.),
            page_size: (2., 1.),
        };
        assert_eq!(scene.canvas(scene::Options::default()).unwrap_err(), "scene coordinate limit");
    }
}

/// Standard WMF META_HEADER plus an original rectangle and EOF, mislabeled .emf.
/// https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-wmf/ba5458c6-e885-41e6-b5d7-d54ef9e1065f
#[test]
fn standard_wmf_with_emf_extension_is_a_named_media_gap() {
    let mut bytes = Vec::new();
    // All sizes are WORD counts: 9 header + 7 rectangle + 3 EOF = 19.
    for word in [1u16, 9, 0x0300, 19, 0, 0, 7, 0, 0, 7, 0, 0x041b, 20, 30, 0, 0, 3, 0, 0] {
        bytes.extend_from_slice(&word.to_le_bytes());
    }
    let mut doc = master("");
    let shape = &mut doc.masters[0].shapes[0];
    shape.shape_type = Some("Foreign".into());
    shape.sheet.sections.clear();
    let relationship_namespace = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
    shape.foreign_data = Some(XmlElement {
        name: "ForeignData".into(),
        namespace: "http://schemas.microsoft.com/office/visio/2012/main".into(),
        attributes: [("ForeignType".into(), "EnhMetaFile".into())].into(),
        children: vec![XmlNode::Element(XmlElement {
            name: "Rel".into(),
            namespace: "http://schemas.microsoft.com/office/visio/2012/main".into(),
            attributes: [("r:id".into(), "original".into()), ("xmlns:r".into(), relationship_namespace.into())].into(),
            attribute_namespaces: [("r:id".into(), relationship_namespace.into())].into(),
            ..Default::default()
        })],
        ..Default::default()
    });
    let target = "visio/media/original.emf";
    let owner = doc.masters[0].part.clone();
    doc.parts.insert(target.into(), Part { bytes: bytes.into(), content_type: Some("image/x-emf".into()), ..Default::default() });
    doc.parts.entry(owner).or_default().relationships.push(Relationship {
        id: "original".into(),
        type_uri: format!("{relationship_namespace}/image"),
        target: target.into(),
        external: false,
    });
    let bytes = misterdiagram_vsdx::write_as(&doc, misterdiagram_vsdx::SaveKind::Stencil).unwrap().bytes;
    let reopened = misterdiagram_vsdx::import(&bytes).unwrap();
    let error = scene::master(&reopened, 0, scene::Options::default()).unwrap_err();
    assert!(error.contains("image format could not be determined"), "{error}");
}
