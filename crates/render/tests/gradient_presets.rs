use misterdiagram_color::Rgba;
use misterdiagram_geom::Point;
use misterdiagram_model::{Cell, Row, Section, skeleton};
use misterdiagram_render::{
    raster, scene,
    style::{Gradient, GradientGeometry},
};

fn gradient(direction: f64) -> Gradient {
    Gradient {
        stops: vec![(0., Rgba::WHITE), (1., Rgba::rgb(0, 0, 128))],
        geometry: GradientGeometry::from_direction(direction),
        angle: 0.,
        smooth: true,
    }
}

/// Square and 2:1 native controls distinguish physical circles from ellipses.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/fillgradientdir-cell-gradient-properties-section
#[test]
fn measured_presets_keep_type_anchor_and_physical_distance() {
    let anchors = [(1., 1.), (0., 1.), (0.5, 0.5), (0.5, 0.), (0.5, 1.), (1., 0.), (0., 0.), (1., 1.), (0., 1.)];
    for (index, (x, y)) in anchors.into_iter().enumerate() {
        let dir = index + 1;
        let g = gradient(dir as f64);
        assert_eq!(g.geometry.anchor(), Some(Point::new(x, y)));
        assert_eq!(matches!(g.geometry, GradientGeometry::Rectangle { .. }), dir >= 8);
        assert_eq!(g.sample_sized(x, y, 2., 1.), Rgba::WHITE);
        // Independent native interior samples, rounded to the box center.
        let expected = [188, 185, 255, 220, 223, 187, 184, 188, 185][index];
        assert!((i16::from(g.sample_sized(0.5, 0.5, 2., 1.).r) - expected).abs() <= 4, "direction {dir}");
    }
    let g = gradient(3.);
    assert!(g.sample_sized(0.5, 0.1, 2., 1.).r > g.sample_sized(0.1, 0.5, 2., 1.).r + 80);
    let corner = gradient(8.);
    assert_eq!(corner.sample_sized(0.3, 0.7, 2., 1.), corner.sample_sized(0.7, 0.3, 2., 1.));
}

/// The two-row path uses physical clockwise angles; theme slot trimming alone
/// must not change the established multirow linear rendering contract.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/fillgradientangle-cell-gradient-properties-section
#[test]
fn two_effective_rows_select_native_blend_without_changing_theme_slots() {
    let mut doc = skeleton::document(false);
    let mut shape = skeleton::rectangle(1, 1., 1., 2., 1., "").unwrap();
    shape.sheet.cells.push(Cell { name: "FillGradientEnabled".into(), value: Some("1".into()), ..Default::default() });
    shape.sheet.sections.push(Section {
        name: "FillGradient".into(),
        rows: (0..10)
            .map(|i| Row {
                index: Some(i),
                deleted: i >= 2,
                cells: if i < 2 {
                    [
                        ("GradientStopPosition", if i == 0 { "0" } else { "1" }),
                        ("GradientStopColor", if i == 0 { "#FFFFFF" } else { "#000080" }),
                        ("GradientStopColorTrans", "0"),
                    ]
                    .into_iter()
                    .map(|(name, value)| Cell { name: name.into(), value: Some(value.into()), ..Default::default() })
                    .collect()
                } else {
                    vec![]
                },
                ..Default::default()
            })
            .collect(),
        ..Default::default()
    });
    doc.pages[0].shapes.push(shape);
    let resolved = scene::page(&doc, 0, Default::default()).unwrap();
    let g = resolved
        .ops
        .iter()
        .find_map(|op| match op {
            scene::Op::Path { paint: scene::Paint::Pattern { gradient: Some(g), .. }, .. } => Some(g),
            _ => None,
        })
        .unwrap();
    assert!(g.smooth);
    assert_eq!(g.stops.len(), 2);
    let mut g = g.clone();
    g.geometry = GradientGeometry::Linear;
    g.angle = std::f64::consts::FRAC_PI_4;
    assert!(g.sample_sized(0.1, 0.1, 2., 1.).r > 240);
    assert!(g.sample_sized(0.9, 0.9, 2., 1.).r < 60);
    assert!(g.sample_sized(0.1, 0.9, 2., 1.).r > g.sample_sized(0.9, 0.1, 2., 1.).r);
    g.smooth = false;
    g.angle = 0.;
    assert_eq!(g.sample_sized(0.5, 0.5, 2., 1.), Rgba::rgb(128, 128, 192));
}

