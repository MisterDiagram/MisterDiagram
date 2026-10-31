//! Original editable-text, styling and hostile-input regressions.
#[path = "support/svg_fonts.rs"]
mod svg_fonts;
use misterdiagram_geom::{Affine, Rect};
use misterdiagram_model::text::{BodyProps, Caps, Paragraph, Run, RunProps, Strike, TextBody};
use misterdiagram_render::{
    pdf,
    scene::{Op, Options, Scene},
    svg,
};
use misterdiagram_text::{NoFields, Opts};
fn scene(body: &TextBody) -> Scene {
    let layout = misterdiagram_text::layout_for_export(
        body,
        &Opts { rect: Rect::new(0., 0., 288., 144.), fields: &NoFields, prompt_color: None, no_shrink: true },
    )
    .unwrap();
    Scene {
        pdf_strokes: Default::default(),
        warnings: vec![],
        ops: vec![Op::Text { layout, export_bounds: None, transform: Affine::scale_non_uniform(1. / 72., -1. / 72.) }],
        bounds: Rect::new(0., -2., 4., 0.),
        page_size: (4., 2.),
    }
}
fn xml(scene: &Scene) -> String {
    svg::render(scene, Options::default()).unwrap()
}
#[test]
fn editable_lines_requested_fonts_styles_and_outlines() {
    let mut body = TextBody::default();
    body.paragraphs.push(Paragraph::new("searchable first line"));
    body.paragraphs.push(Paragraph::new("second"));
    body.paragraphs[0].runs[0].props = RunProps { font: Some("Calibri".into()), size: Some(12.), ..Default::default() };
    body.paragraphs[1].runs[0].props = RunProps {
        font: Some("Arial".into()),
        bold: Some(true),
        italic: Some(true),
        underline: Some("sng".into()),
        strike: Some(Strike::Single),
        caps: Some(Caps::All),
        ..Default::default()
    };
    let scene = scene(&body);
    let xml = xml(&scene);
    assert_eq!(xml.matches("<text ").count(), 1);
    assert_eq!(xml.matches("<tspan x=").count(), 1);
    assert!(!xml.contains("<path"));
    for token in ["Calibri", "Arial", "SECOND", "font-weight=\"bold\"", "font-style=\"italic\"", "underline line-through", "xml:space=\"preserve\""] {
        assert!(xml.contains(token), "{token}: {xml}");
    }
    assert!(xml.contains("Carlito"));
    let tree = resvg::usvg::Tree::from_str(&xml, &svg_fonts::options()).unwrap();
    assert!(!tree.root().children().is_empty());
    let outlines = svg::render(&scene, Options { text_outlines: true, ..Default::default() }).unwrap();
    assert!(outlines.contains("<path"));
    assert!(!outlines.contains("<text"));
    assert_eq!(
        pdf::render(std::slice::from_ref(&scene), Options::default()).unwrap(),
        pdf::render(&[scene], Options { text_outlines: true, ..Default::default() }).unwrap()
    );
}
#[test]
fn escaped_controls_unknown_font_symbols_rotation_and_flip() {
    let mut body = TextBody::default();
    body.paragraphs.push(Paragraph::new("<&>\"'\0\u{1}✔✘"));
    body.paragraphs[0].runs[0].props.font = Some("Original <&>\"' Face".into());
    let mut scene = scene(&body);
    if let Op::Text { transform, .. } = &mut scene.ops[0] {
        *transform = Affine::rotate(0.4) * Affine::scale_non_uniform(-1., 1.) * *transform;
    }
    let xml = xml(&scene);
    assert!(xml.contains("rotate("));
    assert!(xml.contains("&lt;&amp;&gt;&quot;&apos;"));
    assert!(!xml.contains("Noto Sans Symbols"));
    assert!(!xml.contains('\0'));
    resvg::usvg::roxmltree::Document::parse(&xml).unwrap();
    resvg::usvg::Tree::from_str(&xml, &svg_fonts::options()).unwrap();
}
#[test]
fn bounded_large_text_and_paragraphs_and_nonfinite_scene() {
    let opts = Opts { rect: Rect::new(0., 0., 288., 144.), fields: &NoFields, prompt_color: None, no_shrink: true };
    let mut body = TextBody { body: BodyProps { wrap: Some(false), ..Default::default() }, ..Default::default() };
    body.paragraphs.push(Paragraph::new(&"a".repeat(65536)));
    assert!(misterdiagram_text::layout_for_export(&body, &opts).unwrap_err().contains("limit"));
    body.paragraphs[0].runs[0] = Run::new("a".repeat(65533));
    let large = xml(&scene(&body));
    resvg::usvg::roxmltree::Document::parse(&large).unwrap();
    body.paragraphs = vec![Paragraph::new("x"); 4096];
    let mut scene = scene(&body);
    let many = xml(&scene);
    assert_eq!(many.matches("<tspan x=").count(), 4095);
    resvg::usvg::roxmltree::Document::parse(&many).unwrap();
    if let Op::Text { layout, .. } = &mut scene.ops[0] {
        layout.lines[0].baseline = f64::NAN;
    }
    assert!(svg::render(&scene, Options::default()).is_err());
}

