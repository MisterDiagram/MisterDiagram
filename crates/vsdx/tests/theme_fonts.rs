use misterdiagram_model::{Document, Part, XmlElement, XmlNode, skeleton};
use misterdiagram_vsdx::evaluate::Evaluator;

fn element(name: &str, children: Vec<XmlElement>) -> XmlElement {
    XmlElement { name: name.into(), children: children.into_iter().map(XmlNode::Element).collect(), ..Default::default() }
}
fn document(face: &str) -> Document {
    let mut latin = element("latin", vec![]);
    latin.attributes.insert("typeface".into(), face.into());
    let root = element("theme", vec![element("fontScheme", vec![element("minorFont", vec![latin])])]);
    let mut document = skeleton::document(false);
    document.parts.insert("visio/theme/theme1.xml".into(), Part { xml: Some(root.into()), ..Default::default() });
    document
}

/// Requested theme faces survive a capture host's absent-font substitution.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.font.id
#[test]
fn theme_face_is_registered_without_changing_saved_font_ids() {
    let document = document("Contributor Theme Face");
    let evaluator = Evaluator::new(&document).unwrap();
    assert_eq!(evaluator.font_name(4), Some("Calibri"));
    assert!((5..=65535).any(|id| evaluator.font_name(id) == Some("Contributor Theme Face")));
    assert!(!document.face_names.values().any(|face| face == "Contributor Theme Face"));
    let duplicate = self::document("Calibri");
    let evaluator = Evaluator::new(&duplicate).unwrap();
    assert_eq!(evaluator.font_name(4), Some("Calibri"));
    assert_eq!((0..=65535).filter(|id| evaluator.font_name(*id) == Some("Calibri")).count(), 1);
}

#[test]
fn hostile_theme_font_metadata_is_bounded() {
    assert!(Evaluator::new(&document(&"A".repeat(4097))).is_err());
    let mut root = element("theme", vec![]);
    for _ in 0..130 {
        root = element("extension", vec![root]);
    }
    let mut document = skeleton::document(false);
    document.parts.insert("visio/theme/theme1.xml".into(), Part { xml: Some(root.into()), ..Default::default() });
    assert!(Evaluator::new(&document).is_err());
}

/// The resolver and evaluator accept both retained and bytes-only theme parts.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/font-cell-character-section
#[test]
fn raw_theme_faces_resolve_with_colliding_saved_ids() {
    use misterdiagram_model::Cell;
    use misterdiagram_vsdx::evaluate::Scope;
    let xml = b"<theme><clrScheme><dk1><srgbClr val='14283C'/></dk1><schemeID schemeEnum='123'/></clrScheme><fontScheme><majorFont><latin typeface='CONTRIBUTOR EXISTING'/></majorFont><minorFont><latin typeface='Contributor Requested'/></minorFont><schemeID schemeEnum='123'/></fontScheme><fontStyles><fontProps/></fontStyles></theme>";
    for retained in [false, true] {
        let mut document = skeleton::document(false);
        document.face_names.insert(5, "Contributor Existing".into());
        document.face_names.insert(6, "Contributor Other".into());
        let part =
            Part { bytes: xml.to_vec().into(), xml: retained.then(|| misterdiagram_vsdx::parse_xml(xml).unwrap().into()), ..Default::default() };
        document.parts.insert("visio/theme/theme1.xml".into(), part);
        let mut shape = skeleton::rectangle(1, 1., 1., 2., 1., "").unwrap();
        for (name, value) in [
            ("ColorSchemeIndex", "123"),
            ("FontSchemeIndex", "123"),
            ("QuickStyleFontColor", "0"),
            ("QuickStyleFontMatrix", "1"),
            ("QuickStyleType", "1"),
        ] {
            shape.sheet.cells.retain(|cell| cell.name != name);
            shape.sheet.cells.push(Cell { name: name.into(), value: Some(value.into()), ..Default::default() });
        }
        document.pages[0].shapes.push(shape);
        let themes = misterdiagram_theme::Themes::from_document(&document).unwrap();
        let mut evaluator = Evaluator::new(&document).unwrap().with_theme_host(&themes);
        let value = evaluator.evaluate(Scope::PageShape { page: 0, shape: 1 }, "Char.Font");
        assert_eq!(value.numeric(), Ok(7.));
        assert_eq!(evaluator.font_name(4), Some("Calibri"));
        assert_eq!(evaluator.font_name(5), Some("Contributor Existing"));
        assert_eq!(evaluator.font_name(6), Some("Contributor Other"));
        assert_eq!(evaluator.font_name(7), Some("Contributor Requested"));
        assert_eq!(evaluator.font_name(8), None);
    }
    let mut hostile = skeleton::document(false);
    hostile.parts.insert("visio/theme/theme1.xml".into(), Part { bytes: b"<theme><broken>".to_vec().into(), ..Default::default() });
    assert!(Evaluator::new(&hostile).is_err());
}
