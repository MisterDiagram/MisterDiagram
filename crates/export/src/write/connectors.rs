//! Connector element structure retargeted from storytold/deckcraft
//! crates/pptx/src/write/shapes.rs, 90058395b78be26bb7ea7b4914a9f113c28d979d.
//! Copyright (c) 2026 ArtCraft Team and the DeckCraft contributors.
//! SPDX-License-Identifier: MIT OR Apache-2.0
//! Preset connectors and explicit custom-geometry connection sites.
//! ECMA-376 Part 1, 19.3.1.19 (cxnSp), 20.1.9.9 (cxn).
use super::{dml, emu};
use crate::model::connectors::{Connections, Connector};
use kurbo::{Affine, Point, Rect};
use misterdiagram_opc::xml::{A, W};
use misterdiagram_render::scene::{ConnectionSite, EvaluatedShape};

/// https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.drawing.shapeguide.formula
pub(super) fn sites(w: &mut W, sites: &[ConnectionSite], view: Affine, b: Rect) -> Result<(), String> {
    let width = emu(b.width())?.max(1);
    let height = emu(b.height())?.max(1);
    let ox = emu(b.x0)?;
    let oy = emu(b.y0)?;
    w.open0("a:gdLst");
    for (i, s) in sites.iter().enumerate() {
        let p = view * s.position;
        let x = emu(p.x)?.checked_sub(ox).ok_or("PPTX site coordinate overflow")?;
        let y = emu(p.y)?.checked_sub(oy).ok_or("PPTX site coordinate overflow")?;
        w.empty("a:gd", A::new().a("name", format!("cx{i}")).a("fmla", format!("*/ {x} w {width}")));
        w.empty("a:gd", A::new().a("name", format!("cy{i}")).a("fmla", format!("*/ {y} h {height}")));
    }
    w.close("a:gdLst");
    w.empty0("a:ahLst");
    w.open0("a:cxnLst");
    for (i, s) in sites.iter().enumerate() {
        let normal = view * Point::new(s.normal.x, s.normal.y) - view * Point::ORIGIN;
        if !normal.x.is_finite() || !normal.y.is_finite() {
            return Err("PPTX connection direction limit".into());
        }
        let angle = if normal.hypot() < 1e-12 { 0 } else { (normal.atan2().to_degrees().rem_euclid(360.) * 60_000.).round() as i64 };
        w.open("a:cxn", A::new().a("ang", angle));
        w.empty("a:pos", A::new().a("x", format!("cx{i}")).a("y", format!("cy{i}")));
        w.close("a:cxn");
    }
    w.close("a:cxnLst");
    Ok(())
}
/// Endpoints are quantized in page coordinates before taking differences, so a
/// guide site and its connector endpoint use the identical EMU coordinate.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/beginx-cell-1-d-endpoints-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/endx-cell-1-d-endpoints-section
pub(super) fn write(w: &mut W, node: &EvaluatedShape, route: &Connector, map: &Connections, view: Affine) -> Result<(), String> {
    let id = map.ids.get(&node.scope).ok_or("PPTX connector id missing")?;
    w.open0("p:cxnSp");
    w.open0("p:nvCxnSpPr");
    w.empty("p:cNvPr", A::new().a("id", id).a("name", format!("Connector {}", node.id)));
    w.open0("p:cNvCxnSpPr");
    for (tag, attachment) in [("a:stCxn", &route.begin), ("a:endCxn", &route.end)] {
        if let Some(a) = attachment {
            let target = map.targets.get(&a.target).ok_or("PPTX target id missing")?;
            w.empty(tag, A::new().a("id", target).a("idx", a.index));
        }
    }
    w.close("p:cNvCxnSpPr");
    w.empty0("p:nvPr");
    w.close("p:nvCxnSpPr");
    w.open0("p:spPr");
    let points: Vec<_> = route.points.iter().map(|p| view * *p).collect();
    let begin = *points.first().ok_or("PPTX empty route")?;
    let end = *points.last().ok_or("PPTX empty route")?;
    let first = points.get(1).copied().unwrap_or(end) - begin;
    let vertical = points.len() > 2 && first.y.abs() > first.x.abs();
    let local = |p: Point| if vertical { Point::new(p.y, -p.x) } else { p };
    let a = local(begin);
    let b = local(end);
    let (ax, ay, bx, by) = (emu(a.x)?, emu(a.y)?, emu(b.x)?, emu(b.y)?);
    let width = (bx - ax).abs();
    let height = (by - ay).abs();
    let center = Point::new((begin.x + end.x) / 2., (begin.y + end.y) / 2.);
    let off = if vertical { (emu(center.x)? - width / 2, emu(center.y)? - height / 2) } else { (ax.min(bx), ay.min(by)) };
    w.open("a:xfrm", A::new().a("flipH", u8::from(bx < ax)).a("flipV", u8::from(by < ay)).a("rot", if vertical { 5_400_000 } else { 0 }));
    w.empty("a:off", A::new().a("x", off.0).a("y", off.1));
    w.empty("a:ext", A::new().a("cx", width).a("cy", height));
    w.close("a:xfrm");
    let count = points.len().saturating_sub(1).clamp(2, 5);
    let preset = if route.curved {
        format!("curvedConnector{count}")
    } else if points.len() == 2 {
        "straightConnector1".into()
    } else {
        format!("bentConnector{count}")
    };
    w.open("a:prstGeom", A::new().a("prst", preset));
    w.open0("a:avLst");
    for (i, p) in points.iter().skip(1).take(count.saturating_sub(2)).enumerate() {
        let p = local(*p);
        let (value, denominator) = if i % 2 == 0 { (p.x - a.x, b.x - a.x) } else { (p.y - a.y, b.y - a.y) };
        let value = if denominator.abs() < 1e-12 { 0. } else { value / denominator * 100_000. };
        if !value.is_finite() || value.abs() > f64::from(i32::MAX) {
            return Err("PPTX connector adjustment limit".into());
        }
        w.empty("a:gd", A::new().a("name", format!("adj{}", i + 1)).a("fmla", format!("val {}", value.round() as i64)));
    }
    w.close("a:avLst");
    w.close("a:prstGeom");
    w.empty0("a:noFill");
    if node.sections.iter().any(|s| s.stroked()) {
        dml::line(w, node, view.determinant().abs().sqrt())?;
    } else {
        w.open0("a:ln");
        w.empty0("a:noFill");
        w.close("a:ln");
    }
    w.close("p:spPr");
    w.close("p:cxnSp");
    Ok(())
}