#[test]
fn mixed_decoration_stays_local_and_highlights_remain_behind_text() {
    let mut body = TextBody::default();
    let mut p = Paragraph::new("");
    p.runs = vec![
        Run::with(
            "marked",
            RunProps {
                underline: Some("sng".into()),
                highlight: Some(misterdiagram_color::Rgba { r: 255, g: 255, b: 0, a: 255 }),
                ..Default::default()
            },
        ),
        Run::new(" plain"),
    ];
    body.paragraphs.push(p);
    let xml = xml(&scene(&body));
    let doc = resvg::usvg::roxmltree::Document::parse(&xml).unwrap();
    let text = doc.descendants().find(|n| n.has_tag_name("text")).unwrap();
    assert_eq!(text.attribute("text-decoration"), Some("none"));
    let spans: Vec<_> = text.children().filter(|n| n.has_tag_name("tspan")).collect();
    assert_eq!(spans[0].attribute("text-decoration"), Some("underline"));
    assert_eq!(spans[1].attribute("text-decoration"), Some("none"));
    assert!(xml.contains("fill=\"#ffff00\""));
    assert!(xml.find("fill=\"#ffff00\"").unwrap() < xml.find("<text").unwrap());
}
#[test]
fn rtl_text_starts_at_the_visual_glyph_origin() {
    let mut body = TextBody::default();
    body.paragraphs.push(Paragraph::new("אבג"));
    body.paragraphs[0].runs[0].props.font = Some("Arial".into());
    let scene = scene(&body);
    let png = misterdiagram_render::raster::render(&scene, Options::default()).unwrap();
    let xml = xml(&scene);
    let tree = resvg::usvg::Tree::from_str(&xml, &svg_fonts::options()).unwrap();
    let mut pixels = resvg::tiny_skia::Pixmap::new(png.width(), png.height()).unwrap();
    pixels.fill(resvg::tiny_skia::Color::WHITE);
    resvg::render(&tree, resvg::tiny_skia::Transform::identity(), &mut pixels.as_mut());
    let ours = png.enumerate_pixels().filter(|(_, _, p)| p.0[0] < 128).map(|(x, _, _)| x).min().unwrap();
    let svg = pixels.pixels().iter().enumerate().filter(|(_, p)| p.red() < 128).map(|(i, _)| (i % png.width() as usize) as u32).min().unwrap();
    assert!(ours.abs_diff(svg) <= 1, "PNG x={ours}, SVG x={svg}");
}

#[test]
fn mixed_direction_raster_matches_the_layout_base_direction() {
    let mut body = TextBody::default();
    body.paragraphs.push(Paragraph::new("ABC אבג 123"));
    body.paragraphs[0].props.rtl = Some(true);
    body.paragraphs[0].runs[0].props.font = Some("Arial".into());
    let scene = scene(&body);
    let png = misterdiagram_render::raster::render(&scene, Options::default()).unwrap();
    let xml = xml(&scene);
    assert!(xml.contains("direction=\"rtl\""));
    let tree = resvg::usvg::Tree::from_str(&xml, &svg_fonts::options()).unwrap();
    let mut pixels = resvg::tiny_skia::Pixmap::new(png.width(), png.height()).unwrap();
    pixels.fill(resvg::tiny_skia::Color::WHITE);
    resvg::render(&tree, resvg::tiny_skia::Transform::identity(), &mut pixels.as_mut());
    let sum: u64 = png
        .pixels()
        .zip(pixels.pixels())
        .map(|(a, b)| u64::from(a.0[0].abs_diff(b.red())) + u64::from(a.0[1].abs_diff(b.green())) + u64::from(a.0[2].abs_diff(b.blue())))
        .sum();
    let mad = sum as f64 / (f64::from(png.width()) * f64::from(png.height()) * 3.);
    assert!(mad <= 2., "mixed bidi MAD={mad}");
}

