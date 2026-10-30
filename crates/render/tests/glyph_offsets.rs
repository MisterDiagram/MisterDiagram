//! Contributor-original combining-mark and semantic baseline regression for #54.
#[path = "support/svg_fonts.rs"]
mod svg_fonts;
use hayro_syntax::{Pdf, content::ops::TypedInstruction};
use misterdiagram_model::{Cell, skeleton};
use misterdiagram_render::{pdf, raster, scene, svg};
fn marks() -> scene::Scene {
    let mut d = skeleton::document(false);
    let mut s = skeleton::rectangle(1, 1., 1., 2., 1., "\u{0628}\u{0650}\u{0651}").unwrap();
    for n in ["FillPattern", "LinePattern"] {
        s.sheet.cells.push(Cell { name: n.into(), value: Some("0".into()), ..Default::default() });
    }
    d.pages[0].shapes.push(s);
    scene::page(&d, 0, options()).unwrap()
}
fn options() -> scene::Options {
    scene::Options { scale: 2., crop: false, ..Default::default() }
}
#[test]
fn pdf_glyph_matrices_follow_y_down_positions_with_a_leading_mark() {
    // The positioned marks come from the bundled Arabic face; builds without craft-fonts lack it.
    if misterdiagram_fonts::bundled_families(misterdiagram_fonts::script::ARABIC).is_empty() {
        eprintln!("skipped: no bundled Arabic face (built without craft-fonts)");
        return;
    }
    let s = marks();
    let run = s.ops.iter().find_map(|op| if let scene::Op::Text { layout, .. } = op { layout.runs.first() } else { None }).unwrap();
    assert_eq!(run.glyphs.len(), 3);
    let y0 = run.glyphs[0].2;
    let expected: Vec<_> = run.glyphs.iter().map(|g| g.2 - y0).collect();
    assert!(expected[1] < -1. && expected[2] < -1., "fixture must start with a vertically offset mark");
    let bytes = pdf::render(&[s], options()).unwrap();
    let parsed = Pdf::new(bytes).unwrap();
    let mut matrices = vec![];
    let mut ops = parsed.pages()[0].typed_operations();
    while let Some(op) = ops.next() {
        if let TypedInstruction::SetTextMatrix(m) = op {
            matrices.push(m.5.as_f64());
        }
    }
    assert_eq!(matrices.len(), expected.len());
    for (a, b) in matrices.iter().zip(expected) {
        assert!((a - b).abs() < 1e-4, "PDF actual text matrices {matrices:?}, expected y={b}");
    }
}
fn rows(im: &image::RgbaImage) -> (u32, u32) {
    let ys: Vec<_> = im.enumerate_pixels().filter(|(_, _, p)| p.0[3] > 127 && p.0[..3].iter().all(|v| *v < 128)).map(|(_, y, _)| y).collect();
    (*ys.iter().min().unwrap(), *ys.iter().max().unwrap())
}
#[test]
fn editable_svg_does_not_apply_the_first_marks_offset_to_the_whole_run() {
    let s = marks();
    let png = raster::render(&s, options()).unwrap();
    let xml = svg::render(&s, options()).unwrap();
    assert!(xml.contains("baseline-shift=\"0\""));
    let tree = resvg::usvg::Tree::from_str(&xml, &svg_fonts::options()).unwrap();
    let mut pixels = resvg::tiny_skia::Pixmap::new(png.width(), png.height()).unwrap();
    pixels.fill(resvg::tiny_skia::Color::WHITE);
    resvg::render(&tree, resvg::tiny_skia::Transform::identity(), &mut pixels.as_mut());
    let actual = image::RgbaImage::from_raw(png.width(), png.height(), pixels.data().to_vec()).unwrap();
    let (a, b) = (rows(&png), rows(&actual));
    assert!(a.0.abs_diff(b.0) <= 1 && a.1.abs_diff(b.1) <= 1, "PNG{a:?} SVG{b:?}");
}
#[test]
fn svg_retains_semantic_super_and_subscript_baseline_shifts() {
    use misterdiagram_model::text::{Paragraph, TextBody};
    use misterdiagram_text::{NoFields, Opts};
    for baseline in [-0.3, 0.3] {
        let mut p = Paragraph::new("Alpha");
        p.runs[0].props.baseline = Some(baseline);
        p.runs[0].props.size = Some(12.);
        let body = TextBody { paragraphs: vec![p], ..Default::default() };
        let l = misterdiagram_text::layout(
            &body,
            &Opts { rect: kurbo::Rect::new(0., 0., 144., 72.), fields: &NoFields, prompt_color: None, no_shrink: true },
        )
        .unwrap();
        let r = &l.runs[0];
        assert!((r.svg.baseline_shift - baseline * 12.).abs() < 1e-9);
        let line = &l.lines[0];
        assert!((line.baseline - r.glyphs[0].2 - r.svg.baseline_shift).abs() < 1e-9);
    }
}
#[test]
fn pdf_preserves_equal_and_both_signed_vertical_positions() {
    for shifts in [[0., 0., 0.], [0., -3., 2.]] {
        let mut s = marks();
        for op in &mut s.ops {
            if let scene::Op::Text { layout, .. } = op {
                let r = &mut layout.runs[0];
                let y0 = r.glyphs[0].2;
                for (glyph, dy) in r.glyphs.iter_mut().zip(shifts) {
                    glyph.2 = y0 + dy;
                }
            }
        }
        let bytes = pdf::render(&[s], options()).unwrap();
        let parsed = Pdf::new(bytes).unwrap();
        let mut actual = vec![];
        let mut ops = parsed.pages()[0].typed_operations();
        while let Some(op) = ops.next() {
            if let TypedInstruction::SetTextMatrix(m) = op {
                actual.push(m.5.as_f64());
            }
        }
        let expected = if shifts == [0., 0., 0.] { vec![0.] } else { shifts.to_vec() };
        assert_eq!(actual.len(), expected.len());
        for (a, b) in actual.iter().zip(expected) {
            assert!((a - b).abs() < 1e-4, "actual {actual:?}, expected {b}");
        }
    }
}
#[test]
fn nonfinite_semantic_baseline_metadata_is_rejected() {
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let mut s = marks();
        for op in &mut s.ops {
            if let scene::Op::Text { layout, .. } = op {
                layout.runs[0].svg.baseline_shift = bad;
            }
        }
        assert!(svg::render(&s, options()).is_err());
        assert!(raster::render(&s, options()).is_err());
        assert!(pdf::render(&[s], options()).is_err());
    }
}
