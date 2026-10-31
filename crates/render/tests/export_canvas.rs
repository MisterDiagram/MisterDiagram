//! Contributor-original page canvas controls.
//! Native witness: field/export-canvas-2026-10-31.
//! https://learn.microsoft.com/en-us/office/vba/api/visio.page.export
//! https://learn.microsoft.com/en-us/office/vba/api/visio.document.exportasfixedformat
use hayro_syntax::Pdf;
use kurbo::{Affine, Rect, Shape as _};
use misterdiagram_render::{
    pdf,
    scene::{Op, Options, Paint, Scene},
    svg,
};
fn drawing(width: f64, height: f64) -> Scene {
    Scene {
        warnings: vec![],
        pdf_strokes: Default::default(),
        page_size: (width, height),
        bounds: Rect::new(0.75, 1.75, 5.5, 4.5),
        ops: vec![
            Op::Path { path: Rect::new(0.75, 1.75, 1.75, 2.25).to_path(1e-5), paint: Paint::Solid(misterdiagram_color::Rgba::BLACK) },
            Op::Path { path: Rect::new(4.5, 3.5, 5.5, 4.5).to_path(1e-5), paint: Paint::Solid(misterdiagram_color::Rgba::BLACK) },
        ],
    }
}
#[test]
fn svg_page_size_viewbox_coordinates_transparency_and_overflow() {
    for scale in [1., 2.] {
        let s = drawing(4., 6.);
        let o = Options { page_canvas: true, scale, ..Default::default() };
        let xml = svg::render(&s, o).unwrap();
        let doc = resvg::usvg::roxmltree::Document::parse(&xml).unwrap();
        let root = doc.root_element();
        assert_eq!(root.attribute("width"), Some("4in"));
        assert_eq!(root.attribute("height"), Some("6in"));
        assert_eq!(root.attribute("viewBox"), Some(format!("0 0 {} {}", 288., 432.).as_str()));
        assert_eq!(root.attribute("overflow"), Some("visible"));
        assert_eq!(doc.descendants().filter(|n| n.has_tag_name("path")).count(), 2);
        assert_eq!(doc.descendants().filter(|n| n.has_tag_name("rect")).count(), 0);
        let (_, _, view) = s.page_canvas(o).unwrap();
        assert_eq!(view, Affine::new([96. * scale, 0., 0., -96. * scale, 0., 576. * scale]));
        let paths: Vec<_> = doc.descendants().filter_map(|n| n.attribute("d")).collect();
        assert_eq!(paths[0], (Affine::scale(72. / (96. * scale)) * view * Rect::new(0.75, 1.75, 1.75, 2.25).to_path(1e-5)).to_svg());
    }
}
#[test]
fn pdf_each_page_uses_native_physical_canvas_independent_of_scale() {
    for scale in [1., 2.] {
        let o = Options { page_canvas: true, scale, ..Default::default() };
        let bytes = pdf::render(&[drawing(4., 6.), drawing(6., 4.), drawing(72.49 / 72., 108.49 / 72.)], o).unwrap();
        let parsed = Pdf::new(bytes).unwrap();
        assert_eq!(parsed.pages().len(), 3);
        for (page, expected) in parsed.pages().iter().zip([(288., 432.), (432., 288.), (72.375, 108.375)]) {
            let b = page.media_box();
            assert!((b.width() - expected.0).abs() < 1e-4);
            assert!((b.height() - expected.1).abs() < 1e-4);
        }
    }
}
#[test]
fn vector_page_canvas_preserves_hostile_scene_validation() {
    let o = Options { page_canvas: true, ..Default::default() };
    for value in [f64::NAN, f64::INFINITY, -1., 0., 1e12] {
        for size in [(value, 6.), (4., value)] {
            let s = drawing(size.0, size.1);
            assert!(svg::render(&s, o).is_err());
            assert!(pdf::render(&[s], o).is_err());
        }
    }
    let mut s = drawing(4., 6.);
    s.ops.push(Op::Pop);
    assert!(svg::render(&s, o).is_err());
    assert!(pdf::render(&[s], o).is_err());
    for scale in [f64::NAN, 0., -1., 101.] {
        let o = Options { scale, ..o };
        let s = drawing(4., 6.);
        assert!(svg::render(&s, o).is_err());
        assert!(pdf::render(&[s], o).is_err());
    }
}
