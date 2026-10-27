//! Crossing decorations are serialized as Visio Geometry rows, not renderer effects.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/linejumpstyle-cell-page-layout-section
use crate::{
    GlueHost,
    orthogonal::number,
    static_glue::{cell, shape_mut},
};
use misterdiagram_geom::{Affine, Point};
use misterdiagram_model::{Document, Row, Section};
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
use std::collections::{BTreeMap, BTreeSet};

struct Connector {
    page: usize,
    id: u32,
    points: Vec<Point>,
    transform: Affine,
    width: f64,
    height: f64,
    code: f64,
    page_code: f64,
    style: f64,
    factors: [f64; 2],
    directions: [f64; 2],
    order: usize,
    had_jumps: bool,
    inherited_zero: [[bool; 2]; 2],
}
fn row(index: u32, kind: &str, p: Point, a: Option<misterdiagram_model::Cell>) -> Row {
    let mut cells = vec![cell("X", p.x, None), cell("Y", p.y, None)];
    if let Some(a) = a {
        cells.push(a);
    }
    Row {
        index: Some(index),
        row_type: Some(kind.into()),
        attributes: misterdiagram_model::Attributes::from([("IX".into(), index.to_string()), ("T".into(), kind.into())]),
        cells,
        ..Default::default()
    }
}
/// https://learn.microsoft.com/en-us/office/client-developer/visio/arcto-row-geometry-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/polylineto-row-geometry-section
/// Rebuild only crossing decorations, preserving the underlying orthogonal segments.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/conlinejumpcode-cell-shape-layout-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/conlinejumpstyle-cell-shape-layout-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/linejumpfactorx-cell-page-layout-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/linejumpfactory-cell-page-layout-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/conlinejumpdirx-cell-shape-layout-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/conlinejumpdiry-cell-shape-layout-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pagelinejumpdirx-cell-page-layout-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pagelinejumpdiry-cell-page-layout-section
pub(crate) fn apply(
    before: &Document,
    doc: &mut Document,
    host: &mut GlueHost,
    routed: &BTreeSet<(usize, u32)>,
    touched: &BTreeSet<(usize, u32)>,
) -> Result<(), String> {
    if touched.is_empty() {
        return Ok(());
    }
    let pages = touched.iter().filter_map(|(index, _)| doc.pages.get(*index).map(|p| (*index, p.id))).collect::<BTreeMap<_, _>>();
    let connectors = collect(doc, Some(host), &pages)?;
    // A page-wide request has no unselected crossing neighbor to compare.
    if connectors.iter().all(|c| touched.contains(&(c.page, c.id))) {
        return decorate(doc, &connectors, routed);
    }
    let previous = collect(before, None, &pages)?;
    let mut recipients = routed.clone();
    for c in &connectors {
        if touched.contains(&(c.page, c.id)) {
            continue;
        }
        let old = previous.iter().find(|o| o.page == c.page && o.id == c.id);
        let old_crossings = old.map(|c| crossing_signature(c, &previous, touched)).unwrap_or_default();
        if old_crossings != crossing_signature(c, &connectors, touched) {
            recipients.insert((c.page, c.id));
        }
    }
    decorate(doc, &connectors, &recipients)
}
fn collect(doc: &Document, host: Option<&mut GlueHost>, pages: &BTreeMap<usize, u32>) -> Result<Vec<Connector>, String> {
    let themes = misterdiagram_theme::Themes::from_document(doc).map_err(|e| e.to_string())?;
    let text = misterdiagram_text::Host;
    let mut e = Evaluator::new(doc).map_err(|v| v.to_string())?.with_theme_host(&themes).with_text_host(&text);
    if let Some(host) = host {
        e = e.with_glue_host(host);
    }
    let mut connectors = Vec::new();
    let mut segments = 0usize;
    for (index, page_id) in pages {
        let Some(page) = doc.pages.iter().find(|p| p.id == *page_id) else { continue };
        let ps = Scope::Page(page.id);
        let ids = e
            .scopes()
            .filter_map(|s| match s {
                Scope::PageShape { page: p, shape } if p == page.id => Some(shape),
                _ => None,
            })
            .collect::<BTreeSet<_>>();
        for id in ids {
            let scope = Scope::PageShape { page: page.id, shape: id };
            if number(&mut e, scope, "ObjType", 0.) != 2. {
                continue;
            }
            let transform = e.shape_to_page(scope).map_err(|v| v.to_string())?;
            if !transform.determinant().is_finite() || transform.determinant().abs() < 1e-15 {
                return Err("jump transform limit".into());
            }
            let mut rows = BTreeMap::new();
            for n in e.resolved_named_cells(scope) {
                if n.section.is_some_and(|s| s.name == "Geometry" && s.index.unwrap_or(0) == 0)
                    && n.cell.name == "X"
                    && let Some(r) = n.row.filter(|r| !r.deleted)
                {
                    rows.insert(r.index.unwrap_or(0), r.row_type.clone().unwrap_or_else(|| "LineTo".into()));
                }
            }
            if rows.len() > 16384 {
                return Err("jump row limit".into());
            }
            let mut points = Vec::new();
            let mut had_jumps = false;
            for (ix, kind) in rows {
                if kind == "ArcTo" || kind == "PolylineTo" || kind == "MoveTo" && !points.is_empty() {
                    had_jumps = true;
                    points.pop();
                    continue;
                }
                let p = Point::new(number(&mut e, scope, &format!("Geometry1.X{ix}"), 0.), number(&mut e, scope, &format!("Geometry1.Y{ix}"), 0.));
                let p = transform * p;
                if !p.x.is_finite() || !p.y.is_finite() || p.x.abs() > 1e6 || p.y.abs() > 1e6 {
                    return Err("jump coordinate limit".into());
                }
                points.push(p);
            }
            segments = segments.checked_add(points.len()).ok_or("jump segment count")?;
            if segments > 2000 {
                return Err("jump intersection work limit".into());
            }
            let style = number(&mut e, scope, "ConLineJumpStyle", 0.);
            let style = if style == 0. { number(&mut e, ps, "LineJumpStyle", 0.) } else { style };
            let factors = [
                number(&mut e, ps, "LineToLineX", 0.125) * number(&mut e, ps, "LineJumpFactorX", 2. / 3.) / 2.,
                number(&mut e, ps, "LineToLineY", 0.125) * number(&mut e, ps, "LineJumpFactorY", 2. / 3.) / 2.,
            ];
            if factors.iter().any(|v| !v.is_finite() || *v < 0. || *v > 1e4) {
                return Err("jump factor limit".into());
            }
            let directions = [("ConLineJumpDirX", "PageLineJumpDirX"), ("ConLineJumpDirY", "PageLineJumpDirY")].map(|(local, page)| {
                let v = number(&mut e, scope, local, 0.);
                if v == 0. { number(&mut e, ps, page, 0.) } else { v }
            });
            connectors.push(Connector {
                page: *index,
                id,
                points,
                transform,
                width: number(&mut e, scope, "Width", 1.),
                height: number(&mut e, scope, "Height", 1.),
                code: number(&mut e, scope, "ConLineJumpCode", 0.),
                page_code: number(&mut e, ps, "LineJumpCode", 0.),
                style,
                factors,
                directions,
                had_jumps,
                inherited_zero: crate::orthogonal::inherited_zero(&e, scope),
                order: page.shapes.iter().position(|s| s.id == id).unwrap_or(0),
            });
        }
    }
    Ok(connectors)
}
fn decorate(doc: &mut Document, connectors: &[Connector], routed: &BTreeSet<(usize, u32)>) -> Result<(), String> {
    let mut changes = Vec::new();
    for c in connectors {
        if !routed.contains(&(c.page, c.id)) {
            continue;
        }
        let Some(first) = c.points.first() else { continue };
        let inv = c.transform.inverse();
        let mut rows = vec![row(1, "MoveTo", inv * *first, None)];
        let mut decorated = false;
        for pair in c.points.windows(2) {
            let (Some(a), Some(b)) = (pair.first(), pair.get(1)) else { continue };
            let delta = *b - *a;
            let axis = if delta.y.abs() < 1e-9 {
                0
            } else if delta.x.abs() < 1e-9 {
                1
            } else {
                2
            };
            let mut crossings = Vec::new();
            if axis < 2 && c.code != 1. && c.code != 3. && c.code != 4. {
                for other in connectors.iter().filter(|o| o.page == c.page && o.id != c.id && o.code != 4.) {
                    let jumps = c.code == 2.
                        || other.code == 3.
                        || match c.page_code {
                            1. => axis == 0,
                            2. => axis == 1,
                            3. | 4. => c.order > other.order,
                            5. => c.order < other.order,
                            _ => false,
                        };
                    if !jumps {
                        continue;
                    }
                    for pair in other.points.windows(2) {
                        let (Some(u), Some(v)) = (pair.first(), pair.get(1)) else { continue };
                        let cross = if axis == 0 && (u.x - v.x).abs() < 1e-9 {
                            Some(Point::new(u.x, a.y))
                        } else if axis == 1 && (u.y - v.y).abs() < 1e-9 {
                            Some(Point::new(a.x, u.y))
                        } else {
                            None
                        };
                        if let Some(p) = cross {
                            let between = |p: Point, x: Point, y: Point| (p - x).dot(p - y) < -1e-12;
                            if between(p, *a, *b) && between(p, *u, *v) {
                                crossings.push((p - *a).hypot());
                            }
                        }
                    }
                }
            }
            crossings.sort_by(f64::total_cmp);
            crossings.dedup_by(|a, b| (*a - *b).abs() < 1e-9);
            let length = delta.hypot();
            let radius = c.factors.get(axis).copied().unwrap_or(0.);
            let unit = if length > 1e-12 { delta / length } else { misterdiagram_geom::Vec2::ZERO };
            let mut last = 0.;
            for at in crossings {
                if radius <= 0. || at - radius <= last + 1e-9 || at + radius >= length - 1e-9 {
                    continue;
                }
                decorated = true;
                let begin = inv * (*a + unit * (at - radius));
                let end = inv * (*a + unit * (at + radius));
                rows.push(row(u32::try_from(rows.len() + 1).map_err(|_| "jump row index")?, "LineTo", begin, None));
                let direction = if c.directions.get(axis).copied().unwrap_or(0.) == 2. { -1. } else { 1. };
                let (kind, extra) = match c.style {
                    2. => ("MoveTo", None),
                    3. | 4. => {
                        if c.width.abs() < 1e-12 || c.height.abs() < 1e-12 {
                            return Err("jump normalization extent".into());
                        }
                        let rise = if axis == 0 {
                            misterdiagram_geom::Vec2::new(0., radius * direction)
                        } else {
                            misterdiagram_geom::Vec2::new(radius * direction, 0.)
                        };
                        let controls = if c.style == 3. {
                            vec![inv * (*a + unit * (at - radius) + rise), inv * (*a + unit * (at + radius) + rise)]
                        } else {
                            vec![inv * (*a + unit * at + rise)]
                        };
                        let mut formula = "POLYLINE(0, 0".to_owned();
                        for p in controls {
                            formula.push_str(&format!(", {},{}", p.x / c.width, p.y / c.height));
                        }
                        formula.push(')');
                        let mut a = cell("A", &formula, Some(formula.clone()));
                        a.units = Some("POLYLINE".into());
                        ("PolylineTo", Some(a))
                    }
                    _ => {
                        let sign = if axis == 0 { unit.x.signum() } else { unit.y.signum() };
                        (
                            "ArcTo",
                            Some({
                                let mut a = cell("A", -radius * direction * sign, None);
                                a.units = Some("DL".into());
                                a
                            }),
                        )
                    }
                };
                rows.push(row(u32::try_from(rows.len() + 1).map_err(|_| "jump row index")?, kind, end, extra));
                last = at + radius;
            }
            rows.push(row(u32::try_from(rows.len() + 1).map_err(|_| "jump row index")?, "LineTo", inv * *b, None));
        }
        if decorated || c.had_jumps {
            crate::orthogonal::trim_inherited_start(&mut rows, c.inherited_zero);
            changes.push((c.page, c.id, rows));
        }
    }
    for (index, id, rows) in changes {
        let page = doc.pages.get_mut(index).ok_or("jump page missing")?;
        let shape = shape_mut(&mut page.shapes, id, 0).ok_or("jump connector missing")?;
        let section = Section { name: "Geometry".into(), index: Some(0), rows, ..Default::default() };
        if let Some(old) = shape.sheet.sections.iter_mut().find(|s| s.name == "Geometry" && s.index.unwrap_or(0) == 0) {
            old.rows = section.rows;
        } else {
            shape.sheet.sections.push(section);
        }
    }
    Ok(())
}

