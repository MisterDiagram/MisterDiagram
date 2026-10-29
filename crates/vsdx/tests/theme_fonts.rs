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
    document.parts.insert(
        "visio/theme/theme1.xml".into(),
        Part { xml: Some(root.into()), content_type: Some("application/vnd.openxmlformats-officedocument.theme+xml".into()), ..Default::default() },
    );
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
            ("QuickStyleType", "2"),
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

/// A formula cache can name a requested theme face missing from FaceNames.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/font-cell-character-section
#[test]
fn theme_only_font_formula_cache_roundtrips_by_name() {
    use misterdiagram_model::{Cell, Row, Section};
    let mut document = document("Contributor Theme Face");
    let mut shape = skeleton::rectangle(1, 1., 1., 2., 1., "Label").unwrap();
    shape.sheet.sections.push(Section {
        name: "Character".into(),
        rows: vec![Row {
            index: Some(0),
            cells: vec![Cell {
                name: "Font".into(),
                formula: Some("FONT(\"Contributor Theme Face\")".into()),
                value: Some("0".into()),
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    });
    document.pages[0].shapes.push(shape);
    let written = misterdiagram_vsdx::write_with_hosts(&document, None, Some(&misterdiagram_text::Host)).unwrap();
    assert_eq!(written.warning_count, 0, "{:?}", written.warnings);
    let reopened = misterdiagram_vsdx::import(&written.bytes).unwrap();
    assert_eq!(reopened.face_names, document.face_names);
    let cell = &reopened.pages[0].shapes[0].sheet.sections.iter().find(|s| s.name == "Character").unwrap().rows[0].cells[0];
    assert_eq!(cell.value.as_deref(), Some("Contributor Theme Face"));
    assert_eq!(cell.formula.as_deref(), Some("FONT(\"Contributor Theme Face\")"));
    let mut evaluator = Evaluator::new(&reopened).unwrap();
    let value = evaluator.evaluate(misterdiagram_vsdx::evaluate::Scope::PageShape { page: 0, shape: 1 }, "Char.Font");
    assert_eq!(evaluator.font_name(value.numeric().unwrap() as u32), Some("Contributor Theme Face"));
}

#[test]
fn unmapped_successful_font_id_keeps_its_recorded_family() {
    use misterdiagram_model::{Cell, Row, Section};
    use misterdiagram_vsdx::evaluate::Scope;
    for (formula, family, expected) in [
        ("21", "Georgia", "Georgia"),
        ("4", "Georgia", "Calibri"),
        ("REF()", "Georgia", "Calibri"),
        ("\"Different family\"", "Georgia", "Calibri"),
        ("21", "65535", "Calibri"),
        ("21", "Themed", "Calibri"),
        ("21", "Inh", "Calibri"),
        ("21", &"x".repeat(4097), "Calibri"),
    ] {
        let mut doc = skeleton::document(false);
        let mut shape = skeleton::rectangle(7, 1., 1., 2., 1., "Original text").unwrap();
        shape.sheet.sections.retain(|section| section.name != "Character");
        shape.sheet.sections.push(Section {
            name: "Character".into(),
            rows: vec![Row {
                index: Some(0),
                cells: vec![Cell { name: "Font".into(), formula: Some(formula.into()), value: Some(family.into()), ..Default::default() }],
                ..Default::default()
            }],
            ..Default::default()
        });
        doc.pages[0].shapes.push(shape);
        let scope = Scope::PageShape { page: 0, shape: 7 };
        let mut e = Evaluator::new(&doc).unwrap();
        let before = e.evaluate(scope, "Char.Font");
        let body = e.text_body(scope).unwrap();
        assert_eq!(body.paragraphs[0].runs[0].props.font.as_deref(), Some(expected));
        assert_eq!(e.evaluate(scope, "Char.Font"), before);
    }
}

#[test]
fn erroneous_font_formula_retains_the_documents_mapped_default() {
    use misterdiagram_model::{Cell, Row, Section};
    use misterdiagram_vsdx::evaluate::Scope;
    let mut doc = skeleton::document(false);
    doc.face_names.clear();
    doc.face_names.insert(0, "Consolas".into());
    let mut shape = skeleton::rectangle(7, 1., 1., 2., 1., "Original label").unwrap();
    shape.sheet.sections.retain(|section| section.name != "Character");
    shape.sheet.sections.push(Section {
        name: "Character".into(),
        rows: vec![Row {
            index: Some(0),
            cells: vec![Cell { name: "Font".into(), formula: Some("REF()".into()), value: Some("Contributor Stale".into()), ..Default::default() }],
            ..Default::default()
        }],
        ..Default::default()
    });
    doc.pages[0].shapes.push(shape);
    let mut e = Evaluator::new(&doc).unwrap();
    let body = e.text_body(Scope::PageShape { page: 0, shape: 7 }).unwrap();
    assert_eq!(body.paragraphs[0].runs[0].props.font.as_deref(), Some("Consolas"));
}

/// Visio 2021 draws a font it does not have in Calibri, whatever the styles say; installed fonts
/// keep their names (MisterDiagram#87, private witness field/fonts-2026-10-29). Windows cuts face
/// names to 31 characters.
#[test]
fn a_font_visio_does_not_have_is_drawn_in_calibri() {
    use misterdiagram_model::{Cell, Row, Section};
    use misterdiagram_vsdx::evaluate::Scope;
    for (family, expected) in [
        ("Helvetica", "Calibri"),
        ("Roboto", "Calibri"),
        ("NoSuchFont Zq", "Calibri"),
        ("Arial", "Arial"),
        ("times new roman", "times new roman"),
        ("Bahnschrift SemiBold SemiCondensed", "Bahnschrift SemiBold SemiCondensed"),
    ] {
        let mut doc = skeleton::document(false);
        let mut shape = skeleton::rectangle(7, 1., 1., 2., 1., "Label").unwrap();
        shape.sheet.sections.retain(|section| section.name != "Character");
        shape.sheet.sections.push(Section {
            name: "Character".into(),
            rows: vec![Row {
                index: Some(0),
                cells: vec![Cell { name: "Font".into(), formula: Some("21".into()), value: Some(family.into()), ..Default::default() }],
                ..Default::default()
            }],
            ..Default::default()
        });
        doc.pages[0].shapes.push(shape);
        let mut e = Evaluator::new(&doc).unwrap();
        let body = e.text_body(Scope::PageShape { page: 0, shape: 7 }).unwrap();
        assert_eq!(body.paragraphs[0].runs[0].props.font.as_deref(), Some(expected), "{family}");
    }
}
