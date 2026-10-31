//! Native import repairs, observed in field/{dangling-connects,duplicate-ids}-2026-10-31.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
//! https://learn.microsoft.com/en-us/office/client-developer/visio/connect-element-connects_type-complextypevisio-xml
use misterdiagram_model::{Connect, Document, Shape};
use std::collections::BTreeSet;

#[derive(Clone, Debug)]
pub(crate) enum Repair {
    Duplicate(u32),
    Dangling { from: u32, to: u32 },
}
impl std::fmt::Display for Repair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Duplicate(id) => write!(f, "dropped duplicate shape ID {id} and its subtree; kept the first occurrence"),
            Self::Dangling { from, to } => write!(f, "dropped Connect from sheet {from} to sheet {to}: missing or deleted sheet"),
        }
    }
}
#[derive(Clone, Default)]
pub(crate) struct Repairs {
    entries: Vec<Repair>,
    count: usize,
}
impl Repairs {
    fn push(&mut self, repair: Repair) {
        self.count = self.count.saturating_add(1);
        if self.entries.len() < 100 {
            self.entries.push(repair);
        }
    }
}
pub(crate) fn deleted(shape: &Shape) -> bool {
    shape.sheet.attributes.get("Del").is_some_and(|v| v == "1" || v == "true")
}
pub(crate) fn tombstone(shape: &Shape) -> bool {
    shape.id == u32::MAX && shape.master_shape.is_some() && deleted(shape)
}
/// The first depth-first occurrence owns an ID. Deleted inherited sentinels are distinct.
pub(crate) fn first(shape: &Shape, seen: &mut BTreeSet<u32>) -> bool {
    tombstone(shape) || seen.insert(shape.id)
}
/// Live endpoint IDs within one contents part. Sheet 0 names its enclosing PageSheet.
pub(crate) fn active_ids(shapes: &[Shape]) -> BTreeSet<u32> {
    let mut active = BTreeSet::from([0]);
    let mut seen = BTreeSet::new();
    let mut pending: Vec<_> = shapes.iter().rev().collect();
    while let Some(shape) = pending.pop() {
        if !first(shape, &mut seen) || deleted(shape) {
            continue;
        }
        active.insert(shape.id);
        pending.extend(shape.shapes.iter().rev());
    }
    active
}
/// Validate every original occurrence before pruning, including discarded subtrees.
pub(crate) fn normalize(shapes: &mut Vec<Shape>, connects: &mut Vec<Connect>) -> Result<Repairs, crate::ReadError> {
    if shapes.len() > 100_000 || connects.len() > 100_000 {
        return Err(crate::bad("shape or connect count cap"));
    }
    let mut pending: Vec<_> = shapes.iter().map(|s| (s, 0usize)).collect();
    let mut count = 0usize;
    while let Some((shape, depth)) = pending.pop() {
        count = count.saturating_add(1);
        if depth > 128 || count.saturating_add(pending.len()).saturating_add(shape.shapes.len()) > 100_000 {
            return Err(crate::bad("shape count or nesting cap"));
        }
        pending.extend(shape.shapes.iter().map(|s| (s, depth + 1)));
    }
    fn retain(shapes: &mut Vec<Shape>, seen: &mut BTreeSet<u32>, repairs: &mut Repairs) {
        shapes.retain_mut(|shape| {
            if !first(shape, seen) {
                repairs.push(Repair::Duplicate(shape.id));
                return false;
            }
            // Deleted owners reserve their own ID, but their hidden children do not.
            // Native: field/duplicate-ids-2026-10-31/native-deleted-duplicates.
            if !deleted(shape) {
                retain(&mut shape.shapes, seen, repairs);
            }
            true
        });
    }
    let mut repairs = Repairs::default();
    retain(shapes, &mut BTreeSet::new(), &mut repairs);
    let ids = active_ids(shapes);
    connects.retain(|c| {
        let keep = ids.contains(&c.from_sheet) && ids.contains(&c.to_sheet);
        if !keep {
            repairs.push(Repair::Dangling { from: c.from_sheet, to: c.to_sheet });
        }
        keep
    });
    Ok(repairs)
}
/// Import repairs remain visible until the cleaned package is reopened.
/// Messages are bounded to 100; the count includes every discarded record.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pagecontents-elementvisio-xml
pub fn import_warnings(document: &Document) -> (Vec<String>, usize) {
    let mut messages = Vec::new();
    let mut count = 0usize;
    for (kind, id, path) in document.pages.iter().map(|p| ("page", p.id, &p.part)).chain(document.masters.iter().map(|m| ("master", m.id, &m.part))) {
        let Some(repairs) =
            document.parts.get(path).and_then(|part| crate::cache::admitted(part, path)).and_then(|cached| cached.contents.get()).map(|c| &c.2)
        else {
            continue;
        };
        count = count.saturating_add(repairs.count);
        for repair in &repairs.entries {
            if messages.len() == 100 {
                break;
            }
            messages.push(format!("{kind} {id}: {repair}"));
        }
    }
    (messages, count)
}
