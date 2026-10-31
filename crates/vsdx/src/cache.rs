//! Reader-owned proofs for reuse of an admitted immutable package part.
use misterdiagram_model::{Connect, Part, Shape, Shared, XmlElement};
use std::sync::OnceLock;

pub(crate) type Contents = (Shared<Vec<Shape>>, Shared<Vec<Connect>>, crate::repair::Repairs);
pub(crate) struct CachedXml {
    pub owner: String,
    pub bytes: Shared<Vec<u8>>,
    pub xml: Shared<XmlElement>,
    pub nodes: usize,
    pub contents: OnceLock<Contents>,
}
/// Neither a caller's mutable DOM nor a matching filename establishes this proof.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx
pub(crate) fn admitted<'a>(part: &'a Part, owner: &str) -> Option<&'a CachedXml> {
    part.memo.get::<CachedXml>().filter(|cached| cached.owner == owner && cached.bytes.ptr_eq(&part.bytes))
}

/// Reader-proven shared page/master content allocations and their admitted nodes.
/// A detached source, DOM, shape list or Connect list is never discounted. Metadata
/// sheets live outside these content parts and remain charged per document.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pagecontents-elementvisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/mastercontents-elementvisio-xml
pub fn shared_content_nodes(document: &misterdiagram_model::Document) -> Vec<(Shared<XmlElement>, usize)> {
    document
        .pages
        .iter()
        .map(|p| (&p.part, &p.shapes, &p.connects))
        .chain(document.masters.iter().map(|m| (&m.part, &m.shapes, &m.connects)))
        .filter_map(|(name, shapes, connects)| {
            let part = document.parts.get(name)?;
            let cached = admitted(part, name)?;
            let xml = part.xml.as_ref()?;
            let contents = cached.contents.get()?;
            (xml.ptr_eq(&cached.xml) && shapes.ptr_eq(&contents.0) && connects.ptr_eq(&contents.1)).then(|| (xml.clone(), cached.nodes))
        })
        .collect()
}