#[test]
fn editable_metric_twins_are_resolved_without_test_aliases() {
    for (requested, expected) in
        [("Calibri", "Carlito"), ("Cambria", "Caladea"), ("Arial", "Arimo"), ("Times New Roman", "Tinos"), ("Courier New", "Cousine")]
    {
        for (bold, italic) in [(false, false), (true, false), (false, true)] {
            let mut body = TextBody::default();
            body.paragraphs.push(Paragraph::new("Bright copper signals"));
            body.paragraphs[0].runs[0].props =
                RunProps { font: Some(requested.into()), bold: Some(bold), italic: Some(italic), size: Some(18.), ..Default::default() };
            let s = scene(&body);
            let run = s.ops.iter().find_map(|op| if let Op::Text { layout, .. } = op { layout.runs.first() } else { None }).unwrap();
            assert_eq!(run.face.family, expected);
            let xml = xml(&s);
            let doc = resvg::usvg::roxmltree::Document::parse(&xml).unwrap();
            for n in doc.descendants().filter(|n| n.has_tag_name("text") || n.has_tag_name("tspan")) {
                assert_eq!(n.attribute("font-family"), Some(format!("'{requested}', '{expected}'").as_str()));
            }
            let mut options = resvg::usvg::Options::default();
            for bytes in misterdiagram_fonts::bundled() {
                options.fontdb_mut().load_font_data(bytes.to_vec());
            }
            let tree = resvg::usvg::Tree::from_str(&xml, &options).unwrap();
            assert!(!tree.root().children().is_empty());
            assert!(!svg::render(&s, Options { text_outlines: true, ..Default::default() }).unwrap().contains("font-family"));
        }
    }
}

/// Absolute baselines survive empty SVG text chunks, including trailing paragraphs.
/// https://www.w3.org/TR/SVG11/text.html#TSpanElement
#[test]
fn empty_paragraphs_keep_every_visible_svg_baseline() {
    use misterdiagram_model::text::Anchor;
    use misterdiagram_render::raster;
    for anchor in [Anchor::Top, Anchor::Middle, Anchor::Bottom] {
        for value in ["\n\nAlpha", "Alpha\n\nBeta", "Alpha\n\n", "\n\nAlpha\n\nBeta\n\n"] {
            let body = TextBody {
                body: BodyProps { anchor: Some(anchor), ..Default::default() },
                paragraphs: value
                    .split('\n')
                    .map(|text| {
                        let mut p = Paragraph::new(text);
                        p.end_props = RunProps { font: Some("Calibri".into()), size: Some(12.), ..Default::default() };
                        for run in &mut p.runs {
                            run.props = p.end_props.clone();
                        }
                        p
                    })
                    .collect(),
                ..Default::default()
            };
            let scene = scene(&body);
            let Op::Text { layout, .. } = &scene.ops[0] else { panic!("text scene") };
            let xml = xml(&scene);
            let doc = resvg::usvg::roxmltree::Document::parse(&xml).unwrap();
            let text = doc.descendants().find(|n| n.has_tag_name("text")).unwrap();
            for run in &layout.runs {
                if run.svg.text.is_empty() {
                    continue;
                }
                let node = doc.descendants().find(|n| n.is_text() && n.text() == Some(run.svg.text.as_str())).unwrap();
                let line = if run.svg.line == 0 { text } else { node.parent().unwrap().parent().unwrap() };
                let baseline: f64 = line.attribute("y").expect("each line has an absolute baseline").parse().unwrap();
                assert!((baseline - layout.lines[run.svg.line].baseline).abs() < 1e-9);
            }
            let png = raster::render(&scene, Options::default()).unwrap();
            let tree = resvg::usvg::Tree::from_str(&xml, &svg_fonts::options()).unwrap();
            let mut pix = resvg::tiny_skia::Pixmap::new(png.width(), png.height()).unwrap();
            pix.fill(resvg::tiny_skia::Color::WHITE);
            resvg::render(&tree, resvg::tiny_skia::Transform::identity(), &mut pix.as_mut());
            let ink_rows = |bytes: &[u8]| {
                let mut rows = vec![];
                for (y, row) in bytes.chunks(png.width() as usize * 4).enumerate() {
                    if row.as_chunks::<4>().0.iter().any(|p| p[3] > 127 && p[0] < 128) {
                        rows.push(y);
                    }
                }
                rows
            };
            let a = ink_rows(png.as_raw());
            let b = ink_rows(pix.data());
            assert!(!a.is_empty() && !b.is_empty());
            for (from, to) in [(&a, &b), (&b, &a)] {
                assert!(from.iter().all(|y| to.iter().any(|z| y.abs_diff(*z) <= 1)), "{anchor:?} {value:?}: PNG {a:?}, SVG {b:?}");
            }
        }
    }
}
