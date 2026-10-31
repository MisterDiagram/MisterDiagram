//! Contributor-original seven-inch lines, observed in the Brief 31 Visio witness.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/linepattern-cell-line-format-section
//! https://learn.microsoft.com/en-us/office/client-developer/visio/linecap-cell-line-format-section
use hayro_syntax::{Pdf, content::ops::TypedInstruction};
use misterdiagram_model::{Cell, Sheet, skeleton};
use misterdiagram_render::{pdf, raster, scene, svg};

fn set(sheet: &mut Sheet, name: &str, value: impl ToString) {
    sheet.cells.retain(|c| c.name != name);
    sheet.cells.push(Cell { name: name.into(), value: Some(value.to_string()), ..Default::default() });
}
fn drawing(pattern: u32, weight: f64, cap: u32, options: scene::Options) -> scene::Scene {
    let mut doc = skeleton::document(false);
    set(&mut doc.pages[0].sheet, "PageWidth", 8);
    set(&mut doc.pages[0].sheet, "PageHeight", 1);
    let mut shape = skeleton::rectangle(1, 0.5, 0.5, 7., 0.01, "").unwrap();
    shape.sheet.sections[0].rows.truncate(2);
    for (name, value) in [("LinePattern", pattern as f64), ("LineWeight", weight / 72.), ("LineCap", cap as f64), ("FillPattern", 0.)] {
        set(&mut shape.sheet, name, value);
    }
    doc.pages[0].shapes.push(shape);
    scene::page(&doc, 0, options).unwrap()
}
fn svg_pixels(drawing: &scene::Scene, options: scene::Options) -> image::RgbaImage {
    let source = svg::render(drawing, options).unwrap();
    let tree = resvg::usvg::Tree::from_str(&source, &resvg::usvg::Options::default()).unwrap();
    let size = tree.size().to_int_size();
    let mut pixels = resvg::tiny_skia::Pixmap::new(size.width(), size.height()).unwrap();
    resvg::render(&tree, resvg::tiny_skia::Transform::identity(), &mut pixels.as_mut());
    image::RgbaImage::from_raw(size.width(), size.height(), pixels.data().to_vec()).unwrap()
}
fn runs(pixels: &image::RgbaImage) -> Vec<(usize, usize)> {
    let mut ink = vec![false; pixels.width() as usize];
    for (x, _, pixel) in pixels.enumerate_pixels() {
        if pixel[3] > 0 && pixel[0] < 254 {
            ink[x as usize] = true;
        }
    }
    let mut runs = vec![];
    let mut start = None;
    for (i, active) in ink.into_iter().chain([false]).enumerate() {
        if active && start.is_none() {
            start = Some(i);
        }
        if !active && let Some(first) = start.take() {
            runs.push((first, i));
        }
    }
    runs
}
// Counts from native PDF dash operators along each 504 pt line, including the
// endpoint when it coincides with a mark. The exporter may omit that final cap.
const NATIVE: [[usize; 4]; 23] = [
    [1, 1, 1, 1],
    [169, 57, 34, 15],
    [404, 135, 81, 34],
    [237, 79, 48, 20],
    [275, 92, 55, 23],
    [209, 70, 42, 17],
    [113, 38, 23, 10],
    [127, 43, 25, 11],
    [337, 113, 68, 29],
    [673, 225, 135, 57],
    [449, 150, 90, 38],
    [505, 169, 101, 43],
    [404, 135, 81, 34],
    [225, 75, 45, 19],
    [253, 85, 51, 22],
    [85, 29, 17, 8],
    [225, 75, 45, 19],
    [123, 41, 25, 11],
    [145, 49, 29, 13],
    [106, 35, 22, 9],
    [57, 19, 11, 5],
    [64, 22, 13, 6],
    [673, 225, 135, 57],
];
#[test]
fn all_native_patterns_have_complete_pdf_marks_at_both_scales() {
    for pattern in 1..=23 {
        for (wi, weight) in [0.25, 0.75, 1.25, 3.].into_iter().enumerate() {
            for cap in 0..=2 {
                for scale in [1., 3.125] {
                    let options = scene::Options { scale, crop: false, ..Default::default() };
                    let drawing = drawing(pattern, weight, cap, options);
                    let expected = NATIVE[pattern as usize - 1][wi];
                    let gap = match pattern {
                        2..=8 => 5.,
                        9..=15 => 3.,
                        16..=22 => 9.,
                        23 => 2.,
                        _ => 0.,
                    };
                    let clear_pixels = (gap - 1.) * weight * 96. * scale / 72.;
                    for (backend, pixels) in [("PNG", raster::render(&drawing, options).unwrap()), ("SVG", svg_pixels(&drawing, options))] {
                        let visible = runs(&pixels);
                        assert!(!visible.is_empty(), "blank {backend}: pattern={pattern}, weight={weight}, cap={cap}, scale={scale}");
                        // At subpixel pitch multiple physical marks share a pixel.
                        // PDF contours below still count every physical mark.
                        if pattern == 1 || clear_pixels > 2. {
                            assert!(
                                visible.len().abs_diff(expected) <= 2,
                                "{backend} pattern={pattern} weight={weight} cap={cap} scale={scale}: {}, native={expected}",
                                visible.len()
                            );
                        }
                        assert!(
                            visible.windows(2).all(|p| (p[1].0 - p[0].1) as f64 <= clear_pixels.ceil() + 2.),
                            "{backend} missing mark run: pattern={pattern} weight={weight} cap={cap} scale={scale}"
                        );
                        let (_, _, view) = drawing.canvas(options).unwrap();
                        let start = (view * misterdiagram_geom::Point::new(0.5, 0.5)).x;
                        let end = (view * misterdiagram_geom::Point::new(7.5, 0.5)).x;
                        let end_tolerance = gap * weight * 96. * scale / 72. + 2.;
                        assert!(visible[0].0 as f64 <= start + 2., "{backend} missing start");
                        assert!(visible.last().unwrap().1 as f64 >= end - end_tolerance, "{backend} missing end");
                    }
                    let bytes = pdf::render(&[drawing], options).unwrap();
                    let pdf = Pdf::new(bytes).unwrap();
                    let mut operations = pdf.pages()[0].typed_operations();
                    let mut marks: usize = 0;
                    while let Some(op) = operations.next() {
                        if matches!(op, TypedInstruction::MoveTo(_)) {
                            marks += 1;
                        }
                    }
                    let expected = NATIVE[pattern as usize - 1][wi];
                    assert!(
                        marks.abs_diff(expected) <= 2,
                        "pattern={pattern} weight={weight} cap={cap} scale={scale}: PDF={marks}, native={expected}"
                    );
                }
            }
        }
    }
}
#[test]
fn issue70_three_lines_have_even_complete_png_and_svg_marks() {
    let options = scene::Options { crop: false, ..Default::default() };
    let mut doc = skeleton::document(false);
    set(&mut doc.pages[0].sheet, "PageWidth", 8);
    set(&mut doc.pages[0].sheet, "PageHeight", 6);
    let cases = [(5., 3, 1.25, "#5A5A5A", 81usize, 8usize), (3., 2, 2.25, "#404040", 19, 16), (1., 3, 0.75, "#000000", 135, 5)];
    for (index, (y, pattern, weight, color, _, _)) in cases.iter().enumerate() {
        let mut shape = skeleton::rectangle(index as u32 + 1, 0.5, *y, 7., 0.01, "").unwrap();
        shape.sheet.sections[0].rows.truncate(2);
        set(&mut shape.sheet, "LinePattern", pattern);
        set(&mut shape.sheet, "LineWeight", weight / 72.);
        set(&mut shape.sheet, "LineColor", color);
        set(&mut shape.sheet, "LineCap", 0);
        set(&mut shape.sheet, "FillPattern", 0);
        doc.pages[0].shapes.push(shape);
    }
    let drawing = scene::page(&doc, 0, options).unwrap();
    let (_, _, view) = drawing.canvas(options).unwrap();
    for (backend, pixels) in [("PNG", raster::render(&drawing, options).unwrap()), ("SVG", svg_pixels(&drawing, options))] {
        for (y, pattern, weight, _, expected, max_gap) in cases {
            let center = (view * misterdiagram_geom::Point::new(0.5, y)).y as u32;
            let band = image::imageops::crop_imm(&pixels, 0, center - 5, pixels.width(), 11).to_image();
            let marks = runs(&band);
            assert!(marks.len().abs_diff(expected) <= 2, "{backend} pattern={pattern} weight={weight}: {} marks, native={expected}", marks.len());
            assert!(marks.windows(2).all(|p| p[1].0 - p[0].1 <= max_gap), "{backend} has a run of missing marks");
        }
    }
    let parsed = Pdf::new(pdf::render(&[drawing], options).unwrap()).unwrap();
    let mut operations = parsed.pages()[0].typed_operations();
    let mut marks = 0usize;
    while let Some(op) = operations.next() {
        if matches!(op, TypedInstruction::MoveTo(_)) {
            marks += 1;
        }
    }
    assert!(marks.abs_diff(235) <= 6, "three-line PDF: {marks} marks, native235");
}
#[test]
fn hostile_tiny_dash_weights_are_bounded_before_expansion() {
    let options = scene::Options { crop: false, ..Default::default() };
    let mut drawing = drawing(3, 0.75, 0, options);
    for op in &mut drawing.ops {
        if let scene::Op::Stroke { stroke, .. } = op {
            stroke.width = 1e-200;
            stroke.dash_pattern = vec![0., 1e-200].into();
        }
    }
    assert!(raster::render(&drawing, options).unwrap_err().contains("dash expansion"));
    assert!(svg::render(&drawing, options).unwrap_err().contains("dash expansion"));
    assert!(pdf::render(&[drawing], options).unwrap_err().contains("dash expansion"));
    let mut doc = skeleton::document(false);
    let mut shape = skeleton::rectangle(1, 1., 1., 7., 1., "").unwrap();
    set(&mut shape.sheet, "LinePattern", 3);
    set(&mut shape.sheet, "LineWeight", 1e-200);
    doc.pages[0].shapes.push(shape);
    for compound in [0, 1] {
        set(&mut doc.pages[0].shapes[0].sheet, "CompoundType", compound);
        assert!(scene::page(&doc, 0, options).unwrap_err().contains("dash expansion"));
    }
    set(&mut doc.pages[0].shapes[0].sheet, "CompoundType", 0);
    set(&mut doc.pages[0].shapes[0].sheet, "ShdwPattern", 1);
    set(&mut doc.pages[0].shapes[0].sheet, "ShapeShdwType", 1);
    assert!(scene::page(&doc, 0, options).unwrap_err().contains("dash expansion"));
}