/// Hostile directions keep the bounded old fallback; malformed paint cannot
/// bypass validation through the radial branch.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/fillgradientdir-cell-gradient-properties-section
#[test]
fn malformed_gradient_geometry_and_stops_are_bounded() {
    for dir in [-1., 0.5, 10., 13., f64::MAX, f64::NAN, f64::INFINITY] {
        assert_eq!(gradient(dir).geometry, GradientGeometry::Circle { anchor: Point::new(1., 1.) });
    }
    let valid = gradient(3.);
    for anchor in [Point::new(f64::NAN, 0.5), Point::new(0.5, f64::INFINITY), Point::new(-1., 2.)] {
        let invalid = Gradient { geometry: GradientGeometry::Circle { anchor }, ..valid.clone() };
        assert!(!invalid.valid());
        assert_eq!(invalid.sample(0.5, 0.5), Rgba::TRANSPARENT);
    }
    for (w, h) in [(0., 1.), (-1., 1.), (f64::NAN, 1.), (1., f64::INFINITY)] {
        assert_eq!(valid.sample_sized(0.5, 0.5, w, h), Rgba::TRANSPARENT);
    }
    for stops in [vec![], vec![(f64::NAN, Rgba::WHITE)], vec![(0., Rgba::WHITE); 4097]] {
        let mut g = valid.clone();
        g.stops = stops;
        assert!(!g.valid());
        assert_eq!(g.sample(0.5, 0.5), Rgba::TRANSPARENT);
    }
    let mut doc = skeleton::document(false);
    doc.pages[0].shapes.push(skeleton::rectangle(1, 1., 1., 2., 1., "").unwrap());
    let mut rendered = scene::page(&doc, 0, Default::default()).unwrap();
    for op in &mut rendered.ops {
        if let scene::Op::Path { paint, .. } = op {
            *paint = scene::Paint::Pattern {
                fill: misterdiagram_render::style::Fill { pattern: 1, foreground: Rgba::WHITE, background: Rgba::BLACK },
                gradient: Some(Gradient { stops: vec![(f64::NAN, Rgba::WHITE)], ..valid.clone() }),
                bounds: kurbo::Rect::new(0., 0., 2., 1.),
                transform: kurbo::Affine::IDENTITY,
            };
        }
    }
    assert!(raster::render(&rendered, Default::default()).is_err());
}

/// The same transformed radial field is used in the bitmap and SVG backends.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/fillgradientdir-cell-gradient-properties-section
#[test]
fn svg_presets_match_raster_with_scale_rotation_and_nonuniform_group() {
    for direction in 1..=9 {
        let mut doc = skeleton::document(false);
        let mut group = skeleton::rectangle(1, 3., 3., 4., 2., "").unwrap();
        group.shape_type = Some("Group".into());
        let mut child = skeleton::rectangle(2, 1., 1., 2., 1., "").unwrap();
        for (name, value) in [("Angle", std::f64::consts::FRAC_PI_4.to_string()), ("FlipX", "1".into()), ("LinePattern", "0".into())] {
            child.sheet.cells.retain(|c| c.name != name);
            child.sheet.cells.push(Cell { name: name.into(), value: Some(value), ..Default::default() });
        }
        group.shapes.push(child);
        doc.pages[0].shapes.push(group);
        let opts = scene::Options { scale: 2., ..Default::default() };
        let child = &mut doc.pages[0].shapes[0].shapes[0];
        child.sheet.cells.push(Cell { name: "FillGradientEnabled".into(), value: Some("1".into()), ..Default::default() });
        child.sheet.cells.push(Cell { name: "FillGradientDir".into(), value: Some(direction.to_string()), ..Default::default() });
        let mut rendered = scene::page(&doc, 0, opts).unwrap();
        for op in &mut rendered.ops {
            if let scene::Op::Path { paint: scene::Paint::Pattern { gradient: g, .. }, .. } = op {
                *g = Some(gradient(f64::from(direction)));
            }
        }
        let stretch = kurbo::Affine::scale_non_uniform(1.25, 0.75);
        for op in &mut rendered.ops {
            if let scene::Op::Path { path, paint, .. } = op {
                *path = stretch * &*path;
                if let scene::Paint::Pattern { transform, .. } = paint {
                    *transform = stretch * *transform;
                }
            }
        }
        rendered.bounds = stretch.transform_rect_bbox(rendered.bounds);
        let png = raster::render(&rendered, opts).unwrap();
        let xml = misterdiagram_render::svg::render(&rendered, opts).unwrap();
        let tree = resvg::usvg::Tree::from_str(&xml, &resvg::usvg::Options { dpi: 192., ..Default::default() }).unwrap();
        let mut pixels = resvg::tiny_skia::Pixmap::new(png.width(), png.height()).unwrap();
        pixels.fill(resvg::tiny_skia::Color::WHITE);
        resvg::render(&tree, resvg::tiny_skia::Transform::identity(), &mut pixels.as_mut());
        let sum: u64 = png
            .pixels()
            .zip(pixels.pixels())
            .map(|(a, b)| u64::from(a[0].abs_diff(b.red())) + u64::from(a[1].abs_diff(b.green())) + u64::from(a[2].abs_diff(b.blue())))
            .sum();
        let mad = sum as f64 / (f64::from(png.width()) * f64::from(png.height()) * 3.);
        assert!(mad < 4., "direction {direction}: {mad}");
        assert!(!misterdiagram_render::pdf::render(&[rendered], opts).unwrap().is_empty());
    }
}
