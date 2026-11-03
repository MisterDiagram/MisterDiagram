//! DrawingML paint and line output retargeted from storytold/deckcraft
//! crates/pptx/src/write/dml.rs, 90058395b78be26bb7ea7b4914a9f113c28d979d.
//! Copyright (c) 2026 ArtCraft Team and the DeckCraft contributors.
//! SPDX-License-Identifier: MIT OR Apache-2.0
use super::emu;
use kurbo::{Affine, Cap, Join};
use misterdiagram_color::Rgba;
use misterdiagram_opc::xml::{A, W};
use misterdiagram_render::scene::{EvaluatedShape, Paint};

pub(super) fn solid(w: &mut W, c: Rgba) {
    w.open0("a:solidFill");
    color(w, c);
    w.close("a:solidFill");
}
fn color(w: &mut W, c: Rgba) {
    w.open("a:srgbClr", A::new().a("val", c.hex()));
    w.val("a:alpha", u32::from(c.a) * 100_000 / 255);
    w.close("a:srgbClr");
}
/// Resolved fill only; no theme references or style block can override it.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/fillgradient-section
pub(super) fn fill(w: &mut W, paint: Option<&Paint>, view: Affine, output_bounds: kurbo::Rect) -> Result<(), String> {
    match paint {
        None => w.empty0("a:noFill"),
        Some(Paint::Solid(c)) => solid(w, *c),
        Some(Paint::Pattern { fill: f, gradient: None, .. }) => solid(w, f.foreground),
        Some(Paint::Pattern { gradient: Some(g), transform, bounds, .. }) => {
            if g.stops.is_empty() || g.stops.len() > 4096 {
                return Err("PPTX gradient stop limit".into());
            }
            let (angle, stops) = if g.radial { (0., g.stops.clone()) } else { linear_gradient(g, view * *transform, *bounds, output_bounds)? };
            w.open("a:gradFill", A::new().a("rotWithShape", 0).a("flip", "none"));
            w.open0("a:gsLst");
            for (position, c) in &stops {
                w.open("a:gs", A::new().a("pos", percent(*position)?));
                color(w, *c);
                w.close("a:gs");
            }
            w.close("a:gsLst");
            if g.radial {
                w.open("a:path", A::new().a("path", "circle"));
                // The renderer samples this corner, in the paint's local box.
                // ECMA-376 20.1.8.31 defines these insets as the focus rectangle.
                let focus = view * *transform * kurbo::Point::new(bounds.x1, bounds.y0);
                let x = if output_bounds.width() > 1e-12 { (focus.x - output_bounds.x0) / output_bounds.width() } else { 0.5 };
                let y = if output_bounds.height() > 1e-12 { (focus.y - output_bounds.y0) / output_bounds.height() } else { 0.5 };
                w.empty(
                    "a:fillToRect",
                    A::new()
                        .a("l", integer(x * 100000.)?)
                        .a("t", integer(y * 100000.)?)
                        .a("r", integer((1. - x) * 100000.)?)
                        .a("b", integer((1. - y) * 100000.)?),
                );
                w.close("a:path");
            } else {
                w.empty("a:lin", A::new().a("ang", integer(angle)?).a("scaled", 0));
            }
            w.close("a:gradFill");
        }
    }
    Ok(())
}
/// Transform the renderer's scalar gradient by the inverse transpose. Baking
/// rotation expands the path's axis-aligned box, so stops must retain the old
/// scalar domain rather than stretching to the new box.
/// https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.drawing.lineargradientfill.scaled
fn linear_gradient(
    g: &misterdiagram_render::style::Gradient,
    matrix: Affine,
    bounds: kurbo::Rect,
    output: kurbo::Rect,
) -> Result<(f64, Vec<(f64, Rgba)>), String> {
    let (sin, cos) = g.angle.sin_cos();
    let divisor = (cos.abs() + sin.abs()).max(1e-12);
    let x = cos / bounds.width().max(1e-12) / divisor;
    let y = sin / bounds.height().max(1e-12) / divisor;
    let [a, b, c, d, _, _] = matrix.inverse().as_coeffs();
    let k = kurbo::Vec2::new(a * x + b * y, c * x + d * y);
    let center = 0.5 + k.dot(output.center() - matrix * bounds.center());
    let span = k.x.abs() * output.width() + k.y.abs() * output.height();
    if !center.is_finite() || !span.is_finite() || span <= 0. {
        return Err("PPTX gradient transform".into());
    }
    let mut stops = Vec::new();
    let left = center - span / 2.;
    let right = center + span / 2.;
    if !g.stops.iter().any(|(t, _)| (*t - left).abs() < 1e-12) {
        stops.push((0., g.at(left.clamp(0., 1.))));
    }
    for &(t, color) in &g.stops {
        let position = 0.5 + (t - center) / span;
        if (-1e-12..=1. + 1e-12).contains(&position) {
            stops.push((position.clamp(0., 1.), color));
        }
    }
    if !g.stops.iter().any(|(t, _)| (*t - right).abs() < 1e-12) {
        stops.push((1., g.at(right.clamp(0., 1.))));
    }
    if stops.len() < 2 {
        return Err("PPTX gradient stop domain".into());
    }
    Ok((k.atan2().to_degrees().rem_euclid(360.) * 60000., stops))
}

