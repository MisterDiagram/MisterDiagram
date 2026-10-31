//! Contributor-original physical PDF pen controls, witnessed in
//! field/pdf-strokes-2026-10-31 (Visio 2021).
//! https://learn.microsoft.com/en-us/office/client-developer/visio/lineweight-cell-line-format-section
//! https://learn.microsoft.com/en-us/office/client-developer/visio/linepattern-cell-line-format-section
//! https://learn.microsoft.com/en-us/office/client-developer/visio/compoundtype-cell-line-format-section
use hayro_syntax::{Pdf, content::ops::TypedInstruction};
use kurbo::{Affine, BezPath, Point, Rect, Shape as _};
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
fn drawing(weight: f64, kind: &str) -> Document {
    let mut d = skeleton::document(false);
    set(&mut d.pages[0].sheet, "PageWidth", 6);
    set(&mut d.pages[0].sheet, "PageHeight", 3);
    let mut s = skeleton::rectangle(1, 1., 1., 3., 1., "").unwrap();
    if kind != "box" {
        s.sheet.sections[0].rows.truncate(2);
    }
    for (n, v) in [
        ("FillPattern", 0.),
        ("LineWeight", weight / 72.),
        ("LineCap", 0.),
        ("LineJoin", 0.),
        (
            "LinePattern",
            if kind == "dash2" {
                2.
            } else if kind == "dash4" {
                4.
            } else {
                1.
            },
        ),
        ("CompoundType", if kind == "double" { 1. } else { 0. }),
        (
            "BeginArrow",
            if kind == "open" {
                1.
            } else if kind == "filled" {
                4.
            } else {
                0.
            },
        ),
        (
            "EndArrow",
            if kind == "open" {
                1.
            } else if kind == "filled" {
                4.
            } else {
                0.
            },
        ),
        ("BeginArrowSize", 2.),
        ("EndArrowSize", 2.),
    ] {
        set(&mut s.sheet, n, v);
    }
    d.pages[0].shapes.push(s);
    d
}
struct Contour {
    points: Vec<Point>,
    path: BezPath,
}
impl std::ops::Deref for Contour {
    type Target = [Point];
    fn deref(&self) -> &Self::Target {
        &self.points
    }
}
// Read actual PDF paths with graphics-state transforms. Keep curves for exact
// bounds: a Bezier control polygon is larger than its circular cap.
fn contours(bytes: Vec<u8>) -> Vec<Contour> {
    let pdf = Pdf::new(bytes).unwrap();
    let mut ops = pdf.pages()[0].typed_operations();
    let mut transform = Affine::IDENTITY;
    let mut stack = vec![];
    let mut result = vec![];
    let mut points = vec![];
    let mut path = BezPath::new();
    while let Some(op) = ops.next() {
        match op {
            TypedInstruction::SaveState(_) => stack.push(transform),
            TypedInstruction::RestoreState(_) => transform = stack.pop().unwrap(),
            TypedInstruction::Transform(m) => {
                transform *= Affine::new([m.0.as_f64(), m.1.as_f64(), m.2.as_f64(), m.3.as_f64(), m.4.as_f64(), m.5.as_f64()])
            }
            TypedInstruction::MoveTo(p) => {
                if !points.is_empty() {
                    result.push(Contour { points: std::mem::take(&mut points), path: std::mem::take(&mut path) });
                }
                let point = transform * Point::new(p.0.as_f64(), p.1.as_f64());
                points.push(point);
                path.move_to(point);
            }
            TypedInstruction::LineTo(p) => {
                let point = transform * Point::new(p.0.as_f64(), p.1.as_f64());
                points.push(point);
                path.line_to(point);
            }
            TypedInstruction::CubicTo(p) => {
                path.curve_to(
                    transform * Point::new(p.0.as_f64(), p.1.as_f64()),
                    transform * Point::new(p.2.as_f64(), p.3.as_f64()),
                    transform * Point::new(p.4.as_f64(), p.5.as_f64()),
                );
                for (x, y) in [(p.0, p.1), (p.2, p.3), (p.4, p.5)] {
                    points.push(transform * Point::new(x.as_f64(), y.as_f64()));
                }
            }
            TypedInstruction::RectPath(p) => {
                if !points.is_empty() {
                    result.push(Contour { points: std::mem::take(&mut points), path: std::mem::take(&mut path) });
                }
                let (x, y, w, h) = (p.0.as_f64(), p.1.as_f64(), p.2.as_f64(), p.3.as_f64());
                path = transform * Rect::new(x, y, x + w, y + h).to_path(1e-5);
                for p in [(x, y), (x + w, y), (x + w, y + h), (x, y + h)] {
                    points.push(transform * Point::from(p));
                }
            }
            TypedInstruction::ClosePath(_) => path.close_path(),
            TypedInstruction::CubicStartTo(_) | TypedInstruction::CubicEndTo(_) => panic!("unexpected shorthand curve in synthetic PDF"),
            _ => {}
        }
    }
    if !points.is_empty() {
        result.push(Contour { points, path });
    }
    result
}
fn bounds(p: &Contour) -> Rect {
    p.path.bounding_box()
}
fn near(actual: f64, expected: f64, label: &str) {
    assert!((actual - expected).abs() < 2e-4, "{label}: {actual} versus {expected}");
}
fn pen(weight: f64) -> f64 {
    if weight == 0. { 0.375 } else { weight.max(0.14) }
}
#[test]
fn pdf_shape_line_and_dash_pens_are_physical_at_both_export_scales() {
    for kind in ["box", "line", "dash2", "dash4"] {
        for weight in [0., 0.05, 0.1, 0.12, 0.14, 0.25, 2.] {
            for scale in [1., 2., 12.] {
                let o = Options { scale, crop: false, ..Default::default() };
                let s = scene::page(&drawing(weight, kind), 0, o).unwrap();
                let paths = contours(pdf::render(std::slice::from_ref(&s), o).unwrap());
                let label = format!("{kind}/{weight}/{scale}");
                assert!(!paths.is_empty());
                if kind == "box" {
                    assert_eq!(paths.len(), 2);
                    // Inner join vertices can touch the centerline; measure the
                    // straight horizontal bands away from those vertices.
                    let mut ys: Vec<_> = paths
                        .iter()
                        .flat_map(|p| p.windows(2))
                        .filter_map(|pair| ((pair[0].y - pair[1].y).abs() < 1e-5 && (pair[0].x - pair[1].x).abs() > 4.).then_some(pair[0].y))
                        .collect();
                    ys.sort_by(f64::total_cmp);
                    ys.dedup_by(|a, b| (*a - *b).abs() < 1e-5);
                    assert_eq!(ys.len(), 4);
                    near(ys[1] - ys[0], pen(weight), &label);
                    near(ys[3] - ys[2], pen(weight), &label);
                } else {
                    for p in paths.iter().take(5) {
                        near(bounds(p).height(), pen(weight), &label);
                    }
                }
            }
        }
    }
}
#[test]
fn pdf_dashes_follow_pdf_pen_including_zero_and_positive_minimum() {
    for kind in ["dash2", "dash4"] {
        for weight in [0., 0.05, 0.1, 0.25, 2.] {
            for scale in [1., 2.] {
                let o = Options { scale, crop: false, ..Default::default() };
                let s = scene::page(&drawing(weight, kind), 0, o).unwrap();
                let (_, _, view) = s.canvas(o).unwrap();
                let fit = view.determinant().abs().sqrt() / (96. * scale);
                let paths = contours(pdf::render(std::slice::from_ref(&s), o).unwrap());
                let mut b: Vec<_> = paths.iter().map(bounds).collect();
                b.sort_by(|a, b| a.x0.total_cmp(&b.x0));
                let label = format!("{kind}/{weight}/{scale}");
                assert!(b.len() > 2);
                near(b[1].x0 - b[0].x0, 12. * pen(weight) * fit, &label);
                // A round cap adds one physical pen to the seven-unit dash.
                near(b[0].width(), 7. * pen(weight) * fit + pen(weight), &label);
                if kind == "dash4" {
                    near(b[1].height(), pen(weight), &label);
                }
            }
        }
    }
}
#[test]
fn pdf_double_lines_keep_two_bands_and_transparent_middle_for_regular_native_weights() {
    // Native subminimum doubles have malformed bands, documented as a remaining gap.
    for weight in [0., 0.14, 0.25, 2.] {
        for scale in [1., 2.] {
            let o = Options { scale, crop: false, ..Default::default() };
            let s = scene::page(&drawing(weight, "double"), 0, o).unwrap();
            let paths = contours(pdf::render(std::slice::from_ref(&s), o).unwrap());
            assert_eq!(paths.len(), 2);
            let expected = if weight == 0. { 0.375 } else { weight };
            let outer = bounds(&paths[0]).height();
            let inner = bounds(&paths[1]).height();
            near(outer, expected, "double outer");
            near(inner, expected / 3., "double middle");
        }
    }
}
#[test]
fn pdf_drawing_scale_and_nonunit_group_do_not_scale_pens() {
    for mode in ["scaled", "group"] {
        for kind in ["line", "dash2", "double"] {
            for weight in [0., 0.25, 2.] {
                let mut d = drawing(weight, kind);
                if mode == "scaled" {
                    set(&mut d.pages[0].sheet, "DrawingScale", 10);
                    set(&mut d.pages[0].sheet, "PageScale", 1);
                } else {
                    let mut child = d.pages[0].shapes.pop().unwrap();
                    child.sheet.cells.retain(|c| c.name != "Width");
                    child.sheet.cells.push(Cell { name: "Width".into(), formula: Some("Sheet.2!Width*0.5".into()), ..Default::default() });
                    let mut g = skeleton::rectangle(2, 0., 0., 9., 3., "").unwrap();
                    g.shape_type = Some("Group".into());
                    g.sheet.sections.clear();
                    g.shapes.push(child);
                    d.pages[0].shapes.push(g);
                }
                let o = Options { crop: false, ..Default::default() };
                let s = scene::page(&d, 0, o).unwrap();
                let paths = contours(pdf::render(std::slice::from_ref(&s), o).unwrap());
                let expected = if kind == "double" && weight > 0. { weight } else { pen(weight) };
                near(bounds(&paths[0]).height(), expected, &format!("{mode}/{kind}/{weight}"));
                if mode == "group" && kind == "line" {
                    let (_, _, v) = s.canvas(o).unwrap();
                    let fit = v.determinant().abs().sqrt() / 96.;
                    near(bounds(&paths[0]).width() - expected, 4.5 * 72. * fit, "resized child centerline");
                }
            }
        }
    }
}
#[test]
fn pdf_marker_geometry_stays_nominal_while_open_outline_uses_pdf_minimum() {
    for drawing_scale in [1., 10.] {
        for kind in ["filled", "open"] {
            for weight in [0., 0.05, 0.1, 0.25] {
                for scale in [1., 2.] {
                    let o = Options { scale, crop: false, ..Default::default() };
                    let mut d = drawing(weight, kind);
                    set(&mut d.pages[0].sheet, "DrawingScale", drawing_scale);
                    set(&mut d.pages[0].sheet, "PageScale", 1);
                    let s = scene::page(&d, 0, o).unwrap();
                    let paths = contours(pdf::render(std::slice::from_ref(&s), o).unwrap());
                    near(bounds(&paths[0]).height(), pen(weight), "marker shaft pen");
                    assert_eq!(paths.len(), 3);
                    let (_, _, view) = s.canvas(o).unwrap();
                    let fit = view.determinant().abs().sqrt() / (96. * scale);
                    let nominal_half = 2.52 + weight;
                    for marker in paths.iter().skip(1) {
                        if kind == "filled" {
                            near(bounds(marker).height(), 2. * nominal_half * fit, "filled nominal geometry");
                            near(bounds(marker).width(), 2. * nominal_half * fit, "filled nominal length");
                        } else {
                            near(bounds(marker).height(), 2. * nominal_half * fit + pen(weight), "open nominal geometry with round caps");
                            // The 45-degree flanks give a direct normal-distance pen measurement.
                            let mut diagonals: Vec<_> = marker
                                .windows(2)
                                .filter_map(|pair| {
                                    let a = pair[0];
                                    let b = pair[1];
                                    let dx = b.x - a.x;
                                    let dy = b.y - a.y;
                                    (dx.abs() > 0.5 && (dx.abs() - dy.abs()).abs() < 2e-4 && dx * dy > 0.).then_some((a.y - a.x) / 2f64.sqrt())
                                })
                                .collect();
                            diagonals.sort_by(f64::total_cmp);
                            diagonals.dedup_by(|a, b| (*a - *b).abs() < 1e-5);
                            assert_eq!(diagonals.len(), 2, "{kind}/{weight}/{scale}/{drawing_scale}: {diagonals:?}");
                            near(diagonals[1] - diagonals[0], pen(weight), "open marker pen");
                        }
                    }
                    let mut legacy = s.clone();
                    legacy.pdf_strokes.clear();
                    let old = contours(pdf::render(std::slice::from_ref(&legacy), o).unwrap());
                    let old_pen = legacy
                        .ops
                        .iter()
                        .find_map(|op| match op {
                            Op::Stroke { stroke, .. } | Op::HairlineStroke { stroke, .. } => Some(stroke.width * 72.),
                            _ => None,
                        })
                        .unwrap();
                    // This correction changes pens only. Native filled-marker shaft
                    // setbacks differ from the existing geometry and are recorded
                    // separately; preserve both original endpoints, undoing round caps.
                    let (new_cap, old_cap) = if kind == "open" { (pen(weight) / 2., old_pen / 2.) } else { (0., 0.) };
                    near(bounds(&paths[0]).x0 + new_cap, bounds(&old[0]).x0 + old_cap, "unchanged begin setback");
                    near(bounds(&paths[0]).x1 - new_cap, bounds(&old[0]).x1 - old_cap, "unchanged end setback");
                    assert_eq!(raster::render(&s, o).unwrap(), raster::render(&legacy, o).unwrap());
                    assert_eq!(svg::render(&s, o).unwrap(), svg::render(&legacy, o).unwrap());
                }
            }
        }
    }
}
#[test]
fn hostile_pdf_overrides_are_bounded_in_every_backend() {
    let o = Options::default();
    let base = scene::page(&drawing(0.1, "line"), 0, o).unwrap();
    for bad in [f64::NAN, f64::INFINITY, -1., 1e7] {
        let mut s = base.clone();
        s.pdf_strokes.values_mut().next().unwrap().stroke.width = bad;
        assert!(pdf::render(std::slice::from_ref(&s), o).is_err());
        assert!(svg::render(&s, o).is_err());
        assert!(raster::render(&s, o).is_err());
    }
    for mode in ["coordinate", "dash", "index", "operation"] {
        let mut s = base.clone();
        let index = *s.pdf_strokes.keys().next().unwrap();
        match mode {
            "coordinate" => s.pdf_strokes.get_mut(&index).unwrap().path.move_to((f64::NAN, 0.)),
            "dash" => s.pdf_strokes.get_mut(&index).unwrap().stroke.dash_pattern = vec![1e-200, 1e-200].into(),
            "index" => {
                let pen = s.pdf_strokes.remove(&index).unwrap();
                s.pdf_strokes.insert(usize::MAX, pen);
            }
            _ => s.ops[index] = Op::Bounds(Rect::ZERO),
        }
        assert!(pdf::render(std::slice::from_ref(&s), o).is_err(), "{mode}");
        assert!(svg::render(&s, o).is_err(), "{mode}");
        assert!(raster::render(&s, o).is_err(), "{mode}");
    }
}
#[test]
fn caller_strokes_and_unaffected_pdf_pens_keep_existing_content() {
    let o = Options::default();
    let s = scene::page(&drawing(2., "line"), 0, o).unwrap();
    assert!(s.pdf_strokes.is_empty());
    let mut raw = s.clone();
    let mut path = BezPath::new();
    path.move_to((1., 1.));
    path.line_to((3., 1.));
    raw.ops = vec![Op::Stroke { path, stroke: kurbo::Stroke::new(0.05 / 72.), color: misterdiagram_color::Rgba::BLACK }];
    let paths = contours(pdf::render(&[raw], o).unwrap());
    near(bounds(&paths[0]).height(), 0.05, "caller pen");
}
#[test]
fn subminimum_double_fallback_preserves_the_small_native_cap_envelope() {
    // The native giant strips/collapsed offset band remain an explicit gap.
    // Only their small cap envelope is reproduced by this regular double fallback.
    for (weight, envelope) in [(0.05, 0.06), (0.1, 0.12), (0.12, 0.12)] {
        for scale in [1., 2.] {
            let o = Options { scale, crop: false, ..Default::default() };
            let s = scene::page(&drawing(weight, "double"), 0, o).unwrap();
            let paths = contours(pdf::render(std::slice::from_ref(&s), o).unwrap());
            assert_eq!(paths.len(), 2);
            near(bounds(&paths[0]).height(), envelope, "subminimum cap envelope");
        }
    }
}
