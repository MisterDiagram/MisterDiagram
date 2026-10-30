//! Editable SVG text names the requested face and its actually measured metric twin.
//! https://www.w3.org/TR/SVG11/text.html#TextElement
use misterdiagram_geom::Affine;
use misterdiagram_text::{GlyphRun, TextLayout};
use std::fmt::Write as _;

fn escape(out: &mut String, value: &str) {
    for c in value.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            // XML 1.0 cannot represent these even as numeric references.
            '\0'..='\u{8}' | '\u{b}'..='\u{c}' | '\u{e}'..='\u{1f}' | '\u{fffe}' | '\u{ffff}' => out.push('\u{fffd}'),
            _ => out.push(c),
        }
    }
}
/// Serialize a literal CSS family string inside an XML attribute.
/// https://www.w3.org/TR/css-fonts-4/#font-family-prop
fn quoted_family(out: &mut String, family: &str) {
    out.push_str("&apos;");
    for c in family.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '\'' => out.push_str("\\&apos;"),
            '\t' => out.push_str("\\9 "),
            '\n' => out.push_str("\\a "),
            '\r' => out.push_str("\\d "),
            '\u{c}' => out.push_str("\\c "),
            _ => escape(out, c.encode_utf8(&mut [0; 4])),
        }
    }
    out.push_str("&apos;");
}
fn metric_twin(requested: &str, measured: &str) -> bool {
    let twins: &[&str] = match requested.to_ascii_lowercase().as_str() {
        "calibri" => &["Carlito"],
        "cambria" => &["Caladea"],
        "arial" => &["Arimo", "Liberation Sans"],
        "times new roman" => &["Tinos", "Liberation Serif"],
        "courier new" => &["Cousine", "Liberation Mono"],
        _ => &[],
    };
    twins.iter().any(|twin| measured.eq_ignore_ascii_case(twin))
}
fn font_family(out: &mut String, requested: &str, measured: &str) {
    quoted_family(out, requested);
    if metric_twin(requested, measured) {
        out.push_str(", ");
        quoted_family(out, measured);
    }
}
fn style(out: &mut String, run: &GlyphRun, decorations: bool) -> Result<(), String> {
    write!(out, " letter-spacing=\"{}\"", run.svg.spacing).map_err(|e| e.to_string())?;
    out.push_str(" font-family=\"");
    font_family(out, &run.svg.family, &run.face.family);
    let c = run.color;
    write!(
        out,
        "\" font-size=\"{}\" fill=\"#{:02x}{:02x}{:02x}\" fill-opacity=\"{}\" font-weight=\"{}\" font-style=\"{}\" text-decoration=\"{}\"",
        run.size,
        c.r,
        c.g,
        c.b,
        f64::from(c.a) * run.alpha.clamp(0., 1.) / 255.,
        if run.svg.bold { "bold" } else { "normal" },
        if run.svg.italic { "italic" } else { "normal" },
        match (decorations && run.svg.underline, decorations && run.svg.strike) {
            (true, true) => "underline line-through",
            (true, false) => "underline",
            (false, true) => "line-through",
            _ => "none",
        }
    )
    .map_err(|e| e.to_string())
}
fn advance(run: &GlyphRun, value: &str) -> f64 {
    let features: Vec<_> = misterdiagram_fonts::feature("-kern").into_iter().collect();
    misterdiagram_fonts::shape(&run.face, value, &features, |c| c).iter().map(|g| f64::from(g.x_advance)).sum::<f64>() * run.size
        / run.face.upem.max(1.)
        + run.svg.spacing * value.chars().count() as f64
}
pub(super) fn text(out: &mut String, layout: &TextLayout, m: Affine) -> Result<(), String> {
    text_limited(out, layout, m, 256 * 1024 * 1024)
}
fn room(out: &str, family: &str, fragment: &str, limit: usize) -> Result<(), String> {
    let extra = family.len().saturating_mul(8).saturating_add(fragment.len().saturating_mul(6)).saturating_add(2048);
    if out.len().saturating_add(extra) > limit { Err("SVG output limit".into()) } else { Ok(()) }
}
fn text_limited(out: &mut String, layout: &TextLayout, m: Affine, limit: usize) -> Result<(), String> {
    let Some(first) = layout.lines.first() else { return Ok(()) };
    // Factor rotation explicitly while retaining the writer's existing canvas mapping.
    let [a, b, ..] = m.as_coeffs();
    let angle = b.atan2(a);
    write!(
        out,
        "\n<g transform=\"{}\"><text xml:space=\"preserve\" kerning=\"0\" style=\"font-kerning:none\" transform=\"rotate({})\"",
        super::matrix(m * Affine::rotate(-angle)),
        angle.to_degrees()
    )
    .map_err(|e| e.to_string())?;
    let start = |run: Option<&GlyphRun>, line: &misterdiagram_text::LineInfo| run.and_then(|r| r.glyphs.first()).map_or(line.extent.0, |g| g.1);
    write!(out, " x=\"{}\" y=\"{}\"", start(layout.runs.first(), first), first.baseline).map_err(|e| e.to_string())?;
    if let Some(run) = layout.runs.first() {
        room(out, &run.svg.family, "", limit)?;
        style(out, run, false)?;
    }
    // Every line is anchored at its visual left edge, which the layout already computed;
    // renderers disagree on what `text-anchor="end"` means under `direction="rtl"` (usvg 0.47
    // anchors the right edge), so the paragraph direction travels in the isolates below.
    out.push_str(" direction=\"ltr\" text-anchor=\"start\"");
    out.push('>');
    let mut run_index = 0;
    for (i, line) in layout.lines.iter().enumerate() {
        let x = start(layout.runs.get(run_index).filter(|r| r.svg.line == i), line);
        if i != 0 {
            write!(out, "<tspan x=\"{x}\" y=\"{}\" direction=\"ltr\" text-anchor=\"start\">", line.baseline).map_err(|e| e.to_string())?;
        }
        let mut cursor = x;
        while let Some(run) = layout.runs.get(run_index).filter(|r| r.svg.line == i) {
            run_index += 1;
            let mut ci = run.chars.0;
            for fragment in run.svg.text.split('\t') {
                if !fragment.is_empty() {
                    let rx = if run.cells.is_empty() {
                        run.glyphs.first().map_or(cursor, |g| g.1)
                    } else {
                        ci.checked_sub(line.start)
                            .map(|k| line.edges.iter().skip(k).take(fragment.chars().count()).map(|e| e.0).reduce(f64::min).unwrap_or(cursor))
                            .unwrap_or(cursor)
                    };
                    room(out, &run.svg.family, fragment, limit)?;
                    write!(out, "<tspan unicode-bidi=\"isolate\" direction=\"{}\"", if line.rtl { "rtl" } else { "ltr" })
                        .map_err(|e| e.to_string())?;
                    style(out, run, true)?;
                    let offset = run.svg.baseline_shift;
                    write!(out, " dx=\"{}\" baseline-shift=\"{offset}\">", rx - cursor).map_err(|e| e.to_string())?;
                    // usvg 0.47 runs bidi with an LTR base whatever `direction` says; the
                    // isolate characters carry the paragraph direction to any bidi algorithm.
                    if line.rtl {
                        out.push('\u{2067}');
                    }
                    escape(out, fragment);
                    if line.rtl {
                        out.push('\u{2069}');
                    }
                    out.push_str("</tspan>");
                    cursor = rx + advance(run, fragment);
                }
                ci = ci.saturating_add(fragment.chars().count()).saturating_add(1);
            }
        }
        if i != 0 {
            out.push_str("</tspan>");
        }
        if out.len() > limit {
            return Err("SVG output limit".into());
        }
    }
    out.push_str("</text></g>\n");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn repeated_tab_styles_stop_before_the_output_budget() {
        use misterdiagram_model::text::{BodyProps, Paragraph, TextBody};
        use misterdiagram_text::{NoFields, Opts};
        let mut body = TextBody { body: BodyProps { wrap: Some(false), ..Default::default() }, ..Default::default() };
        body.paragraphs.push(Paragraph::new(&"x\t".repeat(1000)));
        body.paragraphs[0].runs[0].props.font = Some("long & family ".repeat(100));
        let layout = misterdiagram_text::layout(
            &body,
            &Opts { rect: kurbo::Rect::new(0., 0., 100., 100.), fields: &NoFields, prompt_color: None, no_shrink: true },
        )
        .unwrap();
        let mut out = String::new();
        assert!(text_limited(&mut out, &layout, Affine::IDENTITY, 32768).unwrap_err().contains("limit"));
        assert!(out.len() < 32768);
    }
    #[test]
    fn only_the_measured_metric_twin_is_named() {
        for (requested, twin) in [
            ("Calibri", "Carlito"),
            ("Cambria", "Caladea"),
            ("Arial", "Arimo"),
            ("Arial", "Liberation Sans"),
            ("Times New Roman", "Tinos"),
            ("Times New Roman", "Liberation Serif"),
            ("Courier New", "Cousine"),
            ("Courier New", "Liberation Mono"),
        ] {
            let mut out = String::new();
            font_family(&mut out, requested, twin);
            assert_eq!(out, format!("&apos;{requested}&apos;, &apos;{twin}&apos;"));
        }
        for (requested, measured) in [
            ("Calibri", "Liberation Sans"),
            ("Calibri", "Inter"),
            ("Calibri", "Noto Sans Symbols"),
            ("Cambria", "Merriweather"),
            ("Original Face", "Carlito"),
            ("Arial", "Arial"),
        ] {
            let mut out = String::new();
            font_family(&mut out, requested, measured);
            assert_eq!(out, format!("&apos;{requested}&apos;"));
        }
    }
    #[test]
    fn css_family_quoting_precedes_xml_escaping() {
        let mut out = String::new();
        font_family(&mut out, "Original, 'Face' \\\"\n\r\u{c}<>&", "Carlito");
        assert_eq!(out, r#"&apos;Original, \&apos;Face\&apos; \\&quot;\a \d \c &lt;&gt;&amp;&apos;"#);
        let mut tab = String::new();
        font_family(&mut tab, "Original\tFace", "Carlito");
        let tab_xml = format!("<text font-family=\"{tab}\"/>");
        let tab_doc = resvg::usvg::roxmltree::Document::parse(&tab_xml).unwrap();
        assert_eq!(tab_doc.root_element().attribute("font-family"), Some("'Original\\9 Face'"));
        let xml = format!("<text font-family=\"{out}\"/>");
        let parsed = resvg::usvg::roxmltree::Document::parse(&xml).unwrap();
        assert!(parsed.root_element().attribute("font-family").unwrap().starts_with("'Original, \\'Face\\'"));
    }
}
