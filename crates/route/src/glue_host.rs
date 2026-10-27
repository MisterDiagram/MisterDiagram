//! Runtime glue from target transforms and Connects, never from a precomputed endpoint cache.
use crate::static_glue::{End, cell, set, shape_mut};
use misterdiagram_geom::{Affine, Point};
use misterdiagram_model::Document;
use misterdiagram_sheet::{HostHooks, Value};
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
use std::collections::BTreeMap;

type Key = (u32, u32, String);
#[derive(Clone)]
struct BoxShape {
    transform: Affine,
    width: f64,
    height: f64,
    min: Point,
    max: Point,
    points: Vec<(Point, [f64; 2])>,
    ink: Option<Vec<misterdiagram_geom::Rect>>,
}
fn number(e: &mut Evaluator<'_>, scope: Scope, name: &str) -> Result<f64, String> {
    let n = e.evaluate(scope, name).numeric().map_err(|e| e.to_string())?;
    if n.is_finite() && n.abs() <= 1e6 { Ok(n) } else { Err("glue coordinate limit".into()) }
}
fn bounds(e: &mut Evaluator<'_>, page: u32, shape: u32) -> Result<BoxShape, String> {
    let scope = Scope::PageShape { page, shape };
    let width = number(e, scope, "Width")?;
    let height = number(e, scope, "Height")?;
    let transform = e.shape_to_page(scope).map_err(|e| e.to_string())?;
    let corners = [Point::new(0., 0.), Point::new(width, 0.), Point::new(width, height), Point::new(0., height)].map(|p| transform * p);
    let mut min = Point::new(f64::INFINITY, f64::INFINITY);
    let mut max = Point::new(f64::NEG_INFINITY, f64::NEG_INFINITY);
    for p in corners {
        if !p.x.is_finite() || !p.y.is_finite() || p.x.abs() > 1e6 || p.y.abs() > 1e6 {
            return Err("glue transform limit".into());
        }
        min.x = min.x.min(p.x);
        min.y = min.y.min(p.y);
        max.x = max.x.max(p.x);
        max.y = max.y.max(p.y);
    }
    let mut points = vec![];
    for named in e.resolved_named_cells(scope) {
        if named.cell.name != "X" || !named.section.is_some_and(|s| s.name == "Connection") {
            continue;
        }
        if points.len() >= 16384 {
            return Err("connection point count limit".into());
        }
        let row = named.row.ok_or("connection row missing")?;
        let sec = named.section.ok_or("connection section missing")?;
        let col = |name: &str| misterdiagram_vsdx::evaluate::cell_name(sec, row, 0, &cell(name, 0, None)).map_err(|e| e.to_string());
        let x = number(e, scope, &named.name)?;
        let y = number(e, scope, &col("Y")?)?;
        let dx = number(e, scope, &col("DirX")?)?;
        let dy = number(e, scope, &col("DirY")?)?;
        let kind = number(e, scope, &col("Type")?)?;
        // Whole-shape glue walks the perimeter. Interior rows remain available
        // to explicit static glue, but must not stop dynamic walking (#60, #69).
        // Witness: Brief26b interior-one/two/cylinder and vendor-recalculated.
        // https://learn.microsoft.com/en-us/office/client-developer/visio/connection-points-section
        let on_perimeter = x >= -1e-6
            && x <= width + 1e-6
            && y >= -1e-6
            && y <= height + 1e-6
            && [x.abs(), (x - width).abs(), y.abs(), (y - height).abs()].iter().any(|v| *v <= 1e-6);
        if (kind == 0. || kind == 2.) && on_perimeter {
            let vector = transform * Point::new(dx, dy) - transform * Point::ORIGIN;
            points.push((transform * Point::new(x, y), [vector.x, vector.y]));
        }
    }
    let ink = crate::foreign_glue::envelopes(e, page, shape)?;
    Ok(BoxShape { transform, width, height, min, max, points, ink })
}
/// One target can serve hundreds of endpoints. Reuse its evaluated geometry only
/// in the same evaluator epoch; assignments or contextual hooks prevent reuse.
#[derive(Default)]
struct TargetBounds {
    epoch: Option<(usize, u64)>,
    values: BTreeMap<(u32, u32), std::rc::Rc<BoxShape>>,
    points: usize,
}
impl TargetBounds {
    fn get(&mut self, e: &mut Evaluator<'_>, page: u32, shape: u32) -> Result<std::rc::Rc<BoxShape>, String> {
        let epoch = e.evaluation_epoch();
        if self.epoch != Some(epoch) {
            self.values.clear();
            self.points = 0;
            self.epoch = Some(epoch);
        }
        if let Some(value) = self.values.get(&(page, shape)) {
            return Ok(value.clone());
        }
        let value = std::rc::Rc::new(bounds(e, page, shape)?);
        // Cache saturation falls back to ordinary computation, never an input refusal.
        let points = value.points.len().checked_add(value.ink.as_ref().map_or(0, Vec::len)).and_then(|n| self.points.checked_add(n));
        if e.evaluation_epoch() == epoch && self.values.len() < 4096 && points.is_some_and(|n| n <= 65536) {
            self.points = points.ok_or("target memo point overflow")?;
            self.values.insert((page, shape), value.clone());
        }
        Ok(value)
    }
}
fn center(b: &BoxShape) -> Point {
    Point::new((b.min.x + b.max.x) * 0.5, (b.min.y + b.max.y) * 0.5)
}
/// Intersect the facing ray with the transformed alignment rectangle.
fn facing(b: &BoxShape, aim: Point) -> Result<Point, String> {
    let determinant = b.transform.determinant();
    if !determinant.is_finite() || determinant.abs() < 1e-15 {
        return Err("singular glue target transform".into());
    }
    let inv = b.transform.inverse();
    let a = inv * aim;
    let c = Point::new(b.width / 2., b.height / 2.);
    let d = a - c;
    let tx = if d.x.abs() > 1e-12 { (b.width.abs() / 2.) / d.x.abs() } else { f64::INFINITY };
    let ty = if d.y.abs() > 1e-12 { (b.height.abs() / 2.) / d.y.abs() } else { f64::INFINITY };
    let t = tx.min(ty);
    if !t.is_finite() {
        return Ok(b.transform * c);
    }
    Ok(b.transform * (c + d * t))
}
fn endpoint(b: &BoxShape, other: &BoxShape, vertical: bool) -> Result<Point, String> {
    let c = center(b);
    let far = center(other);
    // The four straight-route witnesses align the ends inside the overlap of both boxes.
    let low_y = b.min.y.max(other.min.y);
    let high_y = b.max.y.min(other.max.y);
    let low_x = b.min.x.max(other.min.x);
    let high_x = b.max.x.min(other.max.x);
    let horizontal_overlap = low_y <= high_y;
    let vertical_overlap = low_x <= high_x;
    let preferred_vertical = vertical && vertical_overlap || !horizontal_overlap && vertical_overlap;
    let candidate = if preferred_vertical {
        Point::new((low_x + high_x) * 0.5, if far.y >= c.y { b.max.y } else { b.min.y })
    } else if horizontal_overlap {
        Point::new(if far.x >= c.x { b.max.x } else { b.min.x }, (low_y + high_y) * 0.5)
    } else {
        facing(b, far)?
    };
    let mut best = None;
    for (point, dir) in &b.points {
        let horizontal = dir[0].abs() >= dir[1].abs();
        if horizontal == preferred_vertical && dir != &[0., 0.] {
            continue;
        }
        let d = (*point - far).hypot2();
        if best.is_none_or(|(_, old)| d < old) {
            best = Some((*point, d));
        }
    }
    if let Some((p, _)) = best {
        return Ok(p);
    }
    // A nonrotated alignment rectangle accepts the common overlap coordinate directly.
    let coeff = b.transform.as_coeffs();
    if coeff.get(1).is_some_and(|v| v.abs() < 1e-12) && coeff.get(2).is_some_and(|v| v.abs() < 1e-12) { Ok(candidate) } else { facing(b, candidate) }
}
/// Glue hooks for the current edit, indexed by page, connector and calling cell.
/// `_WALKGLUE` and `_XFTRIGGER` are private formula names observed in the corpus.
/// The public contract describes the EventXFMod dependency and endpoint recomputation.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/begtrigger-cell-glue-info-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/endtrigger-cell-glue-info-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/walkpreference-cell-glue-info-section
#[derive(Default)]
pub struct GlueHost {
    pub(crate) outcomes: BTreeMap<(u32, u32), Option<String>>,
    pub(crate) unaffected: bool,
    points: BTreeMap<Key, Point>,
    triggers: BTreeMap<Key, f64>,
    active: std::collections::BTreeSet<(u32, u32)>,
    dirty_pages: std::collections::BTreeSet<u32>,
    requested: std::collections::BTreeSet<(u32, u32)>,
    pub(crate) prior_endpoints: BTreeMap<(u32, u32), [Option<f64>; 4]>,
}
impl HostHooks for GlueHost {
    fn supports(&self, name: &str) -> bool {
        matches!(name, "_WALKGLUE" | "_XFTRIGGER")
    }
    fn hook_at(&mut self, page: Option<u32>, shape: Option<u32>, cell: &str, name: &str, _args: &[Value], cached: &Value) -> Value {
        let (Some(page), Some(shape)) = (page, shape) else { return cached.clone() };
        let key = (page, shape, cell.to_ascii_lowercase());
        match name {
            "_WALKGLUE" => {
                self.points.get(&key).filter(|_| self.active.contains(&(page, shape))).map_or_else(|| cached.clone(), |p| Value::Point(p.x, p.y))
            }
            "_XFTRIGGER" => self.triggers.get(&key).map_or_else(|| cached.clone(), |v| Value::Number(*v)),
            _ => cached.clone(),
        }
    }
}
impl GlueHost {
    /// Page IDs, connector IDs and capacity outcomes of this routing pass.
    pub fn outcomes(&self) -> &BTreeMap<(u32, u32), Option<String>> {
        &self.outcomes
    }

