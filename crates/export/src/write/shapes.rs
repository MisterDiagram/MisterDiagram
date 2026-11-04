//! Native shape trees retargeted from storytold/deckcraft
//! crates/pptx/src/write/shapes.rs, 90058395b78be26bb7ea7b4914a9f113c28d979d.
//! Copyright (c) 2026 ArtCraft Team and the DeckCraft contributors.
//! SPDX-License-Identifier: MIT OR Apache-2.0
use super::{content, dml, emu, text};
use crate::model::Page;
use kurbo::{Affine, Rect, Shape as _};
use misterdiagram_opc::xml::{A, W};
use misterdiagram_render::scene::{EvaluatedShape, Op};

pub(super) struct Pictures<'a> {
    pub media: &'a mut super::media::Media,
    pub package: &'a mut misterdiagram_opc::PackageOut,
    pub rels: &'a mut misterdiagram_opc::RelsOut,
    pub remaining: &'a mut usize,
}
pub(super) fn write(
    w: &mut W,
    page: &Page,
    view: Affine,
    warnings: &mut Vec<String>,
    limit: usize,
    pictures: &mut Pictures<'_>,
) -> Result<(), String> {
    let mut id = page.labels.next_id;
    for node in &page.shapes {
        shape(w, node, &page.connections, &page.labels, view, &mut id, warnings, limit, pictures, 0)?;
    }
    Ok(())
}
fn next(id: &mut u32) -> Result<u32, String> {
    let value = *id;
    if value > 200_000 {
        return Err("PPTX shape id limit".into());
    }
    *id = id.checked_add(1).ok_or("PPTX shape id limit")?;
    Ok(value)
}
fn group_bounds(node: &EvaluatedShape, connections: &crate::model::connectors::Connections, depth: usize) -> Result<Rect, String> {
    if depth > 128 {
        return Err("PPTX group depth limit".into());
    }
    let visible = !node.hidden_own || connections.attached.contains(&node.scope);
    let mut bounds = visible.then_some(node.bounds);
    if visible {
        for section in node.sections.iter().filter(|s| !s.flags.no_show) {
            let path = section.path.bounding_box();
            bounds = Some(bounds.map_or(path, |old| old.union(path)));
        }
    }
    if visible && let Some(block) = &node.text_block {
        let view = crate::write::page_flip(0.);
        let (carrier, _, _) = text::placement(block, view)?;
        let text = block.to_page.transform_rect_bbox(block.rect).union(view.transform_rect_bbox(carrier));
        bounds = Some(bounds.map_or(text, |old| old.union(text)));
    }
    for child in &node.children {
        let b = group_bounds(child, connections, depth + 1)?;
        bounds = Some(bounds.map_or(b, |old| old.union(b)));
    }
    Ok(bounds.unwrap_or(Rect::ZERO))
}
fn xfrm(w: &mut W, b: Rect, group: bool) -> Result<(), String> {
    let (x, y, width, height) = (emu(b.x0)?, emu(b.y0)?, emu(b.width())?.max(1), emu(b.height())?.max(1));
    w.open0("a:xfrm");
    w.empty("a:off", A::new().a("x", x).a("y", y));
    w.empty("a:ext", A::new().a("cx", width).a("cy", height));
    if group {
        w.empty("a:chOff", A::new().a("x", x).a("y", y));
        w.empty("a:chExt", A::new().a("cx", width).a("cy", height));
    }
    w.close("a:xfrm");
    Ok(())
}
/// Native group structure retains z-order while all geometry has baked transforms.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/displaymode-cell-group-properties-section
fn shape(
    w: &mut W,
    node: &EvaluatedShape,
    connections: &crate::model::connectors::Connections,
    labels: &crate::model::labels::Plan,
    view: Affine,
    id: &mut u32,
    warnings: &mut Vec<String>,
    limit: usize,
    pictures: &mut Pictures<'_>,
    depth: usize,
) -> Result<(), String> {
    if depth > 128 {
        return Err("PPTX group depth limit".into());
    }
    let elements = node.sections.iter().fold(0usize, |n, s| n.saturating_add(s.path.elements().len()));
    let sites = connections.sites.get(&node.scope).map_or(&[][..], Vec::as_slice);
    let estimate = elements
        .checked_mul(256)
        .and_then(|n| n.checked_add(sites.len().saturating_mul(512)))
        .and_then(|n| n.checked_add(8192))
        .ok_or("PPTX XML limit")?;
    if w.s.len().checked_add(estimate).is_none_or(|n| n > limit) {
        return Err("PPTX slide XML limit".into());
    }
    let carrier = node.one_d && connections.targets.get(&node.scope) != connections.ids.get(&node.scope);
    let label = labels.nodes.get(&node.scope).ok_or("PPTX label plan missing")?;
    if let Some(group_id) = label.group_id {
        w.open0("p:grpSp");
        w.open0("p:nvGrpSpPr");
        w.empty("p:cNvPr", A::new().a("id", group_id).a("name", format!("Group {}", node.id)));
        w.empty0("p:cNvGrpSpPr");
        w.empty0("p:nvPr");
        w.close("p:nvGrpSpPr");
        w.open0("p:grpSpPr");
        xfrm(w, view.transform_rect_bbox(group_bounds(node, connections, 0)?), true)?;
        w.close("p:grpSpPr");
    }
    if node.children_first {
        for child in &node.children {
            shape(w, child, connections, labels, view, id, warnings, limit, pictures, depth + 1)?;
        }
    }
    if !node.hidden_own || connections.attached.contains(&node.scope) {
        // Keep one native shape per evaluated shape, with independent section flags.
        let mut bounds = None;
        for section in node.sections.iter().filter(|s| !s.flags.no_show) {
            let b = (view * &section.path).bounding_box();
            bounds = Some(bounds.map_or(b, |old: Rect| old.union(b)));
        }
        let bounds = bounds.unwrap_or_else(|| view.transform_rect_bbox(node.bounds));
        let text = node.text_body.as_ref().zip(node.text_block.as_ref());
        let placement = text.map(|(_, block)| text::placement(block, view)).transpose()?;
        let inline = label.inline;
        if let Some(route) = connections.routes.get(&node.scope) {
            super::connectors::write(w, node, route, connections, view)?;
            if carrier {
                let carrier_id = connections.targets.get(&node.scope).ok_or("PPTX carrier id")?;
                w.open0("p:sp");
                w.open0("p:nvSpPr");
                w.empty("p:cNvPr", A::new().a("id", carrier_id).a("name", format!("Connection sites {}", node.id)));
                w.empty0("p:cNvSpPr");
                w.empty0("p:nvPr");
                w.close("p:nvSpPr");
                w.open0("p:spPr");
                let bounds = view.transform_rect_bbox(*connections.boxes.get(&node.scope).ok_or("PPTX carrier box")?);
                xfrm(w, bounds, false)?;
                content::geometry_sites(w, &[], view, bounds, sites)?;
                w.empty0("a:noFill");
                w.open0("a:ln");
                w.empty0("a:noFill");
                w.close("a:ln");
                w.close("p:spPr");
                w.close("p:sp");
            }
        } else {
            let bounds = if node.hidden_own { view.transform_rect_bbox(*connections.boxes.get(&node.scope).ok_or("PPTX site box")?) } else { bounds };
            w.open0("p:sp");
            w.open0("p:nvSpPr");
            w.empty(
                "p:cNvPr",
                A::new().a("id", connections.ids.get(&node.scope).ok_or("PPTX shape id missing")?).a("name", format!("Shape {}", node.id)),
            );
            w.empty0("p:cNvSpPr");
            w.empty0("p:nvPr");
            w.close("p:nvSpPr");
            w.open0("p:spPr");
            xfrm(w, bounds, false)?;
            content::geometry_sites(w, &node.sections, view, bounds, sites)?;
            dml::fill(w, node.paint.as_ref(), view, bounds)?;
            dml::line(w, node, view.determinant().abs().sqrt())?;
            w.close("p:spPr");
            if inline && let (Some((body, _)), Some((_, angle, scale))) = (text, placement) {
                text::body(w, body, scale, angle, limit)?;
            }
            w.close("p:sp");
        }
        if let Some(line) = &node.line
            && [line.begin_arrow, line.end_arrow].into_iter().any(|i| i != 0 && dml::arrow(i).1)
        {
            content::warn(warnings, "PPTX: arrowhead mapped to its nearest DrawingML shape");
        }
        let mut clip = None;
        let mut text_written = inline;
        for op in &node.drawing.ops {
            match op {
                Op::Clip { path, .. } => clip = Some(path),
                Op::Pop => clip = None,
                Op::Text { .. } => {
                    if !text_written && let (Some((body, _)), Some((bounds, angle, scale))) = (text, placement) {
                        text_shape(w, label.text_id.ok_or("PPTX text id missing")?, node.id, body, bounds, angle, scale, limit)?;
                        text_written = true;
                    }
                }
                Op::TextBackground { rect, transform, color } => {
                    content::path_shape(w, id, &(view * *transform * rect.to_path(1e-5)), *color, None, limit)?
                }
                Op::Image { pixels, transform } | Op::RasterImage { pixels, transform } => {
                    let rid = pictures.media.add(pixels, pictures.package, pictures.rels, pictures.remaining)?;
                    if clip.is_some_and(|p| w.s.len().saturating_add(p.elements().len().saturating_mul(256)).saturating_add(8192) > limit) {
                        return Err("PPTX picture geometry limit".into());
                    }
                    let local_clip = clip.map(|path| transform.inverse() * path);
                    picture(w, id, pixels.width(), pixels.height(), view * *transform, &rid, local_clip.as_ref())?;
                }
                _ => {}
            }
        }
        // Retained fallback text has no Op::Text when the raster layout failed.
        if !text_written && let (Some((body, _)), Some((bounds, angle, scale))) = (text, placement) {
            text_shape(w, label.text_id.ok_or("PPTX text id missing")?, node.id, body, bounds, angle, scale, limit)?;
        }
    }
    if !node.children_first {
        for child in &node.children {
            shape(w, child, connections, labels, view, id, warnings, limit, pictures, depth + 1)?;
        }
    }
    if label.group_id.is_some() {
        w.close("p:grpSp");
    }
    Ok(())
}

