//! One census for retained document identities, including transaction owners.
use crate::{Result, Session, operation};
use misterdiagram_model::Document;
use std::{collections::HashSet, sync::Arc};

const MIB: usize = 1024 * 1024;
#[derive(Clone, Copy, Default)]
pub(crate) struct Usage {
    pub nodes: usize,
    pub source: usize,
    pub media: usize,
    pub history: usize,
}
fn media(part: &misterdiagram_model::Part) -> bool {
    part.xml.is_none() && part.content_type.as_deref().is_some_and(|kind| kind.starts_with("image/"))
}
impl Usage {
    fn add(&mut self, doc: &Document, stencil: bool, history: bool) {
        self.nodes = self.nodes.saturating_add(doc.retained_xml_nodes);
        let mut estimate = MIB;
        for part in doc.parts.values() {
            if media(part) {
                self.media = self.media.saturating_add(part.bytes.len());
            } else {
                if stencil {
                    self.source = self.source.saturating_add(part.bytes.len());
                }
                estimate = estimate.saturating_add(part.bytes.len().saturating_mul(16));
            }
        }
        if history && !stencil {
            self.history = self.history.saturating_add(estimate);
        }
    }
}
impl Session {
    pub(crate) fn retained_stencils(&self) -> Vec<Arc<Document>> {
        let mut documents = self.stencil_reservations.clone();
        documents.extend(self.stencils.iter().map(|s| s.document.clone()));
        for snapshot in self.undo.iter().chain(&self.redo) {
            documents.extend(snapshot.stencils.iter().flatten().map(|s| s.document.clone()));
        }
        documents
    }
    pub(crate) fn retention(&self) -> Usage {
        let stencils: HashSet<_> = self.retained_stencils().iter().map(Arc::as_ptr).collect();
        let history: HashSet<_> = self.undo.iter().chain(&self.redo).map(|s| Arc::as_ptr(&s.document)).collect();
        let mut seen = HashSet::new();
        let mut usage = Usage::default();
        for doc in self.retained_documents() {
            let identity = Arc::as_ptr(doc);
            if seen.insert(identity) {
                usage.add(doc, stencils.contains(&identity), history.contains(&identity));
            }
        }
        usage
    }
    /// Opening, editing and reader replacement share the same admission function.
    /// https://learn.microsoft.com/en-us/office/vba/api/visio.documents.openex
    pub(crate) fn reserve(&self, incoming: Option<(&Document, bool)>) -> Result<()> {
        let held = self.retention();
        let mut total = held;
        if let Some((doc, stencil)) = incoming {
            total.add(doc, stencil, false);
        }
        self.check_retention(held, total)
    }
    pub(crate) fn check_retention(&self, held: Usage, total: Usage) -> Result<()> {
        for (name, limit, already, amount) in
            [("stencil source", 128 * MIB, held.source, total.source), ("stencil media", 256 * MIB, held.media, total.media)]
        {
            if amount > limit {
                return Err(retention_error(name, limit, amount.saturating_sub(already), already));
            }
        }
        Ok(())
    }
}
pub(crate) fn retention_error(name: &str, limit: usize, needed: usize, held: usize) -> crate::EngineError {
    operation(format!(
        "retained {name} byte limit: needs {:.1} MiB, {:.1} MiB already retained, limit {} MiB; raise --memory-limit <MiB> or MISTERDIAGRAM_MEMORY_LIMIT",
        needed as f64 / MIB as f64,
        held as f64 / MIB as f64,
        limit / MIB,
    ))
}