#[test]
fn cropped_dot_pitch_tracks_fitted_geometry_through_the_last_mark() {
    // Synthetic five-inch line: native pattern-03 has a five-pixel period at 96 dpi and
    // 0.75 pt. Cropping fits its centerline at 95.6 px/in, so nominal 96 dpi
    // dash spacing accumulates almost two pixels of error at the last dot.
    let mut path = kurbo::BezPath::new();
    path.move_to((0., 2.5));
    path.line_to((5., 2.5));
    let drawing = scene::Scene {
        pdf_strokes: Default::default(),
        warnings: vec![],
        ops: vec![scene::Op::Stroke {
            path,
            stroke: kurbo::Stroke::new(1. / 96.).with_caps(kurbo::Cap::Round).with_dashes(0., [1e-6, 5. / 96. - 1e-6]),
            color: misterdiagram_color::Rgba::BLACK,
        }],
        bounds: kurbo::Rect::new(0., 0., 5., 5.),
        page_size: (5., 5.),
    };
    for scale in [1., 3.125] {
        let options = scene::Options { scale, ..Default::default() };
        let (_, _, view) = drawing.canvas(options).unwrap();
        for (backend, pixels) in [("PNG", raster::render(&drawing, options).unwrap()), ("SVG", svg_pixels(&drawing, options))] {
            let marks = runs(&pixels);
            assert!(marks.len() >= 96, "{backend}: incomplete cropped line");
            for (index, (left, right)) in marks.iter().take(96).enumerate() {
                let expected = (view * kurbo::Point::new(index as f64 * 5. / 96., 2.5)).x;
                let center = (*left + *right) as f64 / 2.;
                assert!((center - expected).abs() < 0.6, "{backend}, scale={scale}, mark={index}: center={center}, expected={expected}");
            }
        }
    }
}
