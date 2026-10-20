//! Reader-owned proofs for reuse of an admitted immutable package part.
use misterdiagram_model::{Connect, Part, Shape, Shared, XmlElement};
use std::sync::OnceLock;

pub(crate) type Contents = (Shared<Vec<Shape>>, Shared<Vec<Connect>>);
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
