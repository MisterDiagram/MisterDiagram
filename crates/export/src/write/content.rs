//! Path serialization retargeted from storytold/deckcraft
//! crates/pptx/src/custgeom.rs, 90058395b78be26bb7ea7b4914a9f113c28d979d.
//! Copyright (c) 2026 ArtCraft Team and the DeckCraft contributors.
//! SPDX-License-Identifier: MIT OR Apache-2.0
use super::emu;
use kurbo::{Affine, BezPath, PathEl, Shape};
use misterdiagram_color::Rgba;
use misterdiagram_opc::xml::{A, W};
use misterdiagram_render::scene::{Op, Paint, Scene};

/// Initial path content for the package milestone. Native shape grouping, text
/// bodies, presets and media are separate exporter milestones.
pub(super) fn write(w: &mut W, scene: &Scene, view: Affine, warnings: &mut Vec<String>, limit: usize) -> Result<(), String> {
    let mut id = 2u32;
    for op in &scene.ops {
        match op {
            Op::Path { path, paint, .. } => {
                let color = match paint {
                    Paint::Solid(c) => *c,
                    Paint::Pattern { fill, .. } => {
                        warn(warnings, "PPTX package preview: pattern and gradient fills use their foreground colour");
                        fill.foreground
                    }
                };
                path_shape(w, &mut id, &(view * path), color, None, limit)?;
            }
            Op::Stroke { path, stroke, color } | Op::HairlineStroke { path, stroke, color, .. } => {
                path_shape(w, &mut id, &(view * path), *color, Some(stroke.width * view.determinant().abs().sqrt()), limit)?;
            }
            Op::HairlineMarker { path, color, .. } => path_shape(w, &mut id, &(view * path), *color, None, limit)?,
            Op::Text { layout, transform, .. } => {
                warn(warnings, "PPTX package preview: text uses glyph paths; editable runs are not implemented yet");
                for (path, color) in misterdiagram_render::scene::text_paths(layout, view * *transform) {
                    path_shape(w, &mut id, &path, color, None, limit)?;
                }
            }
            Op::TextBackground { rect, transform, color } => path_shape(w, &mut id, &(view * *transform * rect.to_path(1e-5)), *color, None, limit)?,
            Op::Image { .. } => warn(warnings, "PPTX package preview: images and raster shadows are not implemented yet"),
            Op::Clip { .. } => warn(warnings, "PPTX package preview: clipping is not implemented yet"),
            Op::Bounds(_) | Op::Pop => {}
        }
    }
    Ok(())
}
fn warn(warnings: &mut Vec<String>, message: &str) {
    if warnings.len() < 1000 && !warnings.iter().any(|s| s == message) {
        warnings.push(message.into());
    }
}
fn path_shape(w: &mut W, id: &mut u32, path: &BezPath, color: Rgba, line: Option<f64>, limit: usize) -> Result<(), String> {
    if path.elements().is_empty() {
        return Ok(());
    }
    if *id > 200_000 || path.elements().len() > 1_000_000 {
        return Err("PPTX path limit".into());
    }
    // ECMA-376 Part 1, 20.1.10.35: ST_LineWidth is 0..20,116,800 EMU.
    let line = line.map(emu).transpose()?;
    if line.is_some_and(|width| !(0..=20_116_800).contains(&width)) {
        return Err("PPTX line width limit".into());
    }
    let estimate = path.elements().len().checked_mul(256).and_then(|n| n.checked_add(2048)).ok_or("PPTX XML limit")?;
    if w.s.len().checked_add(estimate).is_none_or(|n| n > limit) {
        return Err("PPTX slide XML limit".into());
    }
    let bounds = path.bounding_box();
    let (x, y) = (emu(bounds.x0)?, emu(bounds.y0)?);
    let (width, height) = (emu(bounds.width())?.max(1), emu(bounds.height())?.max(1));
    w.open0("p:sp");
    w.open0("p:nvSpPr");
    w.empty("p:cNvPr", A::new().a("id", *id).a("name", format!("Evaluated path {id}")));
    *id = id.checked_add(1).ok_or("PPTX shape id limit")?;
    w.empty0("p:cNvSpPr");
    w.empty0("p:nvPr");
    w.close("p:nvSpPr");
    w.open0("p:spPr");
    w.open0("a:xfrm");
    w.empty("a:off", A::new().a("x", x).a("y", y));
    w.empty("a:ext", A::new().a("cx", width).a("cy", height));
    w.close("a:xfrm");
    w.open0("a:custGeom");
    for tag in ["a:avLst", "a:gdLst", "a:ahLst", "a:cxnLst"] {
        w.empty0(tag);
    }
    w.empty("a:rect", A::new().a("l", "l").a("t", "t").a("r", "r").a("b", "b"));
    w.open0("a:pathLst");
    w.open(
        "a:path",
        A::new().a("w", width).a("h", height).a("fill", if line.is_some() { "none" } else { "norm" }).a("stroke", u8::from(line.is_some())),
    );
    for element in path.elements() {
        let (tag, points): (&str, &[kurbo::Point]) = match element {
            PathEl::MoveTo(p) => ("a:moveTo", std::slice::from_ref(p)),
            PathEl::LineTo(p) => ("a:lnTo", std::slice::from_ref(p)),
            PathEl::QuadTo(a, b) => ("a:quadBezTo", &[*a, *b]),
            PathEl::CurveTo(a, b, c) => ("a:cubicBezTo", &[*a, *b, *c]),
            PathEl::ClosePath => {
                w.empty0("a:close");
                continue;
            }
        };
        w.open0(tag);
        for point in points {
            w.empty("a:pt", A::new().a("x", emu(point.x - bounds.x0)?).a("y", emu(point.y - bounds.y0)?));
        }
        w.close(tag);
    }
    w.close("a:path");
    w.close("a:pathLst");
    w.close("a:custGeom");
    if let Some(width) = line {
        w.empty0("a:noFill");
        w.open("a:ln", A::new().a("w", width));
        solid(w, color);
        w.close("a:ln");
    } else {
        solid(w, color);
        w.open0("a:ln");
        w.empty0("a:noFill");
        w.close("a:ln");
    }
    w.close("p:spPr");
    w.close("p:sp");
    Ok(())
}
fn solid(w: &mut W, color: Rgba) {
    w.open0("a:solidFill");
    w.open("a:srgbClr", A::new().a("val", color.hex()));
    w.val("a:alpha", u32::from(color.a) * 100_000 / 255);
    w.close("a:srgbClr");
    w.close("a:solidFill");
}
