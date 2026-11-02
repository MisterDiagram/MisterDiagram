//! Contributor-original section topology from field/flip-fill-2026-11-01.
//! Section exclusion is documented; cross-section alternate filling is witnessed.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/geometry-section
//! https://learn.microsoft.com/en-us/office/client-developer/visio/nofill-cell-geometry-section
use kurbo::Shape as _;
use misterdiagram_color::Rgba;
use misterdiagram_geom::{Point, Rect};
use misterdiagram_model::{Cell, Document, Row, Section, Shape, skeleton};
use misterdiagram_render::{
    raster,
    scene::{self, Op, Paint},
    svg,
};

fn set(s: &mut Shape, n: &str, v: &str) {
    s.sheet.cells.retain(|c| c.name != n);
    s.sheet.cells.push(Cell { name: n.into(), value: Some(v.into()), ..Default::default() });
}
fn section(index: u32, points: &[(f64, f64)], closed: bool, no_fill: bool) -> Section {
    let mut rows = Vec::new();
    let mut points = points.to_vec();
    if closed {
        points.push(points[0]);
    }
    for (i, (x, y)) in points.into_iter().enumerate() {
        rows.push(Row {
            index: Some(i as u32 + 1),
            row_type: Some(if i == 0 { "MoveTo" } else { "LineTo" }.into()),
            cells: vec![
                Cell { name: "X".into(), value: Some(x.to_string()), ..Default::default() },
                Cell { name: "Y".into(), value: Some(y.to_string()), ..Default::default() },
            ],
            ..Default::default()
        });
    }
    Section {
        name: "Geometry".into(),
        index: Some(index),
        cells: vec![Cell { name: "NoFill".into(), value: Some(if no_fill { "1" } else { "0" }.into()), ..Default::default() }],
        rows,
        ..Default::default()
    }
}
fn drawing(reverse: bool, third: bool, inner_no_fill: bool, pattern: u32) -> Document {
    let mut d = skeleton::document(false);
    let mut bg = skeleton::rectangle(1, 0., 0., 8., 6., "").unwrap();
    set(&mut bg, "FillForegnd", "#d8efe6");
    set(&mut bg, "LinePattern", "0");
    let mut s = skeleton::rectangle(2, 1., 1., 3., 3., "").unwrap();
    set(&mut s, "FillForegnd", "#426fa0");
    set(&mut s, "FillBkgnd", "#f5d596");
    set(&mut s, "FillPattern", &pattern.to_string());
    set(&mut s, "LinePattern", "1");
    set(&mut s, "LineWeight", "0.013888888888888888");
    let mut inner = vec![(0.75, 0.75), (2.25, 0.75), (2.25, 2.25), (0.75, 2.25)];
    if reverse {
        inner.reverse();
    }
    s.sheet.sections.retain(|s| s.name != "Geometry");
    s.sheet.sections.push(section(0, &[(0., 0.), (3., 0.), (3., 3.), (0., 3.)], true, false));
    s.sheet.sections.push(section(1, &inner, true, inner_no_fill));
    if third {
        s.sheet.sections.push(section(2, &[(1.1, 1.1), (1.9, 1.1), (1.9, 1.9), (1.1, 1.9)], true, false));
    }
    d.pages[0].shapes = vec![bg, s].into();
    d
}
fn options() -> scene::Options {
    scene::Options { crop: false, scale: 2., ..Default::default() }
}
fn images(s: &scene::Scene) -> Vec<image::RgbaImage> {
    let opts = options();
    let png = raster::render(s, opts).unwrap();
    let xml = svg::render(s, opts).unwrap();
    let tree = resvg::usvg::Tree::from_str(&xml, &resvg::usvg::Options { dpi: 192., ..Default::default() }).unwrap();
    let mut px = resvg::tiny_skia::Pixmap::new(png.width(), png.height()).unwrap();
    resvg::render(&tree, resvg::tiny_skia::Transform::identity(), &mut px.as_mut());
    let svg = image::RgbaImage::from_fn(px.width(), px.height(), |x, y| {
        let p = px.pixel(x, y).unwrap().demultiply();
        image::Rgba([p.red(), p.green(), p.blue(), p.alpha()])
    });
    vec![png, svg]
}
fn sample(im: &image::RgbaImage, s: &scene::Scene, x: f64, y: f64) -> [u8; 3] {
    let (_, _, view) = s.canvas(options()).unwrap();
    let p = view * Point::new(x, y);
    let q = im.get_pixel(p.x.round() as u32, p.y.round() as u32);
    [q[0], q[1], q[2]]
}
#[test]
fn nested_sections_use_alternate_fill_independent_of_winding() {
    for reverse in [false, true] {
        for third in [false, true] {
            for no_fill in [false, true] {
                let d = drawing(reverse, third, no_fill, 1);
                let s = scene::page(&d, 0, options()).unwrap();
                for im in images(&s) {
                    assert_eq!(sample(&im, &s, 1.3, 1.3), [66, 111, 160]);
                    let hole = if no_fill { [66, 111, 160] } else { [216, 239, 230] };
                    assert_eq!(sample(&im, &s, 1.95, 1.95), hole);
                    let center = if third ^ no_fill { [66, 111, 160] } else { [216, 239, 230] };
                    assert_eq!(sample(&im, &s, 2.5, 2.5), center);
                }
                let xml = svg::render(&s, options()).unwrap();
                assert!(xml.contains("fill-rule=\"evenodd\""));
            }
        }
    }
}
#[test]
fn patterns_and_gradients_clip_out_nested_holes() {
    for pattern in [2, 25] {
        let s = scene::page(&drawing(false, false, false, pattern), 0, options()).unwrap();
        for im in images(&s) {
            assert_eq!(sample(&im, &s, 2.5, 2.5), [216, 239, 230]);
            assert_ne!(sample(&im, &s, 1.3, 1.3), [216, 239, 230]);
        }
        assert!(svg::render(&s, options()).unwrap().contains("clip-rule=\"evenodd\""));
    }
}
#[test]
fn overlap_open_contours_and_excluded_sections_follow_fill_eligibility() {
    let mut d = drawing(false, false, false, 1);
    d.pages[0].shapes[1].sheet.sections.retain(|s| s.name != "Geometry");
    d.pages[0].shapes[1].sheet.sections.extend([
        section(0, &[(0., 0.), (2., 0.), (2., 2.), (0., 2.)], true, false),
        section(1, &[(1., 1.), (3., 1.), (3., 3.), (1., 3.)], true, false),
    ]);
    let s = scene::page(&d, 0, options()).unwrap();
    for im in images(&s) {
        assert_eq!(sample(&im, &s, 2.5, 2.5), [216, 239, 230]);
        assert_eq!(sample(&im, &s, 1.5, 1.5), [66, 111, 160]);
        assert_eq!(sample(&im, &s, 3.5, 3.5), [66, 111, 160]);
    }
    d.pages[0].shapes[1].sheet.sections.retain(|s| s.name != "Geometry");
    d.pages[0].shapes[1]
        .sheet
        .sections
        .extend([section(0, &[(0., 0.), (1., 0.), (0., 1.)], false, false), section(1, &[(1.5, 0.), (3., 0.), (3., 1.5), (1.5, 1.5)], true, false)]);
    let s = scene::page(&d, 0, options()).unwrap();
    for im in images(&s) {
        assert_eq!(sample(&im, &s, 1.2, 1.2), [66, 111, 160]);
        assert_eq!(sample(&im, &s, 1.8, 1.8), [216, 239, 230]);
        assert_eq!(sample(&im, &s, 3., 2.), [66, 111, 160]);
    }
}
#[test]
fn alternate_fill_does_not_leak_into_following_nonzero_paths_or_strokes() {
    let d = drawing(false, false, false, 1);
    let mut s = scene::page(&d, 0, options()).unwrap();
    let mut path = Rect::new(4.5, 1., 6., 2.5).to_path(1e-5);
    path.extend(Rect::new(5., 1.5, 6.5, 3.).to_path(1e-5).elements().iter().copied());
    s.ops.push(Op::Path { path, paint: Paint::Solid(Rgba::BLACK), even_odd: false });
    for im in images(&s) {
        assert_eq!(sample(&im, &s, 5.5, 2.), [0, 0, 0]);
    }
    let strokes = |s: &scene::Scene| {
        s.ops
            .iter()
            .filter_map(|op| match op {
                Op::Stroke { path, stroke, color } | Op::HairlineStroke { path, stroke, color, .. } => Some(format!("{path:?}/{stroke:?}/{color:?}")),
                _ => None,
            })
            .collect::<Vec<_>>()
    };
    let mut no_fill = d;
    set(&mut no_fill.pages[0].shapes[1], "FillPattern", "0");
    let control = scene::page(&no_fill, 0, options()).unwrap();
    assert_eq!(strokes(&s), strokes(&control));
}
#[test]
fn shadow_unions_alternate_fill_with_strokes_before_blur() {
    for blur in [0., 0.08] {
        for lines in [false, true] {
            let mut d = drawing(false, false, false, 1);
            let shape = &mut d.pages[0].shapes[1];
            for (n, v) in [
                ("ShdwPattern", "1"),
                ("ShapeShdwType", "1"),
                ("ShapeShdwOffsetX", "2"),
                ("ShapeShdwOffsetY", "0"),
                ("ShdwForegnd", "#501e6e"),
                ("ShdwForegndTrans", "0"),
                ("LineWeight", "0.2"),
            ] {
                set(shape, n, v);
            }
            set(shape, "ShapeShdwBlur", &blur.to_string());
            if !lines {
                set(shape, "LinePattern", "0");
            }
            let s = scene::page(&d, 0, options()).unwrap();
            for im in images(&s) {
                assert_eq!(sample(&im, &s, 4.5, 2.5), [216, 239, 230], "hole, blur {blur}, lines {lines}");
                assert_eq!(sample(&im, &s, 5.7, 2.5), [80, 30, 110]);
                if lines && blur == 0. {
                    assert_eq!(sample(&im, &s, 5.2, 2.5), [80, 30, 110], "stroke/fill overlap must not cancel");
                }
            }
        }
    }
}

