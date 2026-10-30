//! Contributor-original TextBkgnd cases measured on Visio build 20527.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/textbkgnd-cell-text-block-format-section
//! https://learn.microsoft.com/en-us/office/client-developer/visio/textbkgndtrans-cell-text-block-format-section
use misterdiagram_color::Rgba;
use misterdiagram_geom::{Affine, Rect};
use misterdiagram_model::{Cell, Document, Row, Section, Shape, skeleton};
use misterdiagram_render::{
    pdf, raster,
    scene::{self, Op},
    svg,
};

fn set(shape: &mut Shape, name: &str, formula: &str) {
    shape.sheet.cells.retain(|c| c.name != name);
    shape.sheet.cells.push(Cell { name: name.into(), formula: Some(formula.into()), ..Default::default() });
}
fn document(text: &str, background: &str) -> Document {
    let mut d = skeleton::document(false);
    d.face_names.insert(99, "Arial".into());
    let mut s = skeleton::rectangle(1, 1., 1., 3., 1., text).unwrap();
    for (name, formula) in [("TextBkgnd", background), ("FillPattern", "0")] {
        set(&mut s, name, formula);
    }
    s.sheet.sections.push(Section {
        name: "Character".into(),
        rows: vec![Row {
            index: Some(0),
            cells: vec![
                Cell { name: "Font".into(), value: Some("99".into()), ..Default::default() },
                Cell { name: "Size".into(), formula: Some("12 pt".into()), ..Default::default() },
            ],
            ..Default::default()
        }],
        ..Default::default()
    });
    d.pages[0].shapes.push(s);
    d
}
fn backgrounds(s: &scene::Scene) -> Vec<(Rect, Affine, Rgba)> {
    s.ops
        .iter()
        .filter_map(|op| if let Op::TextBackground { rect, transform, color } = op { Some((*rect, *transform, *color)) } else { None })
        .collect()
}
fn draw(d: &Document) -> scene::Scene {
    scene::page(d, 0, scene::Options::default()).unwrap()
}

#[test]
fn palette_transparency_and_runtime_custom_colors() {
    for n in 1..=24 {
        let s = draw(&document("Alpha beta", &n.to_string()));
        let bg = backgrounds(&s);
        assert_eq!(bg.len(), 1);
        let [r, g, b] = misterdiagram_sheet::functions::default_color(n - 1).unwrap();
        assert_eq!(bg[0].2, Rgba::rgb(r, g, b));
    }
    for n in ["0", "255", "-1", "0.5", "4294967296"] {
        assert!(backgrounds(&draw(&document("Alpha beta", n))).is_empty(), "{n}");
    }
    let mut d = document("Alpha beta", "RGB(40,120,200)+1");
    d.colors.clear();
    assert_eq!(backgrounds(&draw(&d))[0].2, Rgba::rgb(40, 120, 200));
    set(&mut d.pages[0].shapes[0], "TextBkgndTrans", "50%");
    assert_eq!(backgrounds(&draw(&d))[0].2, Rgba::rgba(40, 120, 200, 128));
    set(&mut d.pages[0].shapes[0], "TextBkgnd", "THEMEGUARD(THEMEVAL(\"BackgroundColor\")+1)");
    assert_eq!(backgrounds(&draw(&d))[0].2, Rgba::rgba(255, 255, 255, 128));
    set(&mut d.pages[0].shapes[0], "TextBkgnd", "27");
    d.colors.insert(26, "#123456".into());
    assert_eq!(backgrounds(&draw(&d))[0].2, Rgba::rgba(18, 52, 86, 128));
}

