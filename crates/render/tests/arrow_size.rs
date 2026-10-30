//! Contributor-original line-end size and physical-transform regressions.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/endarrowsize-cell-line-format-section
//! https://learn.microsoft.com/en-us/office/client-developer/visio/lineweight-cell-line-format-section
use kurbo::Shape as _;
use misterdiagram_geom::Rect;
use misterdiagram_model::{Cell, Document, Sheet, skeleton};
use misterdiagram_render::{
    pdf, raster,
    scene::{self, Op, Options},
    svg,
};
fn set(sheet: &mut Sheet, name: &str, value: impl ToString) {
    sheet.cells.retain(|c| c.name != name);
    sheet.cells.push(Cell { name: name.into(), value: Some(value.to_string()), ..Default::default() });
}
fn drawing(weight: f64, size: u32) -> Document {
    let mut d = skeleton::document(false);
    let mut s = skeleton::rectangle(1, 1., 1., 5., 1., "").unwrap();
    s.sheet.sections[0].rows.truncate(2);
    for (n, v) in [
        ("FillPattern", 0.),
        ("LinePattern", 1.),
        ("LineWeight", weight / 72.),
        ("BeginArrow", 13.),
        ("EndArrow", 13.),
        ("BeginArrowSize", f64::from(size)),
        ("EndArrowSize", f64::from(size)),
    ] {
        set(&mut s.sheet, n, v);
    }
    d.pages[0].shapes.push(s);
    d
}
fn markers(d: &Document, scale: f64) -> Vec<Rect> {
    scene::page(d, 0, Options { scale, ..Default::default() })
        .unwrap()
        .ops
        .iter()
        .filter_map(|op| if let Op::Path { path, .. } = op { Some(path.bounding_box()) } else { None })
        .collect()
}
#[test]
fn thin_thick_and_large_markers_use_nominal_point_dimensions() {
    for weight in [0.1, 0.25, 2., 6.] {
        for (size, constant) in [1.44, 1.8, 2.52, 3.24, 3.96, 9., 18.].into_iter().enumerate() {
            let d = drawing(weight, size as u32);
            let unit = (weight + constant) / 72.;
            for scale in [1., 2.] {
                let m = markers(&d, scale);
                assert_eq!(m.len(), 2);
                for b in m {
                    assert!((b.width() - 3. * unit).abs() < 1e-10, "{weight}/{size}/{scale}: {b:?}");
                    assert!((b.height() - 2. * unit).abs() < 1e-10);
                }
                let s = scene::page(&d, 0, Options { scale, ..Default::default() }).unwrap();
                let shaft = s
                    .ops
                    .iter()
                    .find_map(|op| if let Op::Stroke { path, .. } | Op::HairlineStroke { path, .. } = op { Some(path.bounding_box()) } else { None })
                    .unwrap();
                assert!((shaft.x0 - (1. + 3. * unit - (3. * unit * 0.01).max(0.36 / 72.))).abs() < 1e-8);
                assert!((shaft.x1 - (6. - 3. * unit)).abs() < 1e-8);
            }
        }
    }
}
#[test]
fn dashed_group_and_drawing_scale_keep_marker_lengths_physical() {
    let base = drawing(0.25, 2);
    let expected = markers(&base, 2.)[0];
    for mode in ["dashed", "group", "page-scale"] {
        let mut d = base.clone();
        match mode {
            "dashed" => set(&mut d.pages[0].shapes[0].sheet, "LinePattern", 2),
            "group" => {
                let mut child = d.pages[0].shapes.pop().unwrap();
                let mut g = skeleton::rectangle(2, 0., 0., 16., 12., "").unwrap();
                g.shape_type = Some("Group".into());
                g.sheet.sections.clear();
                child.sheet.cells.retain(|c| c.name != "Width");
                child.sheet.cells.push(Cell { name: "Width".into(), formula: Some("Sheet.2!Width*0.625".into()), ..Default::default() });
                set(&mut child.sheet, "Height", 2);
                g.shapes.push(child);
                d.pages[0].shapes.push(g);
            }
            _ => {
                set(&mut d.pages[0].sheet, "PageScale", 1);
                set(&mut d.pages[0].sheet, "DrawingScale", 120);
            }
        }
        for b in markers(&d, 2.) {
            assert!((b.width() - expected.width()).abs() < 1e-9, "{mode}: {b:?}");
            assert!((b.height() - expected.height()).abs() < 1e-9);
        }
    }
}
#[test]
fn marker_geometry_is_shared_by_png_svg_and_pdf() {
    let d = drawing(0.25, 6);
    let o = Options { scale: 2., ..Default::default() };
    let s = scene::page(&d, 0, o).unwrap();
    let png = raster::render(&s, o).unwrap();
    let xml = svg::render(&s, o).unwrap();
    let tree = resvg::usvg::Tree::from_str(&xml, &Default::default()).unwrap();
    let mut pix = resvg::tiny_skia::Pixmap::new(png.width(), png.height()).unwrap();
    pix.fill(resvg::tiny_skia::Color::WHITE);
    resvg::render(&tree, resvg::tiny_skia::Transform::identity(), &mut pix.as_mut());
    let extent = |bytes: &[u8], width: usize| {
        let mut min = usize::MAX;
        let mut max = 0;
        for (i, p) in bytes.as_chunks::<4>().0.iter().enumerate() {
            if p[0] < 128 {
                min = min.min(i / width);
                max = max.max(i / width);
            }
        }
        (min, max)
    };
    let a = extent(png.as_raw(), png.width() as usize);
    let b = extent(pix.data(), png.width() as usize);
    assert!(a.0.abs_diff(b.0) <= 1 && a.1.abs_diff(b.1) <= 1);
    assert!(pdf::render(&[s], o).unwrap().starts_with(b"%PDF"));
}

