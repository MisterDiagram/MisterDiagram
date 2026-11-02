//! An original foreign shape with a deliberately mislabeled part and media type.
use misterdiagram_model::{Cell, Document, Part, Relationship, XmlElement, XmlNode, skeleton};
pub fn document(bytes: Vec<u8>) -> Document {
    let mut doc = skeleton::document(false);
    let mut shape = skeleton::rectangle(7, 2., 3., 4., 2., "").unwrap();
    shape.shape_type = Some("Foreign".into());
    shape.sheet.sections.clear();
    for (name, value) in [("ImgOffsetX", "0.5"), ("ImgOffsetY", "0.25"), ("ImgWidth", "3"), ("ImgHeight", "1.5"), ("Angle", "1.5707963267948966")] {
        shape.sheet.cells.retain(|c| c.name != name);
        shape.sheet.cells.push(Cell { name: name.into(), value: Some(value.into()), ..Default::default() });
    }
    let ns = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
    shape.foreign_data = Some(XmlElement {
        name: "ForeignData".into(),
        namespace: skeleton::VISIO_NS.into(),
        attributes: [
            ("ForeignType".into(), "EnhMetaFile".into()),
            ("MappingMode".into(), "8".into()),
            ("ExtentX".into(), "5080".into()),
            ("ExtentY".into(), "2540".into()),
        ]
        .into(),
        children: vec![XmlNode::Element(XmlElement {
            name: "Rel".into(),
            namespace: skeleton::VISIO_NS.into(),
            attributes: [("r:id".into(), "original".into())].into(),
            attribute_namespaces: [("r:id".into(), ns.into())].into(),
            ..Default::default()
        })],
        ..Default::default()
    });
    doc.pages[0].shapes = vec![shape].into();
    doc.parts.entry(doc.pages[0].part.clone()).or_default().relationships.push(Relationship {
        id: "original".into(),
        type_uri: format!("{ns}/image"),
        target: "visio/media/original.emf".into(),
        external: false,
    });
    doc.parts.insert("visio/media/original.emf".into(), Part { bytes: bytes.into(), content_type: Some("image/x-emf".into()), ..Default::default() });
    doc
}
