//! Export intent, consumed by both DrawingML serialization and raster QA.
use kurbo::{BezPath, PathEl, Point, Shape as _};
use misterdiagram_render::{
    scene::{EvaluatedPage, EvaluatedShape, Op, Paint, Scene, SceneBudget},
    style,
};
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Substitution {
    Pattern,
    Shadow,
    Compound,
    Alternate,
}
impl Substitution {
    pub fn name(self) -> &'static str {
        match self {
            Self::Pattern => "pattern foreground solid",
            Self::Shadow => "shadow omitted",
            Self::Compound => "compound outer line",
            Self::Alternate => "alternate fill separate paths",
        }
    }
}
pub struct Page {
    pub canvas: Scene,
    pub(crate) presentation_canvas: Option<(f64, f64, kurbo::Affine)>,
    pub shapes: Vec<EvaluatedShape>,
    pub substitutions: BTreeSet<Substitution>,
}
impl Page {
    pub fn from_evaluated(page: EvaluatedPage) -> Result<Self, String> {
        Self::with_options(page, Default::default(), &mut SceneBudget::default())
    }
    pub fn with_options(mut page: EvaluatedPage, options: misterdiagram_render::scene::Options, budget: &mut SceneBudget) -> Result<Self, String> {
        flatten_foreign(&mut page, options, budget)?;
        let mut substitutions = BTreeSet::new();
        for shape in &mut page.shapes {
            convert(shape, &mut substitutions, &mut page.scene.warnings, 0)?;
        }
        // This is only the canvas, never the source scene replayed for QA.
        page.scene.ops = Vec::new();
        page.scene.pdf_strokes.clear();
        Ok(Self { presentation_canvas: None, canvas: page.scene, shapes: page.shapes, substitutions })
    }
    /// Rebuild paints from export sections and styles, independently of the
    /// renderer's discarded geometry operations. A model-path defect therefore
    /// changes both DrawingML and this raster, rather than escaping the gate.
    pub fn raster_scene(&self) -> Result<Scene, String> {
        let mut scene = self.canvas.clone();
        let mut budget = SceneBudget::default();
        for shape in &self.shapes {
            draw(shape, &mut scene, &mut budget, 0)?;
        }
        Ok(scene)
    }
}
fn convert(node: &mut EvaluatedShape, substitutions: &mut BTreeSet<Substitution>, warnings: &mut Vec<String>, depth: usize) -> Result<(), String> {
    if depth > 128 {
        return Err("PPTX group depth limit".into());
    }
    if let Some(Paint::Pattern { fill, gradient: None, .. }) = &node.paint {
        if fill.pattern > 1 {
            substitutions.insert(Substitution::Pattern);
            if warnings.len() < 1000 {
                warnings.push(format!("PPTX shape {}: Visio fill pattern {} replaced by foreground solid", node.id, fill.pattern));
            }
        }
        node.paint = Some(Paint::Solid(fill.foreground));
    }
    if node.has_shadow {
        substitutions.insert(Substitution::Shadow);
    }
    if let Some(line) = &mut node.line
        && line.compound == 1
        && node.sections.iter().any(|s| s.stroked())
        && line.pattern != 0
    {
        substitutions.insert(Substitution::Compound);
        line.compound = 0;
    }
    // Keep the foreign-object projection (including vector metafiles) and text.
    // Geometry is rebuilt below from export sections; no source geometry replay.
    if node.content_start > node.drawing.ops.len() {
        return Err("PPTX content range".into());
    }
    node.drawing.ops.drain(..node.content_start);
    node.content_start = 0;
    node.drawing.pdf_strokes.clear();
    let mut paths = Vec::new();
    for section in &node.sections {
        if section.filled() {
            let mut path = BezPath::new();
            for el in section.path.elements() {
                if matches!(el, PathEl::MoveTo(_)) && !path.elements().is_empty() {
                    paths.push(path);
                    path = BezPath::new();
                }
                path.push(*el);
            }
            if !path.elements().is_empty() {
                paths.push(path);
            }
        }
    }
    if paths.len() > 2048 {
        return Err("PPTX contour limit".into());
    }
    let overlap = paths.iter().enumerate().any(|(i, p)| {
        paths.iter().skip(i + 1).any(|q| {
            let r = p.bounding_box().intersect(q.bounding_box());
            r.width() > 1e-9 && r.height() > 1e-9
        })
    });
    if overlap && node.paint.is_some() {
        substitutions.insert(Substitution::Alternate);
        // DrawingML has no alternate fill. Nested contours become stroke-only;
        // overlapping contour sets remain an explicitly reported approximation.
        let mut sections = Vec::new();
        for original in std::mem::take(&mut node.sections) {
            if !original.filled() {
                sections.push(original);
                continue;
            }
            let mut contours = Vec::new();
            let mut path = BezPath::new();
            for el in original.path.elements() {
                if matches!(el, PathEl::MoveTo(_)) && !path.elements().is_empty() {
                    contours.push(path);
                    path = BezPath::new();
                }
                path.push(*el);
            }
            if !path.elements().is_empty() {
                contours.push(path);
            }
            for path in contours {
                let bounds = path.bounding_box();
                let nesting = paths
                    .iter()
                    .filter(|other| {
                        let b = other.bounding_box();
                        b != bounds
                            && b.x0 <= bounds.x0
                            && b.y0 <= bounds.y0
                            && b.x1 >= bounds.x1
                            && b.y1 >= bounds.y1
                            && other.winding(bounds.center()) != 0
                    })
                    .count();
                let mut flags = original.flags;
                if nesting % 2 == 1 {
                    flags.no_fill = true;
                }
                sections.push(misterdiagram_geom::GeometryPath { path, flags });
            }
        }
        node.sections = sections;
    }
    for child in &mut node.children {
        convert(child, substitutions, warnings, depth + 1)?;
    }
    Ok(())
}
fn draw(node: &EvaluatedShape, scene: &mut Scene, budget: &mut SceneBudget, depth: usize) -> Result<(), String> {
    if depth > 128 {
        return Err("PPTX group depth limit".into());
    }
    if node.children_first {
        for child in &node.children {
            draw(child, scene, budget, depth + 1)?;
        }
    }
    let start = scene.ops.len();
    if !node.hidden_own {
        if let Some(paint) = &node.paint {
            let mut path = BezPath::new();
            for section in node.sections.iter().filter(|s| s.filled()) {
                path.extend(section.path.elements().iter().copied());
            }
            if !path.elements().is_empty() {
                scene.ops.push(Op::Path { even_odd: true, path, paint: paint.clone() });
            }
        }
        if let (Some(line), Some(stroke)) = (&node.line, &node.stroke) {
            for section in node.sections.iter().filter(|s| s.stroked()) {
                if line.pattern == 0 || stroke.width <= 0. {
                    continue;
                }
                let closed = section.path.elements().iter().any(|el| matches!(el, PathEl::ClosePath));
                let (begin, end) = if closed { (0, 0) } else { (line.begin_arrow, line.end_arrow) };
                let mut stroke = stroke.clone();
                if filled(begin) {
                    stroke.start_cap = kurbo::Cap::Butt;
                }
                if filled(end) {
                    stroke.end_cap = kurbo::Cap::Butt;
                }
                let shaft = style::marker_shaft(
                    &section.path,
                    style::marker_begin_setback(begin, line.begin_size, node.nominal_weight, node.drawing_scale),
                    style::marker_setback(end, line.end_size, node.nominal_weight),
                );
                scene.ops.push(Op::Stroke { path: shaft, stroke, color: line.color });
                let points: Vec<Point> = section
                    .path
                    .elements()
                    .iter()
                    .filter_map(|el| match el {
                        PathEl::MoveTo(p) | PathEl::LineTo(p) | PathEl::QuadTo(_, p) | PathEl::CurveTo(_, _, p) => Some(*p),
                        _ => None,
                    })
                    .collect();
                for (kind, size, pair, reverse) in
                    [(begin, line.begin_size, points.get(..2), true), (end, line.end_size, points.get(points.len().saturating_sub(2)..), false)]
                {
                    if let Some([a, b]) = pair {
                        let (tip, dir) = if reverse { (*a, (*a - *b).atan2()) } else { (*b, (*b - *a).atan2()) };
                        for arrow in style::arrow(kind, size, node.nominal_weight, tip, dir) {
                            if arrow.filled || arrow.stroke {
                                let path = if arrow.stroke {
                                    kurbo::stroke(
                                        arrow.path.elements().iter().copied(),
                                        &kurbo::Stroke::new(style::marker_weight(node.nominal_weight))
                                            .with_caps(kurbo::Cap::Round)
                                            .with_join(kurbo::Join::Round),
                                        &kurbo::StrokeOpts::default(),
                                        1e-5,
                                    )
                                } else {
                                    arrow.path
                                };
                                scene.ops.push(Op::Path { even_odd: false, path, paint: Paint::Solid(line.color) });
                            }
                        }
                    }
                }
            }
        }
        scene.ops.extend(node.drawing.ops.iter().cloned());
    }
    budget.charge_ops(scene.ops.get(start..).ok_or("PPTX operation range")?)?;
    if !node.children_first {
        for child in &node.children {
            draw(child, scene, budget, depth + 1)?;
        }
    }
    Ok(())
}
fn filled(kind: u32) -> bool {
    matches!(kind, 2 | 4 | 5 | 6 | 8 | 10 | 11 | 13 | 35..=39 | 42)
}