    pub(crate) fn page_dirty(&self, page: u32) -> bool {
        self.dirty_pages.contains(&page)
    }
    pub(crate) fn requested(&self, page: u32, shape: u32) -> bool {
        self.requested.contains(&(page, shape))
    }
    pub(crate) fn requests(&self) -> &std::collections::BTreeSet<(u32, u32)> {
        &self.requested
    }
    /// Compare transforms against the transaction snapshot and invalidate only affected glue triggers.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/confixedcode-cell-shape-layout-section
    /// Endpoint caches are not filled here: evaluation must call `_WALKGLUE` to obtain the new point.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/eventxfmod-cell-events-section
    pub fn prepare(before: &Document, doc: &mut Document) -> Result<Self, String> {
        Self::prepare_scoped(before, doc, false)
    }
    /// A validated predecessor permits exclusion of edits proved independent of
    /// routing. Imported or restored documents use the full validation path.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/eventxfmod-cell-events-section
    pub fn prepare_scoped(before: &Document, doc: &mut Document, validated: bool) -> Result<Self, String> {
        Self::prepare_inner(before, doc, validated, None)
    }
    pub(crate) fn force(&mut self, doc: &mut Document, force: (usize, Option<u32>)) -> Result<(), String> {
        // Preparation may already have advanced glue triggers for this edit.
        // Compare the forced pass to that prepared state, so it cannot advance
        // the same target change twice or discard reasons on another page.
        let prepared = doc.clone();
        let extra = Self::prepare_inner(&prepared, doc, false, Some(force))?;
        self.unaffected &= extra.unaffected;
        self.points.extend(extra.points);
        self.triggers.extend(extra.triggers);
        self.active.extend(extra.active);
        self.dirty_pages.extend(extra.dirty_pages);
        self.requested.extend(extra.requested);
        self.outcomes.extend(extra.outcomes);
        Ok(())
    }
    fn prepare_inner(before: &Document, doc: &mut Document, validated: bool, force: Option<(usize, Option<u32>)>) -> Result<Self, String> {
        if force.is_none() && validated && crate::scope::unaffected(before, doc) {
            return Ok(Self { unaffected: true, ..Self::default() });
        }
        let themes = misterdiagram_theme::Themes::from_document(doc).map_err(|e| e.to_string())?;
        let old_themes = misterdiagram_theme::Themes::from_document(before).map_err(|e| e.to_string())?;
        let text = misterdiagram_text::Host;
        let mut e = Evaluator::new(doc).map_err(|e| e.to_string())?.with_theme_host(&themes).with_text_host(&text);
        let mut old = Evaluator::new(before).map_err(|e| e.to_string())?.with_theme_host(&old_themes).with_text_host(&text);
        let mut host = Self { unaffected: crate::scope::empty_routes(doc), ..Self::default() };
        let mut current_bounds = TargetBounds::default();
        let mut prior_bounds = TargetBounds::default();
        let mut obstacles_memo = crate::orthogonal::ObstacleBounds::default();
        let mut changes = vec![];
        let mut affected = std::collections::BTreeSet::new();
        let mut count = 0usize;
        let mut scope_work = 0usize;
        for (page_index, page) in doc.pages.iter().enumerate() {
            let layout_changed = crate::orthogonal::page_signature(&mut e, page.id)? != crate::orthogonal::page_signature(&mut old, page.id)?;
            if layout_changed {
                host.dirty_pages.insert(page.id);
            }
            let placements = crate::changes::PageChanges::collect(&mut e, &mut old, page.id)?;
            let previous_page = before.pages.iter().find(|p| p.id == page.id);
            fn topology(links: &[misterdiagram_model::Connect]) -> Result<BTreeMap<u32, Vec<&misterdiagram_model::Connect>>, String> {
                if links.len() > 100000 {
                    return Err("glue connection count limit".into());
                }
                let mut by_shape = BTreeMap::<u32, Vec<_>>::new();
                for link in links {
                    by_shape.entry(link.from_sheet).or_default().push(link);
                }
                Ok(by_shape)
            }
            let prior_links = topology(previous_page.map_or(&[], |p| p.connects.as_slice()))?;
            let current_links = topology(&page.connects)?;
            for scope in e.scopes().filter(|s| matches!(s, Scope::PageShape {page:p,..} if *p==page.id)).collect::<Vec<_>>() {
                let Scope::PageShape { shape: id, .. } = scope else { continue };
                if e.evaluate(scope, "ObjType").numeric() != Ok(2.) {
                    continue;
                }
                let current = e.shape(scope).ok_or("route shape missing")?;
                let endpoint_changed = old.shape(scope).is_none_or(|previous| {
                    ["BeginX", "BeginY", "EndX", "EndY", "BegTrigger", "EndTrigger"].iter().any(|n| previous.sheet.cell(n) != current.sheet.cell(n))
                });
                let topology_changed = prior_links.get(&id) != current_links.get(&id);
                let mut prior = [None; 4];
                let endpoint_changed = endpoint_changed
                    || ["BeginX", "BeginY", "EndX", "EndY"].iter().enumerate().any(|(i, n)| {
                        let current = e.evaluate(scope, n);
                        let previous = old.evaluate(scope, n);
                        if let Value::Number(value) = &previous
                            && value.is_finite()
                            && value.abs() <= 1e6
                            && let Some(slot) = prior.get_mut(i)
                        {
                            *slot = Some(*value);
                        }
                        current != previous
                    });
                // Retain only bounded numeric results from the unchanged, pure
                // predecessor. Context hooks or SETF invalidate this memo below.
                // Cache saturation falls back to ordinary endpoint evaluation.
                if force.is_none() && host.prior_endpoints.len() < 4096 && prior.iter().any(Option::is_some) {
                    host.prior_endpoints.insert((page.id, id), prior);
                }
                let forced = force.is_some_and(|(index, shape)| index == page_index && shape.is_none_or(|s| s == id));
                let crossover = !endpoint_changed
                    && !layout_changed
                    && !forced
                    && number(&mut e, scope, "ConFixedCode")? == 0.
                    && old.shape(scope).is_some()
                    && placements.crosses(&mut old, scope, &mut scope_work)?;
                if forced || layout_changed || topology_changed || endpoint_changed || crossover {
                    host.requested.insert((page.id, id));
                }
            }
            let mut links = BTreeMap::<u32, [Option<&misterdiagram_model::Connect>; 2]>::new();
            for c in &page.connects {
                count += 1;
                if count > 100000 {
                    return Err("glue connection count limit".into());
                }
                let side = match c.from_cell.as_deref() {
                    Some("BeginX") => 0,
                    Some("EndX") => 1,
                    _ => continue,
                };
                let slot = links.entry(c.from_sheet).or_default().get_mut(side).ok_or("endpoint index")?;
                if slot.is_some() {
                    return Err("ambiguous endpoint glue".into());
                }
                *slot = Some(c);
            }
            for (id, links) in links {
                let scope = Scope::PageShape { page: page.id, shape: id };
                let dynamic = |c: Option<&misterdiagram_model::Connect>| c.is_some_and(|c| matches!(c.to_cell.as_deref(), Some("PinX" | "PinY")));
                let boxes = links
                    .map(|c| c.map(|c| current_bounds.get(&mut e, page.id, c.to_sheet)).transpose())
                    .into_iter()
                    .collect::<Result<Vec<_>, _>>()?;
                let preference = number(&mut e, scope, "WalkPreference")?;
                let fixed = number(&mut e, scope, "ConFixedCode")? == 2.;
                let routable = number(&mut e, scope, "ObjType")? == 2.;
                let parent = e.parent_to_page(scope).map_err(|e| e.to_string())?;
                if !parent.determinant().is_finite() || parent.determinant().abs() < 1e-15 {
                    return Err("singular connector parent transform".into());
                }
                let mut inputs_changed =
                    old.parent_to_page(scope).ok() != Some(parent) || number(&mut old, scope, "WalkPreference").ok() != Some(preference);
                for (side, link) in links.iter().enumerate() {
                    if link.is_none() {
                        let prefix = if side == 0 { "Begin" } else { "End" };
                        for axis in ["X", "Y"] {
                            let name = format!("{prefix}{axis}");
                            inputs_changed |= number(&mut old, scope, &name).ok() != number(&mut e, scope, &name).ok();
                        }
                    }
                }
                if inputs_changed || host.requested(page.id, id) {
                    affected.insert((page_index, id));
                }
                let mut target_changed = false;
                for (side, link) in links.iter().enumerate() {
                    let Some(link) = link else { continue };
                    let target = boxes.get(side).and_then(Option::as_deref).ok_or("glue target missing")?;
                    target_changed |= prior_bounds.get(&mut old, page.id, link.to_sheet).ok().is_none_or(|o| {
                        o.transform != target.transform
                            || o.width != target.width
                            || o.height != target.height
                            || o.points != target.points
                            || o.ink != target.ink
                    }) || !previous_page.is_some_and(|p| p.connects.contains(link));
                }
                if inputs_changed || target_changed {
                    host.requested.insert((page.id, id));
                }
                if !host.requested(page.id, id) && !layout_changed {
                    continue;
                }
                affected.insert((page_index, id));
                let planned = if !fixed
                    && routable
                    && preference == 0.
                    && links.iter().all(|c| dynamic(*c))
                    && boxes.iter().flatten().all(|b| b.points.is_empty())
                {
                    if let (Some(a), Some(b)) = (boxes.first().and_then(Option::as_deref), boxes.get(1).and_then(Option::as_deref)) {
                        let excluded = links.iter().flatten().map(|c| c.to_sheet).collect::<Vec<_>>();
                        let style = crate::orthogonal::style(&mut e, scope, page.id);
                        let obstacles =
                            if style == 2. || style == 16. { Ok(Vec::new()) } else { crate::orthogonal::obstacles(&mut e, page.id, &excluded) };
                        let clearance = (
                            crate::orthogonal::number(&mut e, Scope::Page(page.id), "LineToLineX", 0.125) * 1.5,
                            crate::orthogonal::number(&mut e, Scope::Page(page.id), "LineToLineY", 0.125) * 1.5,
                        );
                        let a = misterdiagram_geom::Rect::new(a.min.x, a.min.y, a.max.x, a.max.y);
                        let b = misterdiagram_geom::Rect::new(b.min.x, b.min.y, b.max.x, b.max.y);
                        let plan = match obstacles {
                            Ok(obstacles) => crate::orthogonal::endpoints(a, b, &obstacles, style, clearance).map(Vec::from),
                            Err(error) if error == "route obstacle count limit" => crate::orthogonal::with_obstacles(
                                &mut e,
                                page.id,
                                &excluded,
                                Affine::IDENTITY,
                                [a, b],
                                &mut obstacles_memo,
                                |obstacles, axes| crate::orthogonal::endpoint_footprint(a, b, obstacles, style, clearance, axes),
                            ),
                            Err(error) => Err(error),
                        };
                        Some(match plan {
                            Ok(points) => [*points.first().ok_or("endpoint plan length")?, *points.get(1).ok_or("endpoint plan length")?],
                            Err(error) if crate::orthogonal::capacity(&error) => {
                                host.outcomes.insert((page.id, id), Some(error));
                                crate::orthogonal::endpoints(a, b, &[], style, clearance)?
                            }
                            Err(error) => return Err(error),
                        })
                    } else {
                        None
                    }
                } else {
                    None
                };
                for (side, link) in links.iter().enumerate() {
                    let Some(link) = link else { continue };
                    let end = if side == 0 { End::Begin } else { End::End };
                    let target = boxes.get(side).and_then(Option::as_deref).ok_or("glue target missing")?;
                    let old_target = prior_bounds.get(&mut old, page.id, link.to_sheet).ok();
                    let changed = inputs_changed
                        || old_target.as_ref().is_none_or(|o| {
                            o.transform != target.transform
                                || o.width != target.width
                                || o.height != target.height
                                || o.points != target.points
                                || o.ink != target.ink
                        })
                        || !before.pages.iter().find(|p| p.id == page.id).is_some_and(|p| p.connects.contains(link));
                    let prior = e.evaluate(scope, end.trigger()).numeric().map_err(|e| e.to_string())?;
                    let counter = if changed { prior + 1. } else { prior };
                    if !counter.is_finite() || counter > 9007199254740991. {
                        return Err("glue trigger counter limit".into());
                    }
                    host.triggers.insert((page.id, id, end.trigger().to_ascii_lowercase()), counter);
                    if changed {
                        changes.push((page_index, id, end.trigger(), counter));
                        affected.insert((page_index, id));
                    }
                    if dynamic(Some(link)) {
                        let opposite = boxes.get(1usize.saturating_sub(side)).and_then(Option::as_deref);
                        let fallback;
                        let other = if let Some(other) = opposite {
                            other
                        } else {
                            let prefix = if side == 0 { "End" } else { "Begin" };
                            let x = number(&mut e, scope, &format!("{prefix}X"))?;
                            let y = number(&mut e, scope, &format!("{prefix}Y"))?;
                            let point = parent * Point::new(x, y);
                            fallback =
                                BoxShape { transform: Affine::IDENTITY, width: 0., height: 0., min: point, max: point, points: vec![], ink: None };
                            &fallback
                        };
                        let vertical = link.to_cell.as_deref() == Some("PinY")
                            || if side == 0 { preference == 1. || preference == 3. } else { preference == 2. || preference == 3. };
                        let point = if !fixed
                            && routable
                            && matches!(crate::orthogonal::style(&mut e, scope, page.id), 0. | 1.)
                            && preference == 0.
                            && target.points.is_empty()
                            && links.get(1usize.saturating_sub(side)).is_some_and(|c| c.is_some() && !dynamic(*c))
                        {
                            let prefix = if side == 0 { "End" } else { "Begin" };
                            let start =
                                parent * Point::new(number(&mut e, scope, &format!("{prefix}X"))?, number(&mut e, scope, &format!("{prefix}Y"))?);
                            let excluded = links.iter().flatten().map(|c| c.to_sheet).collect::<Vec<_>>();
                            let obstacles = crate::orthogonal::obstacles(&mut e, page.id, &excluded);
                            let clearance = (
                                crate::orthogonal::number(&mut e, Scope::Page(page.id), "LineToLineX", 0.125) * 1.5,
                                crate::orthogonal::number(&mut e, Scope::Page(page.id), "LineToLineY", 0.125) * 1.5,
                            );
                            let preferred = if old_target
                                .as_ref()
                                .is_some_and(|o| o.transform == target.transform && o.width == target.width && o.height == target.height)
                                && before.pages.iter().find(|p| p.id == page.id).is_some_and(|p| p.connects.contains(link))
                            {
                                let prefix = if side == 0 { "Begin" } else { "End" };
                                Some(
                                    parent * Point::new(number(&mut e, scope, &format!("{prefix}X"))?, number(&mut e, scope, &format!("{prefix}Y"))?),
                                )
                            } else {
                                None
                            };
                            let boxes = [
                                misterdiagram_geom::Rect::new(other.min.x, other.min.y, other.max.x, other.max.y),
                                misterdiagram_geom::Rect::new(target.min.x, target.min.y, target.max.x, target.max.y),
                            ];
                            let plan = obstacles
                                .and_then(|obstacles| crate::mixed_glue::endpoint(&mut e, scope, start, boxes, &obstacles, clearance, preferred));
                            match plan {
                                Ok(point) => point,
                                Err(error) if crate::orthogonal::capacity(&error) => {
                                    host.outcomes.insert((page.id, id), Some(error));
                                    endpoint(target, other, vertical)?
                                }
                                Err(error) => return Err(error),
                            }
                        } else if let Some(point) = planned.as_ref().and_then(|p| p.get(side)) {
                            *point
                        } else {
                            endpoint(target, other, vertical)?
                        };
                        let point = if !routable && target.points.is_empty() {
                            let c = center(target);
                            let dx = (point.x - target.min.x).abs().min((point.x - target.max.x).abs());
                            let dy = (point.y - target.min.y).abs().min((point.y - target.max.y).abs());
                            if dx < dy { Point::new(point.x, c.y) } else { Point::new(c.x, point.y) }
                        } else {
                            point
                        };
                        let point = target.ink.as_ref().map_or(point, |ink| {
                            crate::foreign_glue::project(
                                point,
                                misterdiagram_geom::Rect::new(target.min.x, target.min.y, target.max.x, target.max.y),
                                ink,
                            )
                        });
                        let point = parent.inverse() * point;
                        for axis in ["x", "y"] {
                            host.points.insert((page.id, id, format!("{}{axis}", end.prefix().to_ascii_lowercase())), point);
                        }
                    }
                }
            }
        }
        if old.evaluation_epoch() != (0, 0) {
            host.prior_endpoints.clear();
        }
        drop(e);
        let active: std::collections::BTreeSet<_> =
            affected.iter().filter_map(|(index, id)| doc.pages.get(*index).map(|page| (page.id, *id))).collect();
        host.active = active;
        for (page, id, name, value) in changes {
            let page = doc.pages.get_mut(page).ok_or("glue page missing")?;
            let shape = shape_mut(&mut page.shapes, id, 0).ok_or("glue shape missing")?;
            let formula = shape.sheet.cell(name).and_then(|c| c.formula.clone());
            set(&mut shape.sheet, cell(name, value, formula))?;
        }
        let mut centers = Vec::new();
        if !affected.is_empty() && force.is_none() {
            let mut e = Evaluator::new(doc).map_err(|e| e.to_string())?.with_theme_host(&themes).with_text_host(&text).with_glue_host(&mut host);
            for (index, id) in affected {
                let page = doc.pages.get(index).ok_or("glue page missing")?;
                let scope = Scope::PageShape { page: page.id, shape: id };
                if e.resolved_named_cells(scope).iter().any(|n| n.name == "Controls.TextPosition") {
                    centers.push((index, id, number(&mut e, scope, "Width")? / 2., number(&mut e, scope, "Height")? / 2.));
                }
            }
        }
        for (index, id, x, y) in centers {
            let page = doc.pages.get_mut(index).ok_or("glue page missing")?;
            let shape = shape_mut(&mut page.shapes, id, 0).ok_or("glue shape missing")?;
            if !shape.sheet.sections.iter().any(|s| s.name == "Control" && !s.deleted) {
                shape.sheet.sections.push(misterdiagram_model::Section { name: "Control".into(), ..Default::default() });
            }
            let section = shape.sheet.sections.iter_mut().find(|s| s.name == "Control" && !s.deleted).ok_or("control section missing")?;
            if !section.rows.iter().any(|r| r.name.as_deref() == Some("TextPosition") && !r.deleted) {
                section.rows.push(misterdiagram_model::Row { name: Some("TextPosition".into()), ..Default::default() });
            }
            let row = section.rows.iter_mut().find(|r| r.name.as_deref() == Some("TextPosition") && !r.deleted).ok_or("text control row missing")?;
            for (name, value) in [("X", x), ("Y", y), ("XDyn", x), ("YDyn", y)] {
                let c = cell(name, value, name.ends_with("Dyn").then(|| "Inh".into()));
                if let Some(old) = row.cells.iter_mut().find(|c| c.name == name) {
                    *old = c;
                } else {
                    row.cells.push(c);
                }
            }
        }
        Ok(host)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn predecessor_endpoint_memo_rejects_assignment_effects() {
        let mut d = misterdiagram_model::skeleton::document(false);
        let mut line = misterdiagram_model::skeleton::rectangle(1, 1., 1., 2., 1., "").unwrap();
        for (name, value) in [("ObjType", 2.), ("BeginX", 1.), ("BeginY", 1.), ("EndX", 3.), ("EndY", 1.)] {
            set(&mut line.sheet, cell(name, value, None)).unwrap();
        }
        d.pages[0].shapes.push(line);
        let pure = GlueHost::prepare(&d, &mut d.clone()).unwrap();
        assert!(pure.prior_endpoints.contains_key(&(0, 1)));
        set(&mut d.pages[0].shapes[0].sheet, cell("BeginX", 1., Some("SETF(GETREF(PinX),9)+1".into()))).unwrap();
        let assigned = GlueHost::prepare(&d, &mut d.clone()).unwrap();
        assert!(assigned.prior_endpoints.is_empty());
    }
    fn drawing() -> Document {
        let mut doc = misterdiagram_model::skeleton::document(false);
        let page = &mut doc.pages[0];
        page.shapes.push(misterdiagram_model::skeleton::rectangle(1, 1., 6., 1.5, 1.5, "").unwrap());
        page.shapes.push(misterdiagram_model::skeleton::rectangle(2, 5., 6., 1.5, 1.5, "").unwrap());
        misterdiagram_model::skeleton::glue_dynamic(page, 3, 2, [1, 2], [[2.5, 6.75], [5., 6.75]]).unwrap();
        doc
    }
    #[test]
    fn walkglue_is_computed_by_host_not_by_replacing_endpoint_cache() {
        let before = drawing();
        let mut after = before.clone();
        set(&mut after.pages[0].shapes[1].sheet, cell("PinX", 7.75, None)).unwrap();
        set(&mut after.pages[0].shapes[1].sheet, cell("PinY", 5.75, None)).unwrap();
        let mut host = GlueHost::prepare(&before, &mut after).unwrap();
        assert_eq!(after.pages[0].shapes[2].sheet.cell("BeginY").unwrap().value.as_deref(), Some("6.75"));
        assert_eq!(host.hook_at(Some(0), Some(3), "BeginY", "_WALKGLUE", &[], &Value::Number(6.75)), Value::Point(2.5, 6.25));
        assert_eq!(host.hook_at(Some(0), Some(3), "EndTrigger", "_XFTRIGGER", &[], &Value::Number(2.)), Value::Number(3.));
        assert_eq!(host.hook_at(Some(0), Some(3), "Other", "unknown", &[], &Value::Number(17.)), Value::Number(17.));
    }
    #[test]
    fn shared_target_bounds_do_not_hide_formula_assignments() {
        let mut doc = drawing();
        doc.pages[0].shapes[0].sheet.cells.push(cell("User.Assign", 0, Some("SETF(GETREF(Width),3)".into())));
        let mut evaluator = Evaluator::new(&doc).unwrap();
        let mut targets = TargetBounds::default();
        assert_eq!(targets.get(&mut evaluator, 0, 1).unwrap().width, 1.5);
        evaluator.evaluate(Scope::PageShape { page: 0, shape: 1 }, "User.Assign");
        assert_eq!(targets.get(&mut evaluator, 0, 1).unwrap().width, 3.);
    }
    #[test]
    fn hostile_target_coordinates_and_counter_overflow_return_errors() {
        for value in ["NaN", "inf", "1000001"] {
            let before = drawing();
            let mut after = before.clone();
            set(&mut after.pages[0].shapes[1].sheet, cell("PinX", value, None)).unwrap();
            assert!(GlueHost::prepare(&before, &mut after).is_err());
        }
        let before = drawing();
        let mut after = before.clone();
        set(&mut after.pages[0].shapes[1].sheet, cell("PinX", 7.75, None)).unwrap();
        set(&mut after.pages[0].shapes[2].sheet, cell("EndTrigger", "9007199254740991", Some("_XFTRIGGER(Sheet.2!EventXFMod)".into()))).unwrap();
        assert!(GlueHost::prepare(&before, &mut after).is_err());
    }
}