#[test]
fn section_flags_independently_select_fill_and_stroke() {
    for flag in ["NoShow", "NoLine"] {
        let mut d = drawing(false, false, false, 1);
        let geometry = d.pages[0].shapes[1].sheet.sections.iter_mut().find(|s| s.name == "Geometry" && s.index == Some(1)).unwrap();
        geometry.cells.push(Cell { name: flag.into(), value: Some("1".into()), ..Default::default() });
        let s = scene::page(&d, 0, options()).unwrap();
        for im in images(&s) {
            assert_eq!(sample(&im, &s, 2.5, 2.5), if flag == "NoShow" { [66, 111, 160] } else { [216, 239, 230] });
        }
        assert_eq!(s.ops.iter().filter(|op| matches!(op, Op::Stroke { .. } | Op::HairlineStroke { .. })).count(), 1);
    }
    let mut d = drawing(false, false, false, 1);
    let outer = d.pages[0].shapes[1].sheet.sections.iter_mut().find(|s| s.name == "Geometry" && s.index == Some(0)).unwrap();
    outer.cells.iter_mut().find(|c| c.name == "NoFill").unwrap().value = Some("1".into());
    let s = scene::page(&d, 0, options()).unwrap();
    for im in images(&s) {
        assert_eq!(sample(&im, &s, 1.3, 1.3), [216, 239, 230]);
        assert_eq!(sample(&im, &s, 2.5, 2.5), [66, 111, 160]);
    }
}