fn flatten_foreign(page: &mut EvaluatedPage, options: misterdiagram_render::scene::Options, budget: &mut SceneBudget) -> Result<(), String> {
    let (w, h, view) = page.scene.canvas(options)?;
    fn node(
        n: &mut EvaluatedShape,
        options: misterdiagram_render::scene::Options,
        canvas: (u16, u16, kurbo::Affine),
        budget: &mut SceneBudget,
        depth: usize,
    ) -> Result<(), String> {
        let (_, _, view) = canvas;
        if depth > 128 {
            return Err("PPTX foreign depth limit".into());
        }
        if n.foreign_end > n.content_start {
            let ops = n.drawing.ops.get(n.content_start..n.foreign_end).ok_or("PPTX foreign range")?;
            if natural_image_clip(ops) {
                for child in &mut n.children {
                    node(child, options, canvas, budget, depth + 1)?;
                }
                return Ok(());
            }
            let mut source = n.drawing.clone();
            source.pdf_strokes.clear();
            let frame = match (ops.first(), ops.last()) {
                (Some(clip @ Op::Clip { .. }), Some(Op::Pop)) => Some(clip.clone()),
                _ => None,
            };
            source.ops = if frame.is_some() { ops.get(1..ops.len().saturating_sub(1)).ok_or("PPTX foreign frame")?.to_vec() } else { ops.to_vec() };
            let pixels = misterdiagram_render::raster::render_transparent(&source, options, canvas)?;
            let mut b = (pixels.width(), pixels.height(), 0, 0);
            for (x, y, p) in pixels.enumerate_pixels() {
                if p.0[3] != 0 {
                    b = (b.0.min(x), b.1.min(y), b.2.max(x + 1), b.3.max(y + 1));
                }
            }
            let mut replacement = if b.2 > b.0 && b.3 > b.1 {
                // Retain transparent edge samples so a picture texture cannot
                // extend an opaque edge into a larger clipping frame.
                b = (
                    b.0.saturating_sub(2),
                    b.1.saturating_sub(2),
                    b.2.saturating_add(2).min(pixels.width()),
                    b.3.saturating_add(2).min(pixels.height()),
                );
                let bytes = usize::try_from(u64::from(b.2 - b.0) * u64::from(b.3 - b.1) * 4).map_err(|_| "PPTX foreign pixel limit")?;
                budget.charge_bytes(bytes)?;
                let pixels = image::imageops::crop_imm(&pixels, b.0, b.1, b.2 - b.0, b.3 - b.1).to_image();
                vec![Op::RasterImage {
                    pixels: std::sync::Arc::new(pixels),
                    transform: view.inverse() * kurbo::Affine::translate((f64::from(b.0), f64::from(b.1))),
                }]
            } else {
                vec![]
            };
            if let Some(frame) = frame {
                replacement.insert(0, frame);
                replacement.push(Op::Pop);
            }
            let end = n.content_start + replacement.len();
            n.drawing.ops.splice(n.content_start..n.foreign_end, replacement);
            n.foreign_end = end;
        }
        for child in &mut n.children {
            node(child, options, canvas, budget, depth + 1)?;
        }
        Ok(())
    }
    for shape in &mut page.shapes {
        node(shape, options, (w, h, view), budget, 0)?;
    }
    Ok(())
}

