//! One census for retained document identities, including transaction owners.
use crate::{Result, Session, operation};
use misterdiagram_model::Document;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

const MIB: usize = 1024 * 1024;
/// Session retention policy. File/parser/render bounds are independent constants.
#[derive(Clone, Copy, Debug)]
pub struct MemoryLimits {
    bytes: usize,
}
impl Default for MemoryLimits {
    fn default() -> Self {
        Self { bytes: 2048 * MIB }
    }
}
impl MemoryLimits {
    /// Construct a ceiling from positive whole MiB, with checked conversion.
    pub fn from_mib(mib: usize) -> Result<Self> {
        let bytes = mib
            .checked_mul(MIB)
            .filter(|n| *n > 0)
            .ok_or_else(|| operation("memory limit must be positive whole MiB within this platform's address range"))?;
        Ok(Self { bytes })
    }
    pub fn mib(self) -> usize {
        self.bytes / MIB
    }
    pub(crate) fn source(self) -> usize {
        self.bytes / 16
    }
    pub(crate) fn media(self) -> usize {
        self.bytes / 4
    }
    pub(crate) fn history(self) -> usize {
        self.bytes / 8
    }
    pub(crate) fn resident(self) -> usize {
        self.bytes - self.bytes / 4
    }
}

#[derive(Clone, Copy, Default)]
pub(crate) struct Usage {
    pub nodes: usize,
    pub source: usize,
    pub media: usize,
    pub history: usize,
    pub raw: usize,
    pub documents: usize,
}
fn media(part: &misterdiagram_model::Part) -> bool {
    part.xml.is_none() && part.content_type.as_deref().is_some_and(|kind| kind.starts_with("image/"))
}
type Contents = HashMap<usize, misterdiagram_model::Shared<misterdiagram_model::XmlElement>>;
type Pictures = HashMap<usize, misterdiagram_model::Shared<Vec<u8>>>;
fn retain_picture(pictures: &mut Pictures, part: &misterdiagram_model::Part) -> bool {
    if let std::collections::hash_map::Entry::Vacant(entry) = pictures.entry(part.bytes.allocation_id()) {
        // Pin each identity for the reservation lifetime so a freed address cannot
        // be mistaken for an unchanged picture allocation.
        entry.insert(part.bytes.clone());
        true
    } else {
        false
    }
}
impl Usage {
    fn resident(self) -> usize {
        self.raw
            .saturating_sub(self.media)
            .saturating_mul(2)
            .saturating_add(self.media)
            .saturating_add(self.nodes.saturating_mul(320))
            .saturating_add(self.documents.saturating_mul(MIB))
    }
    fn add(
        &mut self,
        doc: &Document,
        stencil: bool,
        history: bool,
        pictures: &mut Pictures,
        history_parts: &mut HashSet<usize>,
        contents: &mut Contents,
    ) {
        let mut duplicate = 0usize;
        for (xml, nodes) in misterdiagram_vsdx::shared_content_nodes(doc) {
            if let std::collections::hash_map::Entry::Vacant(entry) = contents.entry(xml.allocation_id()) {
                // Reservations pin the allocation until their census is released.
                entry.insert(xml);
            } else {
                duplicate = duplicate.saturating_add(nodes);
            }
        }
        self.nodes = self.nodes.saturating_add(doc.retained_xml_nodes.saturating_sub(duplicate));
        self.documents = self.documents.saturating_add(1);
        let mut estimate = MIB;
        for part in doc.parts.values() {
            if media(part) && !retain_picture(pictures, part) {
                continue;
            }
            self.raw = self.raw.saturating_add(part.bytes.len());
            if media(part) {
                self.media = self.media.saturating_add(part.bytes.len());
            } else {
                if stencil {
                    self.source = self.source.saturating_add(part.bytes.len());
                }
                // All retained owners stay alive for this census. An immutable source
                // allocation shared across historical versions is charged once.
                if history && !stencil && history_parts.insert(part.bytes.allocation_id()) {
                    estimate = estimate.saturating_add(part.bytes.len().saturating_mul(16));
                }
            }
        }
        if history && !stencil {
            self.history = self.history.saturating_add(estimate);
        }
    }
}
impl Session {
    /// Reserve declared package buffers before the reader decompresses them.
    /// The reader independently enforces all unchanged hostile-input bounds.
    pub(crate) fn read_document(&self, path: &std::path::Path, stencil: bool) -> Result<Document> {
        use misterdiagram_opc::{MAX_ENTRIES, MAX_PART, MAX_TOTAL};
        let file = std::fs::File::open(path).map_err(operation)?;
        if file.metadata().map_err(operation)?.len() > MAX_PART {
            return Err(operation("compressed file exceeds cap"));
        }
        let mut archive = zip::ZipArchive::new(file).map_err(operation)?;
        if archive.len() > MAX_ENTRIES {
            return Err(operation("too many package entries"));
        }
        let mut bytes = 0u64;
        for i in 0..archive.len() {
            let entry = archive.by_index(i).map_err(operation)?;
            if entry.is_dir() {
                continue;
            }
            if entry.size() > MAX_PART {
                return Err(operation("oversized part: per-file limit 512 MiB"));
            }
            bytes = bytes.checked_add(entry.size()).filter(|n| *n <= MAX_TOTAL).ok_or_else(|| operation("total size cap exceeded"))?;
        }
        let needed = usize::try_from(bytes).map_err(operation)?;
        let held = self.retention();
        // This envelope cannot exceed the combined source/picture allowance.
        let allowance = self.memory.source().saturating_add(self.memory.media());
        if needed > allowance {
            return Err(retention_error("incoming package source reservation", allowance, needed, 0));
        }
        let mut total = held;
        total.raw = total.raw.saturating_add(needed);
        self.check_retention(held, total)?;
        let document = misterdiagram_vsdx::read_with_open_nodes(path, held.nodes).map_err(|error| admission_error(error, held.nodes))?;
        self.reserve(Some((&document, stencil)))?;
        Ok(document)
    }

