//! Editable text retargeted from storytold/deckcraft crates/pptx/src/write/dml.rs,
//! 90058395b78be26bb7ea7b4914a9f113c28d979d.
//! Copyright (c) 2026 ArtCraft Team and the DeckCraft contributors.
//! SPDX-License-Identifier: MIT OR Apache-2.0
use super::{dml, emu};
use kurbo::{Affine, Point, Rect};
use misterdiagram_color::Rgba;
use misterdiagram_model::text::{Anchor, Bullet, Caps, ParaProps, RunKind, RunProps, Spacing, Strike, TextBody};
use misterdiagram_opc::xml::{A, W};
use misterdiagram_render::scene::EvaluatedTextBlock;

/// Saved text-block points, including TxtAngle and ancestor transforms, become
/// a physical rectangle plus independent DrawingML text rotation.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/txtangle-cell-text-transform-section
/// https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.drawing.bodyproperties.rotation
pub(super) fn placement(block: &EvaluatedTextBlock, view: Affine) -> Result<(Rect, f64, f64), String> {
    let m = view * block.to_page;
    let [a, b, c, d, _, _] = m.as_coeffs();
    let sx = a.hypot(b);
    let sy = c.hypot(d);
    if m.as_coeffs().iter().any(|v| !v.is_finite())
        || sx <= 0.
        || sy <= 0.
        || (a * c + b * d).abs() > 1e-8 * sx * sy
        || (sx - sy).abs() > 1e-8 * sx.max(sy)
        || m.determinant() <= 0.
    {
        return Err("PPTX text transform".into());
    }
    let width = block.rect.width() * sx;
    let height = block.rect.height() * sy;
    let center = m * Point::new(block.rect.center().x, block.rect.center().y);
    let angle = b.atan2(a).to_degrees();
    Ok((Rect::from_center_size(center, (width, height)), angle, sx * 72.))
}
fn integer(v: f64, lo: f64, hi: f64) -> Result<i64, String> {
    if !v.is_finite() || v < lo || v > hi {
        return Err("PPTX text metric limit".into());
    }
    Ok(v.round() as i64)
}
fn points(v: f64, scale: f64) -> Result<i64, String> {
    emu(v * scale / 72.)
}
// ECMA-376 Part 1: ST_TextMargin and ST_TextIndent have narrower limits
// than the coordinate type used by text-block insets and tab stops.
fn paragraph_metric(v: f64, scale: f64, signed: bool) -> Result<i64, String> {
    let value = points(v, scale)?;
    let lower = if signed { -51_206_400 } else { 0 };
    if !(lower..=51_206_400).contains(&value) {
        return Err("PPTX paragraph metric limit".into());
    }
    Ok(value)
}
fn checked_text(s: &str) -> Result<(), String> {
    if s.len() > 65536 || s.chars().any(|c| !matches!(c, '\t'|'\n'|'\r'|'\u{20}'..='\u{d7ff}'|'\u{e000}'..='\u{fffd}'|'\u{10000}'..='\u{10ffff}')) {
        return Err("PPTX text string limit".into());
    }
    Ok(())
}
fn reserve(w: &W, mut strings: impl Iterator<Item = usize>, extra: usize, limit: usize) -> Result<(), String> {
    let n = strings.try_fold(extra, |n, v| n.checked_add(v.checked_mul(6)?)).ok_or("PPTX text XML limit")?;
    if w.s.len().checked_add(n).is_none_or(|v| v > limit) {
        return Err("PPTX text XML limit".into());
    }
    Ok(())
}
/// Insets retain evaluated points, scaled only with the whole presentation page.
/// No fit-to-box or font-size compensation for PowerPoint wrapping is applied.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/leftmargin-cell-text-block-format-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/verticalalign-cell-text-block-format-section
/// https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.drawing.bodyproperties
pub(super) fn body(w: &mut W, body: &TextBody, scale: f64, angle: f64, limit: usize) -> Result<(), String> {
    if !scale.is_finite() || scale <= 0. || body.paragraphs.len() > 4096 {
        return Err("PPTX text body limit".into());
    }
    // Bound the entire body before any escaped strings are appended. This
    // includes both run text and properties, not independent partial estimates.
    let mut estimate = 2048usize;
    let mut total_runs = 0usize;
    for p in &body.paragraphs {
        total_runs = total_runs.checked_add(p.runs.len()).ok_or("PPTX text run limit")?;
        if total_runs > 65536 || p.props.tabs.as_ref().is_some_and(|tabs| tabs.len() > 4096) {
            return Err("PPTX text structure limit".into());
        }
        estimate = estimate.saturating_add(4096).saturating_add(p.props.tabs.as_ref().map_or(0, |t| t.len()).saturating_mul(128));
        for s in [p.props.bullet_font.as_ref(), p.props.font_align.as_ref()].into_iter().flatten() {
            estimate = estimate.saturating_add(s.len().saturating_mul(6));
        }
        if let Some(Bullet::Char { char }) = &p.props.bullet {
            estimate = estimate.saturating_add(char.len().saturating_mul(6));
        }
        for (r, text_len) in p.runs.iter().map(|r| (&r.props, r.text.len())).chain(std::iter::once((&p.end_props, 0))) {
            estimate = estimate.saturating_add(2048).saturating_add(text_len.saturating_mul(6));
            for s in [&r.font, &r.font_ea, &r.font_cs, &r.font_sym, &r.lang, &r.underline].into_iter().flatten() {
                estimate = estimate.saturating_add(s.len().saturating_mul(6));
            }
        }
    }
    reserve(w, std::iter::empty(), estimate, limit)?;
    let b = &body.body;
    w.open0("p:txBody");
    w.open(
        "a:bodyPr",
        A::new()
            .a("rot", integer(angle * 60000., -21600000., 21600000.)?)
            .a("lIns", points(b.inset_l.unwrap_or(0.), scale)?)
            .a("rIns", points(b.inset_r.unwrap_or(0.), scale)?)
            .a("tIns", points(b.inset_t.unwrap_or(0.), scale)?)
            .a("bIns", points(b.inset_b.unwrap_or(0.), scale)?)
            .a("anchor", b.anchor.unwrap_or(Anchor::Middle).xml())
            .a("wrap", "square"),
    );
    w.empty0("a:noAutofit");
    w.close("a:bodyPr");
    w.empty0("a:lstStyle");
    if body.paragraphs.is_empty() {
        w.empty0("a:p");
    }
    let mut count = 0usize;
    for p in &body.paragraphs {
        count = count.checked_add(p.runs.len()).ok_or("PPTX text run limit")?;
        if count > 65536 {
            return Err("PPTX text run limit".into());
        }
        w.open0("a:p");
        let size = p.runs.iter().map(|r| r.props.size.unwrap_or(12.)).fold(p.end_props.size.unwrap_or(12.), f64::max);
        paragraph(w, &p.props, size, scale, limit)?;
        for run in &p.runs {
            checked_text(&run.text)?;
            reserve(w, std::iter::once(run.text.len()), 128, limit)?;
            let tag = if matches!(run.kind, RunKind::Break) { "a:br" } else { "a:r" };
            w.open0(tag);
            properties(w, "a:rPr", &run.props, scale, limit)?;
            if tag == "a:r" {
                // Fields contain their evaluated value, not PowerPoint field semantics.
                w.open("a:t", A::new().a("xml:space", "preserve"));
                w.text(&run.text);
                w.close("a:t");
            }
            w.close(tag);
        }
        properties(w, "a:endParaRPr", &p.end_props, scale, limit)?;
        w.close("a:p");
    }
    w.close("p:txBody");
    Ok(())
}
fn spacing(w: &mut W, tag: &str, s: Spacing, size: f64, scale: f64) -> Result<(), String> {
    w.open0(tag);
    match s {
        Spacing::Pct(v) => w.val("a:spcPct", integer(v * 100000., 0., 13200000.)?),
        Spacing::FontPct(v) => w.val("a:spcPts", integer(v * size * scale * 100., 0., 158400.)?),
        Spacing::Pts(v) => w.val("a:spcPts", integer(v * scale * 100., 0., 158400.)?),
    }
    w.close(tag);
    Ok(())
}
/// Visio font-relative SpLine becomes explicit point spacing. Bullet resets and
/// tab stops are emitted even when a prior paragraph had different formatting.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/spline-cell-paragraph-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/bullet-cell-paragraph-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/position-cell-tabs-section
fn paragraph(w: &mut W, p: &ParaProps, size: f64, scale: f64, limit: usize) -> Result<(), String> {
    let tabs = p.tabs.as_deref().unwrap_or(&[]);
    if tabs.len() > 4096 {
        return Err("PPTX tab count limit".into());
    }
    let bullet = match &p.bullet {
        Some(Bullet::Char { char }) => Some(char.as_str()),
        None | Some(Bullet::None) => None,
        _ => return Err("PPTX unsupported evaluated bullet".into()),
    };
    for s in [p.bullet_font.as_deref(), bullet].into_iter().flatten() {
        checked_text(s)?;
    }
    reserve(w, [p.bullet_font.as_ref().map_or(0, String::len), bullet.map_or(0, str::len)].into_iter(), 2048 + tabs.len() * 128, limit)?;
    w.open(
        "a:pPr",
        A::new()
            .a("algn", p.align.map_or("l", |v| v.xml()))
            .a("marL", paragraph_metric(p.margin_left.unwrap_or(0.), scale, false)?)
            .a("marR", paragraph_metric(p.margin_right.unwrap_or(0.), scale, false)?)
            .a("indent", paragraph_metric(p.indent.unwrap_or(0.), scale, true)?)
            .a("defTabSz", points(p.default_tab.unwrap_or(36.), scale)?)
            .a("rtl", u8::from(p.rtl.unwrap_or(false))),
    );
    for (tag, s) in [("a:lnSpc", p.line_spacing), ("a:spcBef", p.space_before), ("a:spcAft", p.space_after)] {
        if let Some(s) = s {
            spacing(w, tag, s, size, scale)?;
        }
    }
    if let Some(bullet) = bullet {
        if let Some(c) = p.bullet_color {
            w.open0("a:buClr");
            dml::color(w, c);
            w.close("a:buClr");
        } else {
            w.empty0("a:buClrTx");
        }
        if let Some(s) = p.bullet_size {
            w.val("a:buSzPct", integer(s * 100000., 25000., 400000.)?);
        } else {
            w.empty0("a:buSzTx");
        }
        if let Some(font) = &p.bullet_font {
            w.empty("a:buFont", A::new().a("typeface", font));
        } else {
            w.empty0("a:buFontTx");
        }
        w.empty("a:buChar", A::new().a("char", if bullet.is_empty() { "•" } else { bullet }));
    } else {
        w.empty0("a:buNone");
    }
    w.open0("a:tabLst");
    if let Some(pos) = p.bullet_tab.filter(|_| bullet.is_some()) {
        w.empty("a:tab", A::new().a("pos", points(pos, scale)?).a("algn", "l"));
    }
    for tab in tabs {
        w.empty(
            "a:tab",
            A::new().a("pos", points(tab.pos, scale)?).a(
                "algn",
                match tab.align.as_str() {
                    "ctr" => "ctr",
                    "r" => "r",
                    "dec" => "dec",
                    _ => "l",
                },
            ),
        );
    }
    w.close("a:tabLst");
    w.close("a:pPr");
    Ok(())
}
/// Fully resolved character properties never refer to a theme font or colour.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/style-cell-character-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pos-cell-character-section
/// https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.drawing.runproperties
fn properties(w: &mut W, tag: &str, r: &RunProps, scale: f64, limit: usize) -> Result<(), String> {
    let strings = [r.font.as_deref(), r.font_ea.as_deref(), r.font_cs.as_deref(), r.font_sym.as_deref(), r.lang.as_deref(), r.underline.as_deref()];
    for s in strings.into_iter().flatten() {
        checked_text(s)?;
    }
    reserve(w, strings.into_iter().flatten().map(str::len), 1536, limit)?;
    let underline = match r.underline.as_deref().unwrap_or("none") {
        s @ ("none" | "sng" | "dbl" | "heavy" | "dotted" | "dottedHeavy" | "dash" | "dashHeavy" | "dashLong" | "dashLongHeavy" | "dotDash"
        | "dotDashHeavy" | "dotDotDash" | "dotDotDashHeavy" | "wavy" | "wavyHeavy" | "wavyDbl") => s,
        _ => return Err("PPTX underline value".into()),
    };
    w.open(
        tag,
        A::new()
            .a("sz", integer(r.size.unwrap_or(12.) * scale * 100., 100., 400000.)?)
            .a("b", u8::from(r.bold.unwrap_or(false)))
            .a("i", u8::from(r.italic.unwrap_or(false)))
            .a("u", underline)
            .a(
                "strike",
                match r.strike.unwrap_or(Strike::None) {
                    Strike::None => "noStrike",
                    Strike::Single => "sngStrike",
                    Strike::Double => "dblStrike",
                },
            )
            .a(
                "cap",
                match r.caps.unwrap_or(Caps::None) {
                    Caps::None => "none",
                    Caps::Small => "small",
                    Caps::All => "all",
                },
            )
            .a("baseline", integer(r.baseline.unwrap_or(0.) * 100000., -1000000., 1000000.)?)
            .a("spc", integer(r.spacing.unwrap_or(0.) * scale * 100., -400000., 400000.)?)
            .a("kern", integer(r.kern.unwrap_or(0.) * scale * 100., 0., 400000.)?)
            .o("lang", r.lang.as_deref()),
    );
    dml::solid(w, r.fill.unwrap_or(Rgba::BLACK));
    w.empty("a:latin", A::new().a("typeface", r.font.as_deref().unwrap_or("Calibri")));
    for (tag, font) in [("a:ea", &r.font_ea), ("a:cs", &r.font_cs), ("a:sym", &r.font_sym)] {
        if let Some(font) = font {
            w.empty(tag, A::new().a("typeface", font));
        }
    }
    w.close(tag);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use misterdiagram_model::text::{Paragraph, Run};
    #[test]
    fn combined_text_and_font_strings_are_bounded_before_xml_growth() {
        let r = Run::with("&".repeat(2000), RunProps { font: Some("&".repeat(2000)), ..Default::default() });
        let text = TextBody { paragraphs: vec![Paragraph { runs: vec![r], ..Default::default() }], ..Default::default() };
        let mut w = W::new();
        let before = w.s.len();
        assert!(body(&mut w, &text, 1., 0., 20000).is_err());
        assert_eq!(w.s.len(), before);
    }
}