/// Compare only crossing relationships involving a changed route. The 2,000
/// retained-point cap bounds each old/new intersection product.
fn crossing_signature(c: &Connector, all: &[Connector], touched: &BTreeSet<(usize, u32)>) -> Vec<(u32, f64, f64)> {
    let mut out = Vec::new();
    for other in all.iter().filter(|o| o.page == c.page && o.id != c.id && touched.contains(&(o.page, o.id))) {
        for pair in c.points.windows(2) {
            let (Some(a), Some(b)) = (pair.first(), pair.get(1)) else { continue };
            for pair in other.points.windows(2) {
                let (Some(u), Some(v)) = (pair.first(), pair.get(1)) else { continue };
                let p = if (a.y - b.y).abs() < 1e-9 && (u.x - v.x).abs() < 1e-9 {
                    Some(Point::new(u.x, a.y))
                } else if (a.x - b.x).abs() < 1e-9 && (u.y - v.y).abs() < 1e-9 {
                    Some(Point::new(a.x, u.y))
                } else {
                    None
                };
                if let Some(p) = p
                    && (p - *a).dot(p - *b) < -1e-12
                    && (p - *u).dot(p - *v) < -1e-12
                {
                    out.push((other.id, p.x, p.y));
                }
            }
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.total_cmp(&b.1)).then(a.2.total_cmp(&b.2)));
    out.dedup();
    out
}