#[test]
fn hairline_geometry_is_fixed_in_points_and_keeps_native_svg_exception() {
    let d = drawing(0., 2);
    for scale in [1., 2.] {
        let o = Options { scale, crop: false, ..Default::default() };
        let s = scene::page(&d, 0, o).unwrap();
        let markers: Vec<_> =
            s.ops
                .iter()
                .filter_map(|op| {
                    if let Op::HairlineMarker { path, svg_path, .. } = op { Some((path.bounding_box(), svg_path.bounding_box())) } else { None }
                })
                .collect();
        assert_eq!(markers.len(), 2);
        for (normal, svg) in markers {
            assert!((normal.width() * 72. - 7.56).abs() < 1e-10);
            assert!((normal.height() * 72. - 5.04).abs() < 1e-10);
            assert!((svg.width() * 72. - 9.81).abs() < 1e-10);
            assert!((svg.height() * 72. - 6.54).abs() < 1e-10);
        }
        let xml = svg::render(&s, o).unwrap();
        let doc = resvg::usvg::roxmltree::Document::parse(&xml).unwrap();
        let (_, _, view) = s.canvas(o).unwrap();
        let filled: Vec<_> = doc.descendants().filter(|n| n.has_tag_name("path") && n.attribute("fill") == Some("#000000")).collect();
        assert_eq!(filled.len(), 2);
        for marker in filled {
            let p = kurbo::BezPath::from_svg(marker.attribute("d").unwrap()).unwrap();
            assert!(((view.inverse() * p).bounding_box().width() * 72. - 9.81).abs() < 1e-8);
        }
        assert!(!raster::render(&s, o).unwrap().is_empty());
        assert!(pdf::render(&[s], o).unwrap().starts_with(b"%PDF"));
    }
}

#[test]
fn hostile_marker_sizes_and_alternate_paths_remain_bounded() {
    use misterdiagram_geom::Point;
    use misterdiagram_render::style;
    for size in [7, u32::MAX] {
        assert!(style::arrow(13, size, 0.25 / 72., Point::ZERO, 0.).is_empty());
        assert_eq!(style::marker_setback(13, size, 0.25 / 72.), 0.);
        assert_eq!(style::marker_begin_setback(13, size, 0.25 / 72., 1.), 0.);
    }
    for weight in [-1., f64::NAN, f64::INFINITY, f64::MAX] {
        assert!(style::arrow(13, 2, weight, Point::ZERO, 0.).is_empty());
        assert_eq!(style::marker_setback(13, 2, weight), 0.);
    }
    let mut s = scene::page(&drawing(0., 2), 0, Options::default()).unwrap();
    for op in &mut s.ops {
        if let Op::HairlineMarker { svg_path, .. } = op {
            svg_path.line_to((f64::NAN, 0.));
            break;
        }
    }
    assert!(svg::render(&s, Options::default()).is_err());
    assert!(raster::render(&s, Options::default()).is_err());
    assert!(pdf::render(&[s], Options::default()).is_err());
}

/// Legacy THEME and modern THEMEVAL have different unthemed pen defaults.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/theme-function
#[test]
fn legacy_theme_pen_keeps_the_witnessed_quarter_point_marker() {
    for (formula, weight) in [
        ("THEME(\"ConnectorWeight\")", 0.25),
        ("THEME(\"LineWeight\")", 0.25),
        ("THEMEGUARD(IF(CELLISTHEMED(FALSE),THEME(\"ConnectorWeight\"),SETATREFEXPR(THEME(\"ConnectorWeight\"))))", 0.25),
        ("THEMEVAL(\"ConnectorWeight\")", 0.75),
        ("THEMEVAL(\"LineWeight\")", 0.75),
    ] {
        let mut d = drawing(8., 2);
        let c = d.pages[0].shapes[0].sheet.cells.iter_mut().find(|c| c.name == "LineWeight").unwrap();
        c.formula = Some(formula.into());
        for scale in [1., 2.] {
            let m = markers(&d, scale);
            assert_eq!(m.len(), 2);
            for bounds in m {
                assert!((bounds.width() * 72. - 3. * (weight + 2.52)).abs() < 1e-8, "{formula}: {bounds:?}");
            }
        }
    }
}