fn integer(n: f64) -> Result<i64, String> {
    if !n.is_finite() || n.abs() > 2e9 { Err("PPTX numeric limit".into()) } else { Ok(n.round() as i64) }
}
fn percent(n: f64) -> Result<i64, String> {
    if !(0. ..=1.).contains(&n) {
        return Err("PPTX percentage limit".into());
    }
    integer(n * 100_000.)
}
/// Physical renderer dash periods divided by pen width, per DrawingML custDash.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/linepattern-cell-line-format-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/lineweight-cell-line-format-section
pub(super) fn line(w: &mut W, node: &EvaluatedShape, scale: f64) -> Result<(), String> {
    let Some((line, stroke)) = node.line.as_ref().zip(node.stroke.as_ref()).filter(|(l, _)| l.pattern != 0) else {
        w.open0("a:ln");
        w.empty0("a:noFill");
        w.close("a:ln");
        return Ok(());
    };
    let width = emu(stroke.width * scale)?;
    if !(0..=20_116_800).contains(&width) {
        return Err("PPTX line width limit".into());
    }
    w.open(
        "a:ln",
        A::new()
            .a("w", width)
            .a(
                "cap",
                match stroke.start_cap {
                    Cap::Butt => "flat",
                    Cap::Round => "rnd",
                    Cap::Square => "sq",
                },
            )
            .a("cmpd", "sng")
            .a("algn", "ctr"),
    );
    solid(w, line.color);
    if stroke.dash_pattern.is_empty() {
        w.val("a:prstDash", "solid");
    } else {
        if stroke.dash_pattern.len() > 4096 || stroke.dash_pattern.len() % 2 != 0 || stroke.width <= 0. {
            return Err("PPTX dash limit".into());
        }
        w.open0("a:custDash");
        for &[d, s] in stroke.dash_pattern.as_chunks::<2>().0 {
            w.empty("a:ds", A::new().a("d", integer(d / stroke.width * 100_000.)?.max(0)).a("sp", integer(s / stroke.width * 100_000.)?.max(0)));
        }
        w.close("a:custDash");
    }
    match stroke.join {
        Join::Round => w.empty0("a:round"),
        Join::Bevel => w.empty0("a:bevel"),
        Join::Miter => w.empty("a:miter", A::new().a("lim", integer(stroke.miter_limit * 100_000.)?)),
    }
    let open = node.sections.iter().any(|s| s.stroked() && !s.path.elements().iter().any(|e| matches!(e, kurbo::PathEl::ClosePath)));
    for (tag, index, size) in [("a:headEnd", line.begin_arrow, line.begin_size), ("a:tailEnd", line.end_arrow, line.end_size)] {
        let (kind, _) = arrow(if open { index } else { 0 });
        w.empty(tag, A::new().a("type", kind).a("w", arrow_size(size)).a("len", arrow_size(size)));
    }
    w.close("a:ln");
    Ok(())
}
/// All 45 native indexes, with shape approximations marked explicitly. A drawn
/// but unsupported marker remains a triangle, never a silently missing end.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/endarrow-cell-line-format-section
pub fn arrow(index: u32) -> (&'static str, bool) {
    const TABLE: [(&str, bool); 46] = [
        ("none", false),     // 0
        ("arrow", false),    // 1
        ("triangle", false), // 2
        ("arrow", true),     // 3
        ("triangle", true),  // 4
        ("stealth", true),   // 5
        ("triangle", true),  // 6
        ("arrow", true),     // 7
        ("stealth", true),   // 8
        ("triangle", true),  // 9
        ("oval", false),     // 10
        ("diamond", true),   // 11
        ("arrow", true),     // 12
        ("triangle", true),  // 13
        ("triangle", true),  // 14
        ("triangle", true),  // 15
        ("triangle", true),  // 16
        ("stealth", true),   // 17
        ("triangle", true),  // 18
        ("stealth", true),   // 19
        ("oval", true),      // 20
        ("diamond", true),   // 21
        ("diamond", true),   // 22
        ("triangle", true),  // 23
        ("triangle", true),  // 24
        ("triangle", true),  // 25
        ("triangle", true),  // 26
        ("arrow", true),     // 27
        ("arrow", true),     // 28
        ("oval", true),      // 29
        ("oval", true),      // 30
        ("oval", true),      // 31
        ("oval", true),      // 32
        ("oval", true),      // 33
        ("diamond", true),   // 34
        ("oval", true),      // 35
        ("oval", true),      // 36
        ("oval", true),      // 37
        ("diamond", true),   // 38
        ("triangle", true),  // 39
        ("triangle", true),  // 40
        ("oval", true),      // 41
        ("oval", true),      // 42
        ("arrow", true),     // 43
        ("arrow", true),     // 44
        ("arrow", true),     // 45
    ];
    usize::try_from(index).ok().and_then(|i| TABLE.get(i)).copied().unwrap_or(("triangle", true))
}
/// https://learn.microsoft.com/en-us/office/client-developer/visio/beginarrowsize-cell-line-format-section
fn arrow_size(size: u32) -> &'static str {
    match size {
        0 | 1 => "sm",
        2 => "med",
        _ => "lg",
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_drawn_arrow_remains_visible() {
        for index in 1..=45 {
            assert_ne!(arrow(index).0, "none");
        }
        assert_eq!(arrow(u32::MAX), ("triangle", true));
        assert_eq!(arrow_size(0), "sm");
        assert_eq!(arrow_size(2), "med");
        assert_eq!(arrow_size(6), "lg");
    }
}