#[test]
fn native_rectangles_follow_lines_instead_of_shape_or_margins() {
    // Native local point rectangles from original field/textbkgnd-2026-10-30.
    for (text, margins, narrow, expected) in [
        ("Alpha beta", false, false, Rect::new(79.3092, 28.8, 136.6911, 43.2001)),
        ("Alpha\nBeta gamma\nDelta", false, false, Rect::new(73.983, 14.4, 142.0172, 57.6002)),
        ("Alpha beta", true, false, Rect::new(90.109, 34.2, 147.4909, 48.6001)),
        ("Alpha beta gamma delta", false, true, Rect::new(12.3929, 7.2, 52.407, 64.8002)),
    ] {
        let mut d = document(text, "3");
        let shape = &mut d.pages[0].shapes[0];
        if margins {
            for (n, v) in [("LeftMargin", "0.4 in"), ("RightMargin", "0.1 in"), ("TopMargin", "0.2 in"), ("BottomMargin", "0.05 in")] {
                set(shape, n, v);
            }
        }
        if narrow {
            set(shape, "TxtWidth", "0.9 in");
        }
        let s = draw(&d);
        let bg = backgrounds(&s);
        assert_eq!(bg.len(), 1);
        let actual = bg[0].0;
        for (a, b) in [actual.x0, actual.y0, actual.x1, actual.y1].into_iter().zip([expected.x0, expected.y0, expected.x1, expected.y1]) {
            assert!((a - b).abs() <= 0.5, "{text}: {actual:?} != {expected:?}");
        }
    }
}

#[test]
fn background_is_above_geometry_below_text_and_remains_an_svg_rectangle() {
    let d = document("Alpha beta", "3");
    let s = draw(&d);
    let i = s.ops.iter().position(|op| matches!(op, Op::TextBackground { .. })).unwrap();
    assert!(s.ops[..i].iter().any(|op| matches!(op, Op::Stroke { .. } | Op::HairlineStroke { .. })));
    assert!(matches!(s.ops[i + 1], Op::Text { .. }));
    for outlines in [false, true] {
        let xml = svg::render(&s, scene::Options { text_outlines: outlines, ..Default::default() }).unwrap();
        assert_eq!(xml.matches("<rect data-text-background=").count(), 1);
        assert!(xml.contains("fill=\"#ff0000\""));
    }
    let image = raster::render(&s, scene::Options::default()).unwrap();
    assert!(image.pixels().filter(|p| p.0 == [255, 0, 0, 255]).count() > 400);
    let bytes = pdf::render(&[s], scene::Options::default()).unwrap();
    assert!(bytes.starts_with(b"%PDF"));
    for text in ["", "Alpha beta"] {
        let mut d = document(text, "3");
        set(&mut d.pages[0].shapes[0], "HideText", "1");
        assert!(backgrounds(&draw(&d)).is_empty());
    }
    assert!(backgrounds(&draw(&document("", "3"))).is_empty());
}

#[test]
fn background_shares_text_transform_for_flips_rotation_and_scale() {
    for (x, y) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
        let mut d = document("Alpha beta", "3");
        let shape = &mut d.pages[0].shapes[0];
        set(shape, "FlipX", &x.to_string());
        set(shape, "FlipY", &y.to_string());
        set(shape, "TxtAngle", "25 deg");
        for c in &mut d.pages[0].sheet.cells {
            if c.name == "DrawingScale" {
                c.value = Some("2".into());
                c.formula = None;
            }
        }
        let s = draw(&d);
        let bg = backgrounds(&s);
        assert_eq!(bg.len(), 1);
        let text = s.ops.iter().find_map(|op| if let Op::Text { transform, .. } = op { Some(*transform) } else { None }).unwrap();
        assert_eq!(bg[0].1, text);
        let b = text.transform_rect_bbox(bg[0].0);
        assert!(s.bounds.x0 <= b.x0 && s.bounds.y0 <= b.y0 && s.bounds.x1 >= b.x1 && s.bounds.y1 >= b.y1);
    }
}

#[test]
fn hostile_background_coordinates_are_rejected_by_every_backend() {
    for bad in [f64::NAN, f64::INFINITY, 1e9] {
        let s = scene::Scene {
            warnings: vec![],
            ops: vec![Op::TextBackground { rect: Rect::new(bad, 0., 1., 1.), transform: Affine::IDENTITY, color: Rgba::BLACK }],
            bounds: Rect::new(0., 0., 1., 1.),
            page_size: (1., 1.),
        };
        let o = scene::Options::default();
        assert!(raster::render(&s, o).is_err());
        assert!(svg::render(&s, o).is_err());
        assert!(pdf::render(&[s], o).is_err());
    }
}
