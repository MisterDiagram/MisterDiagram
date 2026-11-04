//! Stable shape/label membership, independently inspectable before XML emission.
//! ECMA-376 Part 1, 19.3.1.22 (grpSp), 19.3.1.51 (txBody).
//! https://learn.microsoft.com/en-us/office/client-developer/visio/txtpinx-cell-text-transform-section
use super::connectors::Connections;
use kurbo::{Rect, Shape as _};
use misterdiagram_render::scene::{EvaluatedShape, Op, SceneBudget};
use misterdiagram_vsdx::evaluate::Scope;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Node {
    pub shape_id: u32,
    pub group_id: Option<u32>,
    pub parent_id: Option<u32>,
    pub text_id: Option<u32>,
    pub inline: bool,
    pub one_d: bool,
}
#[derive(Default)]
pub struct Plan {
    pub nodes: BTreeMap<Scope, Node>,
    pub(crate) next_id: u32,
}
impl Plan {
    pub(super) fn prepare(shapes: &[EvaluatedShape], connections: &Connections, height: f64, budget: &mut SceneBudget) -> Result<Self, String> {
        let next_id =
            connections.ids.values().chain(connections.targets.values()).copied().max().unwrap_or(1).checked_add(1).ok_or("PPTX id limit")?;
        let mut result = Self { next_id, ..Default::default() };
        result.collect(shapes, connections, crate::write::page_flip(height), budget, None, 0)?;
        Ok(result)
    }
    fn next(&mut self) -> Result<u32, String> {
        let id = self.next_id;
        if id > 200_000 {
            return Err("PPTX shape id limit".into());
        }
        self.next_id = id.checked_add(1).ok_or("PPTX shape id limit")?;
        Ok(id)
    }
    fn collect(
        &mut self,
        nodes: &[EvaluatedShape],
        connections: &Connections,
        view: kurbo::Affine,
        budget: &mut SceneBudget,
        parent: Option<u32>,
        depth: usize,
    ) -> Result<(), String> {
        if depth > 128 {
            return Err("PPTX label depth limit".into());
        }
        for node in nodes {
            budget.charge_bytes(160)?;
            let shape_id = *connections.ids.get(&node.scope).ok_or("PPTX label shape id missing")?;
            let visible = !node.hidden_own || connections.attached.contains(&node.scope);
            let text = visible && node.text_body.is_some() && node.text_block.is_some();
            let mut bounds = None;
            for section in node.sections.iter().filter(|s| !s.flags.no_show) {
                let b = (view * &section.path).bounding_box();
                bounds = Some(bounds.map_or(b, |old: Rect| old.union(b)));
            }
            let bounds = bounds.unwrap_or_else(|| view.transform_rect_bbox(node.bounds));
            let placement = if text { node.text_block.as_ref().map(|b| crate::write::text::placement(b, view)).transpose()? } else { None };
            // Test coincidence before presentation scaling, so the choice is
            // stable across pages and the intent manifest needs no slide writer.
            let inline = !node.one_d
                && placement.is_some_and(|(b, angle, _)| {
                    angle.abs() < 1e-9
                        && [b.x0 - bounds.x0, b.y0 - bounds.y0, b.x1 - bounds.x1, b.y1 - bounds.y1].iter().all(|v| v.abs() < 1e-9)
                        && node.drawing.ops.iter().all(|op| matches!(op, Op::Text { .. } | Op::Bounds(_)))
                });
            let carrier = node.one_d && connections.targets.get(&node.scope) != connections.ids.get(&node.scope);
            let group_id = if node.group || carrier || (text && !inline && !node.one_d) { Some(self.next()?) } else { None };
            let parent_id = group_id.or(parent);
            let text_id = if inline {
                Some(shape_id)
            } else if text {
                Some(self.next()?)
            } else {
                None
            };
            if self.nodes.insert(node.scope, Node { shape_id, group_id, parent_id, text_id, inline, one_d: node.one_d }).is_some() {
                return Err("PPTX duplicate label identity".into());
            }
            self.collect(&node.children, connections, view, budget, parent_id, depth + 1)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn auxiliary_ids_refuse_at_the_existing_limit_without_wrapping() {
        let mut plan = Plan { next_id: 200_000, ..Default::default() };
        assert_eq!(plan.next().unwrap(), 200_000);
        assert!(plan.next().is_err());
        plan.next_id = u32::MAX;
        assert!(plan.next().is_err());
    }
}