/// ECMA-376 Part 1, 19.3.1.37 (pic). Foreign internal content is painted
/// into transparent pixels. The outer frame remains
/// picture geometry, preserving the renderer's clip/composite boundary.
/// ECMA-376 Part 1, 19.3.1.44 (spPr) uses CT_ShapeProperties, including custGeom.
fn picture(w: &mut W, id: &mut u32, width: u32, height: u32, transform: Affine, rid: &str, clip: Option<&kurbo::BezPath>) -> Result<(), String> {
    let [a, b, c, d, _, _] = transform.as_coeffs();
    let sx = a.hypot(b);
    let sy = c.hypot(d);
    if !sx.is_finite() || !sy.is_finite() || sx <= 0. || sy <= 0. || (a * c + b * d).abs() > 1e-8 * sx * sy {
        return Err("PPTX foreign image transform".into());
    }
    w.open0("p:pic");
    w.open0("p:nvPicPr");
    w.empty("p:cNvPr", A::new().a("id", next(id)?).a("name", "Foreign picture"));
    w.empty0("p:cNvPicPr");
    w.empty0("p:nvPr");
    w.close("p:nvPicPr");
    w.open0("p:blipFill");
    w.empty("a:blip", A::new().a("r:embed", rid));
    w.open0("a:stretch");
    w.empty0("a:fillRect");
    w.close("a:stretch");
    w.close("p:blipFill");
    w.open0("p:spPr");
    let extent = (sx * f64::from(width), sy * f64::from(height));
    let center = transform * kurbo::Point::new(f64::from(width) / 2., f64::from(height) / 2.);
    let angle = (b.atan2(a).to_degrees().rem_euclid(360.) * 60000.).round() as i64;
    w.open("a:xfrm", A::new().a("rot", angle).a("flipV", if transform.determinant() < 0. { 1 } else { 0 }));
    w.empty("a:off", A::new().a("x", emu(center.x - extent.0 / 2.)?).a("y", emu(center.y - extent.1 / 2.)?));
    w.empty("a:ext", A::new().a("cx", emu(extent.0)?.max(1)).a("cy", emu(extent.1)?.max(1)));
    w.close("a:xfrm");
    if let Some(clip) = clip {
        let section = misterdiagram_geom::GeometryPath {
            path: Affine::scale_non_uniform(sx, sy) * clip,
            flags: misterdiagram_geom::GeometryFlags { no_line: true, ..Default::default() },
        };
        content::geometry(w, &[section], Affine::IDENTITY, Rect::new(0., 0., extent.0, extent.1))?;
    } else {
        w.open("a:prstGeom", A::new().a("prst", "rect"));
        w.empty0("a:avLst");
        w.close("a:prstGeom");
    }
    w.empty0("a:noFill");
    w.open0("a:ln");
    w.empty0("a:noFill");
    w.close("a:ln");
    w.close("p:spPr");
    w.close("p:pic");
    Ok(())
}

