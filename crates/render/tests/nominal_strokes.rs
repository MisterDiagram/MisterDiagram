//! Contributor-original nominal SVG stroke and raster stability controls.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/lineweight-cell-line-format-section
use kurbo::Shape as _;
use misterdiagram_model::{Cell, Document, skeleton};
use misterdiagram_render::{
    pdf, raster,
    scene::{self, Op, Options},
    svg,
};
fn drawing(weight: f64, kind: &str) -> Document {
    let mut d = skeleton::document(false);
    let mut s = skeleton::rectangle(1, 1., 1., 3., 1., "").unwrap();
    if kind != "shape" {
        s.sheet.sections[0].rows.truncate(2);
    }
    for (name, value) in [
        ("FillPattern", 0.),
        ("LineCap", 0.),
        ("LineWeight", weight / 72.),
        ("LinePattern", if kind == "dash" { 2. } else { 1. }),
        ("CompoundType", if kind == "double" { 1. } else { 0. }),
        ("BeginArrow", 0.),
        ("EndArrow", if kind == "marked" { 13. } else { 0. }),
    ] {
        s.sheet.cells.retain(|c| c.name != name);
        s.sheet.cells.push(Cell { name: name.into(), value: Some(value.to_string()), ..Default::default() });
    }
    d.pages[0].shapes.push(s);
    d
}
#[test]
fn svg_shapes_connectors_and_dashes_use_nominal_width_at_every_scale() {
    for kind in ["shape", "line", "dash"] {
        for weight in [0., 0.1, 0.25, 0.5, 0.75, 1., 2.] {
            for scale in [1., 2.] {
                let o = Options { scale, crop: false, ..Default::default() };
                let s = scene::page(&drawing(weight, kind), 0, o).unwrap();
                let xml = svg::render(&s, o).unwrap();
                let doc = resvg::usvg::roxmltree::Document::parse(&xml).unwrap();
                let widths: Vec<f64> = doc.descendants().filter_map(|n| n.attribute("stroke-width")).map(|v| v.parse().unwrap()).collect();
                assert_eq!(widths.len(), 1);
                let expected = if weight == 0. { 0.75 } else { weight };
                assert!((widths[0] / (96. * scale) * 72. - expected).abs() < 1e-9, "{kind}/{weight}/{scale}: {widths:?}");
                if kind == "dash" {
                    let dashes = doc.descendants().find_map(|n| n.attribute("stroke-dasharray")).unwrap();
                    // Existing dash pitch follows fitted path geometry, while pen width
                    // remains in output points. Undo that view transform for this check.
                    let (_, _, view) = s.canvas(o).unwrap();
                    let fit = view.determinant().abs().sqrt();
                    let actual: Vec<f64> = dashes.split_whitespace().map(|s| s.parse::<f64>().unwrap() / fit * 72.).collect();
                    assert_eq!(actual.len(), 2);
                    for (a, b) in actual.into_iter().zip([7. * expected, 5. * expected]) {
                        assert!((a - b).abs() < 1e-9, "{kind}/{weight}/{scale}: dash {a} versus {b}; {dashes}");
                    }
                }
            }
        }
    }
}
#[test]
fn vector_overrides_preserve_raster_and_pdf_and_compound_nominal_extents() {
    for kind in ["shape", "line", "dash", "marked", "double"] {
        for weight in [0., 0.1, 0.25, 2.] {
            let o = Options { scale: 2., crop: false, ..Default::default() };
            let s = scene::page(&drawing(weight, kind), 0, o).unwrap();
            let mut original = s.clone();
            for op in &mut original.ops {
                *op = match op.clone() {
                    Op::HairlineStroke { path, stroke, color, .. } => Op::Stroke { path, stroke, color },
                    Op::HairlineMarker { path, color, .. } => Op::Path { path, paint: scene::Paint::Solid(color) },
                    other => other,
                };
            }
            assert_eq!(raster::render(&s, o).unwrap(), raster::render(&original, o).unwrap());
            assert_eq!(pdf::render(std::slice::from_ref(&s), o).unwrap(), pdf::render(&[original], o).unwrap());
            if kind == "double" {
                let vector = s
                    .ops
                    .iter()
                    .find_map(|op| match op {
                        Op::HairlineMarker { svg_path, .. } => Some(svg_path),
                        Op::Path { path, .. } => Some(path),
                        _ => None,
                    })
                    .unwrap();
                let expected = if weight == 0. { 0.75 } else { weight };
                assert!((vector.bounding_box().height() * 72. - expected).abs() < 1e-9);
            }
            // One filled marker creates unequal shaft caps, exercising SVG outline emission.
            assert!(svg::render(&s, o).unwrap().contains("<path"));
        }
    }
}
#[test]
fn vector_stroke_metadata_is_rejected_when_nonfinite_or_out_of_bounds() {
    for bad in [f64::NAN, f64::INFINITY, -1., 1e7] {
        let o = Options::default();
        let mut s = scene::page(&drawing(0.25, "line"), 0, o).unwrap();
        let op = s.ops.iter_mut().find(|op| matches!(op, Op::HairlineStroke { .. })).unwrap();
        if let Op::HairlineStroke { svg_width, .. } = op {
            *svg_width = bad;
        }
        assert!(svg::render(&s, o).is_err());
        assert!(raster::render(&s, o).is_err());
        assert!(pdf::render(&[s], o).is_err());
    }
}

#[test]
fn nominal_svg_pen_stays_physical_under_drawing_scale() {
    for kind in ["shape", "line", "dash"] {
        for weight in [0., 0.25, 2.] {
            for drawing_scale in [1., 120.] {
                let mut d = drawing(weight, kind);
                for (name, value) in [("PageScale", 1.), ("DrawingScale", drawing_scale)] {
                    d.pages[0].sheet.cells.retain(|c| c.name != name);
                    d.pages[0].sheet.cells.push(Cell { name: name.into(), value: Some(value.to_string()), ..Default::default() });
                }
                let o = Options { scale: 2., crop: false, ..Default::default() };
                let s = scene::page(&d, 0, o).unwrap();
                let xml = svg::render(&s, o).unwrap();
                let parsed = resvg::usvg::roxmltree::Document::parse(&xml).unwrap();
                let width: f64 = parsed.descendants().find_map(|n| n.attribute("stroke-width")).unwrap().parse().unwrap();
                let expected = if weight == 0. { 0.75 } else { weight };
                assert!((width / 192. * 72. - expected).abs() < 1e-9, "{kind}/{weight}/{drawing_scale}");
            }
        }
    }
}

#[test]
fn svg_dash_expansion_uses_the_existing_scene_budget() {
    let o = Options::default();
    let mut s = scene::page(&drawing(0.25, "line"), 0, o).unwrap();
    let op = s.ops.iter_mut().find(|op| matches!(op, Op::HairlineStroke { .. })).unwrap();
    if let Op::HairlineStroke { stroke, svg_width, .. } = op {
        stroke.dash_pattern = vec![1., 1.].into();
        *svg_width = 1e-12;
    }
    assert!(svg::render(&s, o).unwrap_err().contains("dash expansion"));
    assert!(raster::render(&s, o).unwrap_err().contains("dash expansion"));
    assert!(pdf::render(&[s], o).unwrap_err().contains("dash expansion"));
}