// A picture's natural rectangular frame already provides this clip in DrawingML.
// Retain its original samples and filter, avoiding a second alpha quantization.
fn natural_image_clip(ops: &[Op]) -> bool {
    let [Op::Clip { path, .. }, Op::Image { pixels, transform }, Op::Pop] = ops else {
        return false;
    };
    let [a, b, c, d, _, _] = transform.as_coeffs();
    if (a * c + b * d).abs() > 1e-8 * a.hypot(b) * c.hypot(d) {
        return false;
    }
    let inverse = transform.inverse();
    if inverse.as_coeffs().iter().any(|v| !v.is_finite()) {
        return false;
    }
    let local = inverse * path;
    let points: Vec<_> = local
        .elements()
        .iter()
        .filter_map(|e| match e {
            PathEl::MoveTo(p) | PathEl::LineTo(p) => Some(*p),
            _ => None,
        })
        .collect();
    if points.len() != 4 || local.elements().len() != 5 || !matches!(local.elements().last(), Some(PathEl::ClosePath)) {
        return false;
    }
    let corners =
        [(0., 0.), (f64::from(pixels.width()), 0.), (f64::from(pixels.width()), f64::from(pixels.height())), (0., f64::from(pixels.height()))];
    corners.into_iter().all(|(x, y)| points.iter().any(|p| (p.x - x).abs() < 1e-8 && (p.y - y).abs() < 1e-8))
}