    pub(crate) fn retained_stencils(&self) -> Vec<Arc<Document>> {
        let mut documents = self.stencil_reservations.clone();
        documents.extend(self.stencils.iter().map(|s| s.document.clone()));
        for snapshot in self.undo.iter().chain(&self.redo) {
            documents.extend(snapshot.stencils.iter().flatten().map(|s| s.document.clone()));
        }
        documents
    }
    pub(crate) fn retention(&self) -> Usage {
        self.retention_census().0
    }
    fn retention_census(&self) -> (Usage, Pictures, Contents) {
        let stencils: HashSet<_> = self.retained_stencils().iter().map(Arc::as_ptr).collect();
        let history: HashSet<_> = self.undo.iter().chain(&self.redo).map(|s| Arc::as_ptr(&s.document)).collect();
        let mut seen = HashSet::new();
        let mut usage = Usage::default();
        let mut pictures = Pictures::new();
        let mut history_parts = HashSet::new();
        let mut contents = Contents::new();
        for doc in self.retained_documents() {
            let identity = Arc::as_ptr(doc);
            if seen.insert(identity) {
                usage.add(doc, stencils.contains(&identity), history.contains(&identity), &mut pictures, &mut history_parts, &mut contents);
            }
        }
        (usage, pictures, contents)
    }
    /// Opening, editing and reader replacement share the same admission function.
    /// https://learn.microsoft.com/en-us/office/vba/api/visio.documents.openex
    pub(crate) fn reserve(&self, incoming: Option<(&Document, bool)>) -> Result<()> {
        let (held, mut pictures, mut contents) = self.retention_census();
        let mut total = held;
        if let Some((doc, stencil)) = incoming {
            total.add(doc, stencil, false, &mut pictures, &mut HashSet::new(), &mut contents);
        }
        self.check_retention(held, total)
    }
    pub(crate) fn check_retention(&self, held: Usage, total: Usage) -> Result<()> {
        check(self.memory, held, total)
    }
    pub(crate) fn reservation(&self) -> Reservation {
        let (held, pictures, contents) = self.retention_census();
        Reservation { memory: self.memory, held, pictures, contents }
    }
}
fn check(memory: MemoryLimits, held: Usage, total: Usage) -> Result<()> {
    for (name, limit, already, amount) in [
        ("stencil source", memory.source(), held.source, total.source),
        ("stencil media", memory.media(), held.media, total.media),
        ("session resident estimate", memory.resident(), held.resident(), total.resident()),
    ] {
        if amount > limit {
            return Err(retention_error(name, limit, amount.saturating_sub(already), already));
        }
    }
    Ok(())
}
/// A mutation's original owners remain pinned until its last publication.
#[derive(Clone)]
pub(crate) struct Reservation {
    memory: MemoryLimits,
    held: Usage,
    pictures: Pictures,
    contents: Contents,
}
impl Reservation {
    pub(crate) fn parts<'a>(&self, document: &Document, parts: impl Iterator<Item = &'a misterdiagram_model::Part>) -> Result<usize> {
        let mut total = self.held;
        let mut pictures = self.pictures.clone();
        let mut contents = self.contents.clone();
        total.add(document, false, false, &mut pictures, &mut HashSet::new(), &mut contents);
        let initial_nodes = total.nodes;
        for part in parts {
            if media(part) && !retain_picture(&mut pictures, part) {
                continue;
            }
            total.raw = total.raw.saturating_add(part.bytes.len());
            if media(part) {
                total.media = total.media.saturating_add(part.bytes.len());
            }
            // Imported immutable bytes preserve namespace declarations and the
            // reader's coalesced-text accounting exactly, before any clone.
            if part.xml.is_some() && !part.bytes.is_empty() {
                misterdiagram_opc::xml::charge_retained_nodes(&part.bytes, &mut total.nodes, misterdiagram_vsdx::MAX_RETAINED_XML_NODES, self.held.nodes)
                    .map_err(|error| operation(format!("{error}; needs at least {} new nodes, {} already retained; --memory-limit and MISTERDIAGRAM_MEMORY_LIMIT do not change this fixed admission bound", total.nodes.saturating_sub(self.held.nodes), self.held.nodes)))?;
                continue;
            }
            // Copying a retained DOM also consumes node storage before reimport.
            let mut pending = part.xml.iter().map(|root| &**root).collect::<Vec<_>>();
            while let Some(element) = pending.pop() {
                total.nodes = total.nodes.saturating_add(1).saturating_add(element.attributes.len());
                for child in &element.children {
                    match child {
                        misterdiagram_model::XmlNode::Element(e) => pending.push(e),
                        misterdiagram_model::XmlNode::Text(_) => total.nodes = total.nodes.saturating_add(1),
                    }
                }
                if total.nodes > misterdiagram_vsdx::MAX_RETAINED_XML_NODES {
                    return Err(operation(format!(
                        "retained XML node limit: needs {} nodes, {} already retained, limit {}; --memory-limit and MISTERDIAGRAM_MEMORY_LIMIT do not change this fixed admission bound",
                        total.nodes.saturating_sub(self.held.nodes),
                        self.held.nodes,
                        misterdiagram_vsdx::MAX_RETAINED_XML_NODES
                    )));
                }
            }
        }
        check(self.memory, self.held, total)?;
        Ok(total.nodes.saturating_sub(initial_nodes))
    }
}