/// A distinct saved text rectangle stays editable without changing its size to
/// compensate for PowerPoint's independent line-breaking engine.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/txtpinx-cell-text-transform-section
fn text_shape(
    w: &mut W,
    id: u32,
    source: u32,
    body: &misterdiagram_model::text::TextBody,
    bounds: Rect,
    angle: f64,
    scale: f64,
    limit: usize,
) -> Result<(), String> {
    if w.s.len().saturating_add(8192) > limit {
        return Err("PPTX text shape limit".into());
    }
    w.open0("p:sp");
    w.open0("p:nvSpPr");
    w.empty("p:cNvPr", A::new().a("id", id).a("name", format!("Text {source}")));
    w.empty("p:cNvSpPr", A::new().a("txBox", 1));
    w.empty0("p:nvPr");
    w.close("p:nvSpPr");
    w.open0("p:spPr");
    xfrm(w, bounds, false)?;
    content::geometry(
        w,
        &[misterdiagram_geom::GeometryPath {
            path: bounds.to_path(1e-5),
            flags: misterdiagram_geom::GeometryFlags { no_fill: true, no_line: true, ..Default::default() },
        }],
        Affine::IDENTITY,
        bounds,
    )?;
    w.empty0("a:noFill");
    w.open0("a:ln");
    w.empty0("a:noFill");
    w.close("a:ln");
    w.close("p:spPr");
    text::body(w, body, scale, angle, limit)?;
    w.close("p:sp");
    Ok(())
}
