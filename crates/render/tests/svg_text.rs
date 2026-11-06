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
        fit_empty: false,
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

#[test]
fn round_bullets_use_raster_outlines_and_keep_text_gaps() {
    use misterdiagram_model::text::{Align, Bullet};
    use misterdiagram_render::{raster, scene::text_paths};
    for (align, rtl) in [(Align::Left, false), (Align::Center, false), (Align::Right, false), (Align::Right, true)] {
        let mut body = TextBody::default();
        for value in ["First item", "Second item", "Third item"] {
            let mut p = Paragraph::new(value);
            p.props.bullet = Some(Bullet::Char { char: "•".into() });
            p.props.bullet_tab = Some(24.);
            p.props.bullet_size = Some(1.5);
            p.props.bullet_color = Some(misterdiagram_color::Rgba { r: 42, g: 80, b: 130, a: 127 });
            p.props.align = Some(align);
            p.props.rtl = Some(rtl);
            body.paragraphs.push(p);
        }
        let mut s = scene(&body);
        let Op::Text { layout, transform, .. } = &mut s.ops[0] else { panic!("text scene") };
        *transform = Affine::rotate(0.2) * *transform;
        for run in layout.runs.iter_mut().filter(|r| r.cells.is_empty()) {
            run.alpha = 0.6;
        }
        let before = raster::render(&s, Options::default()).unwrap();
        let xml = xml(&s);
        assert_eq!(xml.matches('•').count(), 0);
        assert_eq!(xml.matches("<text ").count(), 1);
        let doc = resvg::usvg::roxmltree::Document::parse(&xml).unwrap();
        let paths: Vec<_> = doc.descendants().filter(|n| n.has_tag_name("path")).collect();
        assert_eq!(paths.len(), 3);
        let Op::Text { layout, transform, .. } = &s.ops[0] else { panic!("text scene") };
        let mut bullets = layout.clone();
        bullets.runs.retain(|r| r.cells.is_empty());
        bullets.decos.clear();
        let expected = text_paths(&bullets, s.canvas(Options::default()).unwrap().2 * *transform);
        for (node, (path, color)) in paths.iter().zip(expected) {
            assert_eq!(node.attribute("d"), Some(path.to_svg().as_str()));
            assert_eq!(node.attribute("fill"), Some("#2a5082"));
            assert_eq!(node.attribute("fill-opacity").unwrap().parse::<f64>().unwrap(), f64::from(color.a) / 255.);
            assert!(!node.ancestors().any(|n| n.has_tag_name("text")));
        }
        for run in layout.runs.iter().filter(|r| !r.cells.is_empty()) {
            let node = doc.descendants().find(|n| n.is_text() && n.text().is_some_and(|t| t.contains(run.svg.text.as_str()))).unwrap();
            let span = node.parent().unwrap();
            let line = span.parent().unwrap();
            let x: f64 = line.attribute("x").unwrap().parse().unwrap();
            let dx: f64 = span.attribute("dx").unwrap().parse().unwrap();
            let left = layout.lines[run.svg.line].edges.iter().map(|e| e.0).reduce(f64::min).unwrap();
            assert!((x + dx - left).abs() < 1e-9, "{align:?} {rtl}: {x} + {dx} != {left}");
        }
        assert_eq!(before, raster::render(&s, Options { font_names: misterdiagram_render::scene::FontNames::Exact, ..Default::default() }).unwrap());
    }
}

#[test]
fn custom_and_numbered_bullets_and_literal_round_text_stay_text() {
    use misterdiagram_model::text::Bullet;
    let mut body = TextBody::default();
    for bullet in
        [Bullet::Char { char: "◆".into() }, Bullet::Char { char: "••".into() }, Bullet::AutoNum { scheme: "arabicPeriod".into(), start_at: 1 }]
    {
        let mut p = Paragraph::new("literal • remains");
        p.props.bullet = Some(bullet);
        body.paragraphs.push(p);
    }
    let xml = xml(&scene(&body));
    assert!(!xml.contains("<path"));
    assert!(xml.contains('◆'));
    assert!(xml.contains("••"));
    assert!(xml.contains("1."));
    assert_eq!(xml.matches("literal • remains").count(), 3);
}

#[test]
fn wrapped_bullet_has_one_outline_and_continuations_stay_text() {
    use misterdiagram_model::text::Bullet;
    let mut body = TextBody::default();
    let mut p = Paragraph::new(&"A long original item with several words ".repeat(5));
    p.props.bullet = Some(Bullet::Char { char: "•".into() });
    body.paragraphs.push(p);
    let s = scene(&body);
    let Op::Text { layout, .. } = &s.ops[0] else { panic!("text scene") };
    assert!(layout.lines.len() >= 3);
    let xml = xml(&s);
    assert_eq!(xml.matches("<path").count(), 1);
    assert_eq!(xml.matches("<text ").count(), 1);
    assert!(!xml.contains('•'));
}