pub(crate) fn fixed_admission_error(name: &str, needed: usize, held: usize, limit: usize, units: &str) -> crate::EngineError {
    operation(format!(
        "retained {name} limit: needs {needed} {units}, {held} already retained, limit {limit}; --memory-limit and MISTERDIAGRAM_MEMORY_LIMIT do not change this fixed admission bound"
    ))
}

/// Add session guidance without changing the reader's independent admission policy.
pub(crate) fn admission_error(error: impl std::fmt::Display, held: usize) -> crate::EngineError {
    let error = error.to_string();
    if error.contains("retained XML node limit") {
        let needed = misterdiagram_vsdx::MAX_RETAINED_XML_NODES.saturating_sub(held).saturating_add(1);
        operation(format!(
            "{error}; needs at least {needed} new nodes, {held} already retained; --memory-limit and MISTERDIAGRAM_MEMORY_LIMIT do not change this fixed admission bound"
        ))
    } else {
        operation(error)
    }
}

pub(crate) fn retention_error(name: &str, limit: usize, needed: usize, held: usize) -> crate::EngineError {
    let displayed_limit = if limit.is_multiple_of(MIB) { (limit / MIB).to_string() } else { format!("{:.6}", limit as f64 / MIB as f64) };
    operation(format!(
        "retained {name} byte limit: needs {:.1} MiB, {:.1} MiB already retained, limit {displayed_limit} MiB; reservation bytes: needs {needed}, already retained {held}, limit {limit}; raise --memory-limit <MiB> or MISTERDIAGRAM_MEMORY_LIMIT",
        needed as f64 / MIB as f64,
        held as f64 / MIB as f64,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    /// https://learn.microsoft.com/en-us/office/vba/api/visio.application.endundoscope
    #[test]
    fn reimport_node_refusal_names_fixed_bound_and_restores_document() {
        let mut s = Session::with_memory_limits(MemoryLimits::from_mib(8192).unwrap());
        s.execute("sheet.set", &json!({"scope":"page","name":"PageWidth","value":20})).unwrap();
        let mut stencil = crate::files::blank_document(false);
        stencil.retained_xml_nodes = misterdiagram_vsdx::MAX_RETAINED_XML_NODES - s.retained_xml_nodes();
        s.stencils.push(crate::stencils::Stencil { path: "original-node-reservation".into(), document: Arc::new(stencil) });
        let before = s.document.clone();
        let history = s.undo.len();
        let error = s.execute("sheet.set", &json!({"scope":"page","name":"PageWidth","value":21})).unwrap_err().to_string();
        assert!(error.contains("retained XML node limit"), "{error}");
        assert!(error.contains("needs at least"), "{error}");
        assert!(error.contains("--memory-limit and MISTERDIAGRAM_MEMORY_LIMIT do not change this fixed admission bound"), "{error}");
        assert!(Arc::ptr_eq(&before, &s.document));
        assert_eq!(s.undo.len(), history);
    }
    #[test]
    fn fractional_limits_and_small_reservations_are_exact_in_errors() {
        let error = retention_error("stencil source", MIB / 16, 19, MIB / 16).to_string();
        assert!(error.contains("limit 0.062500 MiB"), "{error}");
        assert!(error.contains("reservation bytes: needs 19, already retained 65536, limit 65536"), "{error}");
    }
    #[test]
    fn inherited_reservations_survive_compound_and_batch_publication() {
        let mut s = Session::new();
        let mut sentinel = s.document.as_ref().clone();
        sentinel.retained_xml_nodes = 12345;
        sentinel.parts.insert("original.bin".into(), misterdiagram_model::Part { bytes: vec![0; 4096].into(), ..Default::default() });
        let sentinel = Arc::new(sentinel);
        s.read_reservations = vec![sentinel.clone(), sentinel.clone()];
        s.stencil_reservations = vec![sentinel.clone()];
        let source = s.retention().source;
        assert_eq!(s.retention().nodes, 12345);
        let params = json!({"x":1,"y":1,"width":2,"height":1,"text":"hello"});
        s.execute("shape.draw.rect", &params).unwrap();
        s.execute_batch(&json!({"commands":[{"command":"shape.draw.rect","params":params}]})).unwrap();
        assert_eq!(s.retention().source, source);
        assert!(s.read_reservations.iter().any(|d| Arc::ptr_eq(d, &sentinel)));
        assert!(s.stencil_reservations.iter().any(|d| Arc::ptr_eq(d, &sentinel)));
    }
    #[test]
    fn copied_resources_reserve_media_and_cumulative_nodes_before_clone() {
        let mut s = Session::with_memory_limits(MemoryLimits::from_mib(16).unwrap());
        let part = misterdiagram_model::Part { bytes: vec![0; 5 * MIB].into(), content_type: Some("image/bmp".into()), ..Default::default() };
        let error = s.reservation().parts(s.document(), std::iter::once(&part)).unwrap_err().to_string();
        assert!(error.contains("needs 5.0 MiB") && error.contains("limit 4 MiB"), "{error}");
        s.memory = MemoryLimits::default();
        let mut doc = s.document.as_ref().clone();
        doc.retained_xml_nodes = misterdiagram_vsdx::MAX_RETAINED_XML_NODES;
        let part = misterdiagram_model::Part {
            xml: Some(misterdiagram_model::XmlElement { name: "Original".into(), ..Default::default() }.into()),
            ..Default::default()
        };
        assert!(s.reservation().parts(&doc, std::iter::once(&part)).unwrap_err().to_string().contains("retained XML node limit"));
    }
    #[test]
    fn picture_census_deduplicates_versions_and_reservations_but_not_replacements() {
        let mut session = Session::new();
        let image =
            misterdiagram_model::Part { bytes: vec![7; 4096].into(), content_type: Some("image/x-original-test".into()), ..Default::default() };
        Arc::make_mut(&mut session.document).parts.insert("visio/media/original.bin".into(), image.clone());
        let original = session.document.clone();
        let mut version = original.as_ref().clone();
        version.pages[0].name = "Another version".into();
        session.read_reservations.push(original.clone());
        session.document = Arc::new(version);
        assert_eq!(session.retention().media, 4096);
        session.reserve(Some((&original, false))).unwrap();
        let reservation = session.reservation();
        reservation.parts(session.document(), std::iter::once(&image)).unwrap();
        let mut replaced = original.as_ref().clone();
        replaced.parts.get_mut("visio/media/original.bin").unwrap().bytes[0] = 11;
        session.document = Arc::new(replaced);
        assert_eq!(session.retention().media, 8192);
        assert_eq!(original.parts["visio/media/original.bin"].bytes[0], 7);
        assert_eq!(session.document.parts["visio/media/original.bin"].bytes[0], 11);
        let separate = misterdiagram_model::Part { bytes: vec![7; 4096].into(), ..image };
        assert!(!separate.bytes.ptr_eq(&original.parts["visio/media/original.bin"].bytes));
    }
    #[test]
    fn shared_nonmedia_parts_keep_thirty_two_ordinary_undo_snapshots() {
        let mut s = Session::new();
        s.edit_document(|doc, _| {
            doc.parts.insert(
                "visio/original-history.bin".into(),
                misterdiagram_model::Part { bytes: vec![7; MIB].into(), content_type: Some("application/octet-stream".into()), ..Default::default() },
            );
            Ok(())
        })
        .unwrap();
        for i in 1..=40 {
            s.execute("page.rename", &json!({"name":format!("history {i}")})).unwrap();
        }
        assert_eq!(s.undo.len(), 32, "shared parts must not evict ordinary history");
        assert!(s.retention().history < 64 * MIB);
        for _ in 0..32 {
            assert_eq!(s.execute("edit.undo", &json!({})).unwrap()["changed"], true);
        }
        assert_eq!(s.document.pages[0].name, "history 8");
        assert_eq!(s.execute("edit.undo", &json!({})).unwrap()["changed"], false);
        for _ in 0..32 {
            assert_eq!(s.execute("edit.redo", &json!({})).unwrap()["changed"], true);
        }
        assert_eq!(s.document.pages[0].name, "history 40");
    }
    #[test]
    fn history_charges_detached_and_equal_but_distinct_nonmedia_allocations() {
        let mut doc = Document::default();
        doc.parts.insert("original.bin".into(), misterdiagram_model::Part { bytes: vec![7; 4096].into(), ..Default::default() });
        let mut detached = doc.clone();
        detached.parts.get_mut("original.bin").unwrap().bytes[0] = 11;
        let mut equal = doc.clone();
        equal.parts.get_mut("original.bin").unwrap().bytes = vec![7; 4096].into();
        let mut usage = Usage::default();
        let mut pictures = Pictures::new();
        let mut history_parts = HashSet::new();
        let mut contents = Contents::new();
        for d in [&doc, &doc, &detached, &equal] {
            usage.add(d, false, true, &mut pictures, &mut history_parts, &mut contents);
        }
        assert_eq!(usage.history, 4 * MIB + 3 * 4096 * 16);
        assert_eq!(usage.raw, 4 * 4096, "resident/source admission remains conservative");
        assert_eq!(doc.parts["original.bin"].bytes[0], 7);
    }
    #[test]
    fn shared_content_resident_charge_requires_every_reader_allocation() {
        let mut original = misterdiagram_model::skeleton::document(false);
        original.pages[0].shapes.push(misterdiagram_model::skeleton::rectangle(1, 0., 0., 1., 1., "Original").unwrap());
        let doc = misterdiagram_vsdx::import(&misterdiagram_vsdx::write(&original).unwrap().bytes).unwrap();
        let allocations = misterdiagram_vsdx::shared_content_nodes(&doc);
        let count: usize = allocations.iter().map(|(_, n)| n).sum();
        assert!(count > 0 && count < doc.retained_xml_nodes);
        let measure = |other: &Document| {
            let mut usage = Usage::default();
            let mut pictures = Pictures::new();
            let mut history = HashSet::new();
            let mut contents = Contents::new();
            usage.add(&doc, false, true, &mut pictures, &mut history, &mut contents);
            usage.add(other, false, true, &mut pictures, &mut history, &mut contents);
            usage
        };
        let shared = measure(&doc.clone());
        assert_eq!(shared.nodes, doc.retained_xml_nodes * 2 - count);
        assert_eq!(shared.resident(), shared.raw * 2 + shared.nodes * 320 + 2 * MIB);
        let page_part = doc.pages[0].part.clone();
        let page_nodes = allocations.iter().find(|(xml, _)| xml.ptr_eq(doc.parts[&page_part].xml.as_ref().unwrap())).unwrap().1;
        for kind in 0..4 {
            let mut detached = doc.clone();
            match kind {
                0 => {
                    let part = detached.parts.get_mut(&page_part).unwrap();
                    part.bytes = part.bytes.to_vec().into();
                }
                1 => {
                    let part = detached.parts.get_mut(&page_part).unwrap();
                    part.xml = Some((**part.xml.as_ref().unwrap()).clone().into());
                }
                2 => detached.pages[0].shapes[0].name = "Detached".into(),
                _ => detached.pages[0].connects.push(misterdiagram_model::Connect::default()),
            }
            let usage = measure(&detached);
            assert_eq!(usage.nodes, shared.nodes + page_nodes, "detached kind {kind}");
            assert_eq!(usage.resident(), shared.resident() + page_nodes * 320);
        }
        let separate = misterdiagram_vsdx::import(&misterdiagram_vsdx::write(&original).unwrap().bytes).unwrap();
        assert_eq!(measure(&separate).nodes, doc.retained_xml_nodes * 2, "equal content is not shared storage");
    }
}