/// Brief 63 CF-01: inspect the actual solid fill operation, not a clip or scene flag.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/geometry-section
#[test]
fn pdf_nested_closed_sections_use_even_odd_solid_fill() {
    use hayro_syntax::content::ops::TypedInstruction as T;
    for reverse in [false, true] {
        let mut d = drawing(reverse, false, false, 1);
        d.pages[0].shapes.remove(0);
        set(&mut d.pages[0].shapes[0], "LinePattern", "0");
        let s = scene::page(&d, 0, options()).unwrap();
        let bytes = misterdiagram_render::pdf::render(&[s], options()).unwrap();
        let pdf = hayro_syntax::Pdf::new(bytes).unwrap();
        let mut starts = 0;
        let mut alternate = 0;
        let mut operations = pdf.pages()[0].typed_operations();
        while let Some(op) = operations.next() {
            match op {
                T::MoveTo(_) | T::RectPath(_) => starts += 1,
                T::FillPathEvenOdd(_) => {
                    if starts == 2 {
                        alternate += 1;
                    }
                    starts = 0;
                }
                T::FillPathNonZero(_) => {
                    assert_ne!(starts, 2, "nested solid shape must use PDF f*, never f");
                    starts = 0;
                }
                T::EndPath(_) => starts = 0,
                _ => {}
            }
        }
        assert_eq!(alternate, 1, "PDF must fill the two nested closed subpaths with f*");
    }
}
