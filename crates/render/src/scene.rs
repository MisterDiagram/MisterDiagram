//! Evaluated drawing operations in y-up page inches; output transforms flip Y once.
use crate::style::{self, Fill, Line};
use kurbo::Shape as _;
use misterdiagram_color::Rgba;
use misterdiagram_geom::{Affine, BezPath, Point, Rect};
use misterdiagram_model::{Document, Shape};
use misterdiagram_sheet::Value;
use misterdiagram_text::{NoFields, Opts, TextLayout};
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
use std::sync::Arc;

mod budget;
pub use budget::SceneBudget;

pub const MAX_PIXELS: usize = 32 * 1024 * 1024;
#[derive(Clone, Debug)]
pub enum Paint {
    Solid(Rgba),
    Pattern { fill: Fill, gradient: Option<style::Gradient>, bounds: Rect, transform: Affine },
}
#[derive(Clone, Debug)]
pub enum Op {
    Bounds(Rect),
    Clip { path: BezPath, even_odd: bool },
    Pop,
    Path { path: BezPath, paint: Paint },
    Stroke { path: BezPath, stroke: kurbo::Stroke, color: Rgba },
    Image { pixels: Arc<image::RgbaImage>, transform: Affine },
    Text { layout: TextLayout, transform: Affine },
}
#[derive(Clone, Debug)]
pub struct Scene {
    pub warnings: Vec<String>,
    pub ops: Vec<Op>,
    pub bounds: Rect,
    pub page_size: (f64, f64),
}
#[derive(Clone, Copy, Debug)]
pub struct Options {
    /// SVG only: retain the historical glyph path serialization.
    pub text_outlines: bool,
    pub scale: f64,
    pub print: bool,
    pub crop: bool,
    /// Page.Export uses visible geometry and a fixed canvas for empty documents.
    /// https://learn.microsoft.com/en-us/office/vba/api/visio.page.export
    pub visio_export: bool,
    /// Cells the evaluator cannot compute take the value saved in the file, each one
    /// reported in the scene's warnings. User-facing renders of opened files enable this;
    /// oracle comparisons keep the error (MisterDiagram#78).
    pub cached_fallback: bool,
}
impl Default for Options {
    fn default() -> Self {
        Self { text_outlines: false, scale: 1., print: false, crop: true, visio_export: false, cached_fallback: false }
    }
}
impl Scene {
    pub fn canvas(&self, options: Options) -> Result<(u16, u16, Affine), String> {
        if !options.scale.is_finite() || options.scale <= 0. || options.scale > 100. {
            return Err("render scale outside (0,100]".into());
        }
        let mut depth = 0usize;
        let mut elements = 0usize;
        if self.ops.len() > 200_000 {
            return Err("scene operation limit".into());
        }
        for op in &self.ops {
            match op {
                Op::Path { path, .. } | Op::Stroke { path, .. } | Op::Clip { path, .. } => {
                    elements = elements.saturating_add(path.elements().len());
                    if elements > 4_000_000 {
                        return Err("scene path limit".into());
                    }
                    for el in path.elements() {
                        let finite = |p: Point| p.x.is_finite() && p.y.is_finite() && p.x.abs() <= 1e6 && p.y.abs() <= 1e6;
                        let valid = match *el {
                            kurbo::PathEl::MoveTo(p) | kurbo::PathEl::LineTo(p) => finite(p),
                            kurbo::PathEl::QuadTo(a, p) => finite(a) && finite(p),
                            kurbo::PathEl::CurveTo(a, b, p) => finite(a) && finite(b) && finite(p),
                            kurbo::PathEl::ClosePath => true,
                        };
                        if !valid {
                            return Err("scene coordinate limit".into());
                        }
                    }
                }
                Op::Image { pixels, transform } => {
                    if u64::from(pixels.width()) * u64::from(pixels.height()) > MAX_PIXELS as u64
                        || transform.as_coeffs().iter().any(|v| !v.is_finite())
                    {
                        return Err("scene image limit".into());
                    }
                }
                Op::Text { layout, transform } => {
                    if layout.runs.len() > 65536
                        || layout.lines.len() > 65536
                        || layout
                            .runs
                            .iter()
                            .map(|r| r.svg.text.len().saturating_add(r.svg.family.len()))
                            .try_fold(0usize, |a, b| a.checked_add(b))
                            .is_none_or(|n| n > 16 * 1024 * 1024)
                        || layout.lines.iter().any(|l| {
                            !l.baseline.is_finite() || !l.extent.0.is_finite() || l.edges.iter().any(|(a, b)| !a.is_finite() || !b.is_finite())
                        })
                        || layout.runs.iter().any(|r| r.svg.line >= layout.lines.len() || !r.alpha.is_finite() || !r.svg.spacing.is_finite())
                        || layout.runs.windows(2).any(|r| r.first().zip(r.get(1)).is_some_and(|(a, b)| a.svg.line > b.svg.line))
                        || transform.as_coeffs().iter().any(|v| !v.is_finite())
                        || layout.runs.iter().any(|r| {
                            r.glyphs.len() > 65536
                                || !r.size.is_finite()
                                || r.size < 0.
                                || r.size > 1e6
                                || r.glyphs.iter().any(|(_, x, y)| !x.is_finite() || !y.is_finite())
                        })
                    {
                        return Err("scene text limit".into());
                    }
                }
                Op::Pop | Op::Bounds(_) => {}
            }
            if let Op::Stroke { stroke, .. } = op
                && (!stroke.width.is_finite()
                    || stroke.width < 0.
                    || stroke.width > 1e6
                    || !stroke.miter_limit.is_finite()
                    || stroke.miter_limit < 0.
                    || stroke.miter_limit > 1e6
                    || !stroke.dash_offset.is_finite()
                    || stroke.dash_offset.abs() > 1e6
                    || stroke.dash_pattern.len() > 4096
                    || stroke.dash_pattern.iter().any(|n| !n.is_finite() || *n < 0. || *n > 1e6)
                    || (!stroke.dash_pattern.is_empty() && stroke.dash_pattern.iter().sum::<f64>() <= 0.))
            {
                return Err("scene stroke limit".into());
            }
            if let Op::Stroke { path, stroke, .. } = op {
                elements = elements.saturating_add(dash_elements(path, stroke)?);
                if elements > 4_000_000 {
                    return Err("scene path limit (dash expansion)".into());
                }
            }
            match op {
                Op::Clip { .. } => {
                    depth += 1;
                    if depth > 128 {
                        return Err("scene clip depth".into());
                    }
                }
                Op::Pop => {
                    depth = depth.checked_sub(1).ok_or("unbalanced scene clip")?;
                }
                _ => {}
            }
        }
        if depth != 0 {
            return Err("unclosed scene clip".into());
        }
        let b = self.bounds;
        if [b.x0, b.x1, b.y0, b.y1].iter().any(|n| !n.is_finite() || n.abs() > 1e6) {
            return Err("render bounds limit".into());
        }
        let dpi = 96. * options.scale;
        // Export extents share the rasterizer eighth-pixel device grid.
        // end-39.svg gives397.0133px; its PNG is397px, not398.
        let extent = |v: f64| ((v * dpi * 8.).round() / 8. - 1e-7).ceil().max(1.);
        let w = extent(b.width());
        let h = extent(b.height());
        if w > f64::from(u16::MAX) || h > f64::from(u16::MAX) || w * h > MAX_PIXELS as f64 {
            return Err("render pixel limit".into());
        }
        let sx = if w > 2. && b.width() > 0. { (w - 2.) / b.width() } else { dpi };
        let sy = if h > 2. && b.height() > 0. { (h - 2.) / b.height() } else { dpi };
        // Keep geometry isotropic on the export canvas. Portrait route witnesses
        // expose the distortion caused by fitting the axes independently.
        // https://learn.microsoft.com/en-us/office/vba/api/visio.page.export
        let fit = sx.max(sy);
        Ok((
            w as u16,
            h as u16,
            Affine::translate((w / 2., h / 2.))
                * Affine::scale_non_uniform(fit, -fit)
                * Affine::translate((-(b.x0 + b.x1) / 2., -(b.y0 + b.y1) / 2.)),
        ))
    }
}
/// Local-coordinate extent an InfiniteLine row is clipped to: the page, seen through the
/// inverse shape transform and widened by one inch, or the historical 20,000 in square when
/// there is no page (masters) or the transform cannot be inverted.
fn infinite_line_clip(shape_to_page: Affine, page: Option<Rect>) -> Rect {
    const FALLBACK: Rect = Rect::new(-1e4, -1e4, 1e4, 1e4);
    let Some(page) = page.filter(|r| r.width() > 0. && r.height() > 0. && r.width().is_finite() && r.height().is_finite()) else {
        return FALLBACK;
    };
    let det = shape_to_page.determinant();
    if !det.is_finite() || det.abs() < 1e-12 {
        return FALLBACK;
    }
    let local = shape_to_page.inverse().transform_rect_bbox(page).inflate(1., 1.);
    let inside = [local.x0, local.y0, local.x1, local.y1].iter().all(|v| v.is_finite() && v.abs() <= 1e4);
    if inside { local } else { FALLBACK }
}
fn number(e: &mut Evaluator<'_>, scope: Scope, name: &str, default: f64) -> f64 {
    e.evaluate(scope, name).numeric().ok().filter(|n| n.is_finite() && n.abs() <= 1e6).unwrap_or(default)
}
/// Retain an explicitly cached zero when an invalid LineWeight cannot evaluate.
/// The B25 readonly Dell instance witness reports ResultIU=0 for REF()*ScaleAdj;
/// substituting the ordinary missing-cell default adds spurious endpoint marks.
/// Other invalid caches retain the prior fallback until separately witnessed.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/lineweight-cell-line-format-section
fn line_weight(e: &mut Evaluator<'_>, scope: Scope) -> f64 {
    match e.evaluate(scope, "LineWeight").numeric() {
        Ok(value) if value.is_finite() && value.abs() <= 1e6 => value,
        Err(_) if e.cell(scope, "LineWeight").and_then(|c| c.value.as_deref()).and_then(|v| v.parse::<f64>().ok()) == Some(0.) => 0.,
        _ => 0.01,
    }
}
fn color(e: &mut Evaluator<'_>, scope: Scope, name: &str, trans: &str) -> Rgba {
    let mut c = match e.evaluate(scope, name) {
        Value::Color { rgb: [r, g, b], .. } => Rgba::rgb(r, g, b),
        Value::Number(n) if n.is_finite() && n >= 0. && n <= f64::from(u32::MAX) => {
            e.document.colors.get(&(n as u32)).and_then(|s| Rgba::from_hex(s)).unwrap_or_else(|| {
                let [r, g, b] = misterdiagram_sheet::functions::default_color(n as u32).unwrap_or([0, 0, 0]);
                Rgba::rgb(r, g, b)
            })
        }
        _ => Rgba::BLACK,
    };
    c.a = ((1. - number(e, scope, trans, 0.).clamp(0., 1.)) * 255.).round() as u8;
    c
}
/// Resolve gradient stops through the same inherited cells as ShapeSheet evaluation.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/fillgradient-section
fn gradient(e: &mut Evaluator<'_>, scope: Scope) -> Option<style::Gradient> {
    if number(e, scope, "FillGradientEnabled", 0.) == 0. {
        return None;
    }
    let themed_count = e.theme_fill_gradient_stop_count(scope).ok().flatten();
    let mut stops = Vec::new();
    let mut fallback = Vec::new();
    for index in 0..10 {
        let suffix = if index == 0 { String::new() } else { format!("[{}]", index + 1) };
        let position_name = format!("FillGradientStops.GradientStopPosition{suffix}");
        let color_name = format!("FillGradientStops.GradientStopColor{suffix}");
        let trans_name = format!("FillGradientStops.GradientStopColorTrans{suffix}");
        let position = number(e, scope, &position_name, if index == 0 { 0. } else { 1. });
        let c = color(e, scope, &color_name, &trans_name);
        let themed = [&position_name, &color_name, &trans_name].iter().all(|name| {
            e.cell(scope, name).is_none()
                || e.effective_formula(scope, name).is_some_and(|formula| {
                    formula.bytes().filter(|b| !b.is_ascii_whitespace()).map(|b| b.to_ascii_uppercase()).eq(b"THEMEVAL()".iter().copied())
                })
        });
        if index < 2 {
            fallback.push((position, c));
        }
        // Theme metadata distinguishes genuine white-at-zero stops from unused slots.
        // Explicit cell overrides remain part of the gradient beyond the theme's list.
        if !themed || themed_count.is_none_or(|count| index < count) {
            stops.push((position, c));
        }
    }
    if stops.is_empty() {
        stops = fallback;
    }
    stops.sort_by(|a, b| a.0.total_cmp(&b.0));
    Some(style::Gradient { stops, radial: number(e, scope, "FillGradientDir", 0.) != 0., angle: number(e, scope, "FillGradientAngle", 0.) })
}
/// Background pages paint before the foreground. PageScale/DrawingScale convert drawing units.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/page-element-pages_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pagescale-cell-page-properties-section
/// The default export selects the first foreground page; background pages are composited by BackPage.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/page-element-pages_type-complextypevisio-xml
pub fn default_page_index(document: &Document) -> Result<usize, String> {
    document
        .pages
        .iter()
        .position(|p| !p.attributes.get("Background").is_some_and(|s| s == "1" || s == "true"))
        .ok_or_else(|| "no foreground pages".into())
}
pub fn page(document: &Document, index: usize, options: Options) -> Result<Scene, String> {
    page_budgeted(document, index, options, &mut SceneBudget::default())
}
/// Share a retained-scene allocation budget across pages of one export.
pub fn page_budgeted(document: &Document, index: usize, options: Options, budget: &mut SceneBudget) -> Result<Scene, String> {
    let themes = misterdiagram_theme::Themes::from_document(document).map_err(|e| e.to_string())?;
    let host = misterdiagram_text::Host;
    let evaluator = Evaluator::new(document).map_err(|e| e.to_string())?.with_theme_host(&themes).with_text_host(&host);
    let mut evaluator = if options.cached_fallback { evaluator.with_cached_fallback() } else { evaluator };
    page_prepared_budgeted(&mut evaluator, index, options, budget)
}
/// Cells the evaluator could not compute and filled from the file's saved values.
fn fallback_warnings(evaluator: &mut Evaluator<'_>, scene: &mut Scene) {
    let (messages, count) = evaluator.take_fallback_warnings();
    let shown = messages.len();
    scene.warnings.extend(messages);
    if count > shown {
        scene.warnings.push(format!("{} more drawing recovery warnings", count - shown));
    }
}
/// Render a page using a reusable index; evaluation effects are reset at the boundary.
pub fn page_prepared(evaluator: &mut Evaluator<'_>, index: usize, options: Options) -> Result<Scene, String> {
    page_prepared_budgeted(evaluator, index, options, &mut SceneBudget::default())
}
fn page_prepared_budgeted(evaluator: &mut Evaluator<'_>, index: usize, options: Options, budget: &mut SceneBudget) -> Result<Scene, String> {
    if evaluator.text_metrics_deferred() {
        return Err("deferred text metrics cannot render".into());
    }
    evaluator.enable_render_diagnostics();
    evaluator.reset_evaluation();
    let document = evaluator.document;
    let p = document.pages.get(index).ok_or("page index out of range")?;
    let scope = Scope::Page(p.id);
    let ratio = evaluator.render_number(scope, "PageScale")? / evaluator.render_number(scope, "DrawingScale")?.max(1e-12);
    let mut scene = Scene {
        warnings: vec![],
        ops: vec![],
        bounds: Rect::ZERO,
        page_size: (evaluator.render_number(scope, "PageWidth")? * ratio, evaluator.render_number(scope, "PageHeight")? * ratio),
    };
    let mut pages = vec![];
    let mut current = p;
    loop {
        if pages.contains(&current.id) || pages.len() >= 128 {
            return Err("background page cycle or depth limit".into());
        }
        pages.push(current.id);
        let Some(id) = current.attributes.get("BackPage").and_then(|s| s.parse::<u32>().ok()) else {
            break;
        };
        current = document.pages.iter().find(|p| p.id == id).ok_or("missing background page")?;
    }
    for id in pages.into_iter().rev() {
        let p = document.pages.iter().find(|p| p.id == id).ok_or("missing page")?;
        let scope = Scope::Page(id);
        let ratio = evaluator.render_number(scope, "PageScale")? / evaluator.render_number(scope, "DrawingScale")?.max(1e-12);
        shapes(evaluator, &p.shapes, scope, &p.part, Affine::scale(ratio), options, &mut scene, 0, budget)?;
    }
    fallback_warnings(evaluator, &mut scene);
    budget.charge(std::mem::size_of::<Scene>().saturating_add(scene.warnings.iter().map(String::capacity).sum::<usize>()))?;
    finish_bounds(&mut scene, options.crop);
    if options.visio_export && p.shapes.is_empty() && !p.attributes.contains_key("BackPage") {
        scene.bounds = Rect::new(0., 0., 5., 3.);
    }
    scene.canvas(options)?;
    Ok(scene)
}
pub fn master(document: &Document, index: usize, options: Options) -> Result<Scene, String> {
    let themes = misterdiagram_theme::Themes::from_document(document).map_err(|e| e.to_string())?;
    let host = misterdiagram_text::Host;
    let evaluator = Evaluator::new(document).map_err(|e| e.to_string())?.with_theme_host(&themes).with_text_host(&host);
    let mut evaluator = if options.cached_fallback { evaluator.with_cached_fallback() } else { evaluator };
    master_prepared(&mut evaluator, index, options)
}
/// Render a master using a reusable index; no state leaks from earlier masters.
pub fn master_prepared(evaluator: &mut Evaluator<'_>, index: usize, options: Options) -> Result<Scene, String> {
    if evaluator.text_metrics_deferred() {
        return Err("deferred text metrics cannot render".into());
    }
    evaluator.enable_render_diagnostics();
    evaluator.reset_evaluation();
    let document = evaluator.document;
    let m = document.masters.get(index).ok_or("master index out of range")?;
    let mut scene = Scene { warnings: vec![], ops: vec![], bounds: Rect::ZERO, page_size: (1., 1.) };
    let scope = Scope::Master(m.id);
    let ratio = evaluator.render_number(scope, "PageScale")? / evaluator.render_number(scope, "DrawingScale")?.max(1e-12);
    shapes(evaluator, &m.shapes, scope, &m.part, Affine::scale(ratio), options, &mut scene, 0, &mut SceneBudget::default())?;
    fallback_warnings(evaluator, &mut scene);
    finish_bounds(&mut scene, true);
    if options.visio_export && m.shapes.is_empty() {
        scene.bounds = Rect::new(0., 0., 5., 3.);
    }
    // A master has no raster allocation until its contact-cell scale is known.
    // raster::render validates the fitted canvas before allocating pixels.
    Ok(scene)
}
fn finish_bounds(scene: &mut Scene, crop: bool) {
    let mut bounds: Option<Rect> = None;
    let mut clips: Vec<Rect> = Vec::new();
    for op in &scene.ops {
        let b = match op {
            Op::Bounds(b) => *b,
            Op::Clip { path, .. } => {
                let b = path.bounding_box();
                clips.push(clips.last().map_or(b, |previous| previous.intersect(b)));
                continue;
            }
            Op::Pop => {
                clips.pop();
                continue;
            }
            // Geometry and foreign objects carry logical bounds separately from their device strokes.
            Op::Path { .. } | Op::Stroke { .. } => continue,
            Op::Image { pixels, transform } => {
                transform.transform_rect_bbox(Rect::new(0., 0., f64::from(pixels.width()), f64::from(pixels.height())))
            }
            Op::Text { layout, transform } => {
                let mut b: Option<Rect> = None;
                for (path, _) in text_paths(layout, *transform) {
                    let r = path.bounding_box();
                    b = Some(b.map_or(r, |b| b.union(r)));
                }
                // Export bounds include the logical ascent below the baseline,
                // not just painted descenders. SVG baselines witness the origin.
                for run in &layout.runs {
                    let ascent = run.face.typographic_metrics().0 / run.face.upem * run.size;
                    if layout.para_lines.get(run.para).is_some_and(|(_, count)| *count > 1) {
                        // Wrapped paragraphs reserve the complete available line box.
                        let r = transform.transform_rect_bbox(Rect::new(
                            layout.inner.x0 - run.size * 0.5,
                            layout.inner.y0,
                            layout.inner.x1 + run.size * 0.5,
                            layout.inner.y0,
                        ));
                        b = Some(b.map_or(r, |old| old.union(r)));
                    }
                    for (gid, x, y) in &run.glyphs {
                        let outline = misterdiagram_fonts::FontDb::global().outline(&run.face, *gid);
                        let ink = outline.bounding_box();
                        let scale = run.size / run.face.upem;
                        let (left, right) = if outline.elements().is_empty() {
                            (*x, *x)
                        } else {
                            (x + ink.x0 * scale - run.size * 0.5, x + ink.x1 * scale + run.size * 0.5)
                        };
                        let r = transform.transform_rect_bbox(Rect::new(left, y - ascent, right, y + ascent));
                        b = Some(b.map_or(r, |old| old.union(r)));
                    }
                }
                let Some(b) = b else { continue };
                b
            }
        };
        let b = clips.last().map_or(b, |clip| clip.intersect(b));
        if clips.is_empty() || (b.width() > 0. && b.height() > 0.) {
            bounds = Some(bounds.map_or(b, |old| old.union(b)));
        }
    }
    scene.bounds = if crop {
        bounds.unwrap_or(Rect::new(0., 0., scene.page_size.0, scene.page_size.1))
    } else {
        Rect::new(0., 0., scene.page_size.0, scene.page_size.1)
    };
}
/// Expand the pen after transforming the centerline, keeping line weight physical.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/lineweight-cell-line-format-section
pub(crate) fn stroke_path(path: &BezPath, stroke: &kurbo::Stroke, view: Affine, dpi: f64) -> BezPath {
    let mut stroke = stroke.clone();
    stroke.width *= dpi;
    let dash_scale = view.determinant().abs().sqrt();
    stroke.dash_offset *= dash_scale;
    for dash in &mut stroke.dash_pattern {
        *dash *= dash_scale;
    }
    let path = view * path;
    kurbo::stroke(path.elements().iter().copied(), &stroke, &kurbo::StrokeOpts::default(), 1e-4)
}
pub fn text_paths(layout: &TextLayout, transform: Affine) -> Vec<(BezPath, Rgba)> {
    let db = misterdiagram_fonts::FontDb::global();
    let mut paths = Vec::new();
    for deco in layout.decos.iter().filter(|d| d.behind) {
        paths.push((transform * deco.rect.to_path(1e-5), deco.color));
    }
    for run in &layout.runs {
        let scale = run.size / run.face.upem.max(1.);
        let mut color = run.color;
        color.a = (f64::from(color.a) * run.alpha.clamp(0., 1.)).round() as u8;
        for &(gid, x, y) in &run.glyphs {
            let outline = db.outline(&run.face, gid);
            if outline.elements().is_empty() {
                continue;
            }
            let m = transform * Affine::translate((x, y)) * Affine::scale(scale);
            paths.push((m * outline.as_ref(), color));
        }
    }
    for deco in layout.decos.iter().filter(|d| !d.behind) {
        paths.push((transform * deco.rect.to_path(1e-5), deco.color));
    }
    paths
}
/// Visibility and print flags are tested on the assigned page layers.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/visible-cell-layer-properties-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/print-cell-layer-properties-section
fn visible(e: &mut Evaluator<'_>, scope: Scope, parent: Scope, print: bool) -> bool {
    let membership = e.evaluate(scope, "LayerMember").text();
    for index in membership.split(';').filter_map(|s| s.parse::<u32>().ok()) {
        let column = if print { "Print" } else { "Visible" };
        let name = if index == 0 { format!("Layers.{column}") } else { format!("Layers.{column}[{}]", index.saturating_add(1)) };
        if number(e, parent, &name, 1.) == 0. {
            return false;
        }
    }
    !print || number(e, scope, "NonPrinting", 0.) == 0.
}
/// Group DisplayMode controls whether group geometry/text is hidden, behind, or in front of members.
/// Text stays readable when its shape is flipped about either axis (W5).
/// https://learn.microsoft.com/en-us/office/client-developer/visio/flipx-cell-shape-transform-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/flipy-cell-shape-transform-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/displaymode-cell-group-properties-section
fn shapes(
    e: &mut Evaluator<'_>,
    items: &[Shape],
    parent: Scope,
    part: &str,
    scale: Affine,
    options: Options,
    scene: &mut Scene,
    depth: usize,
    budget: &mut SceneBudget,
) -> Result<(), String> {
    if depth > 128 {
        return Err("shape depth limit".into());
    }
    for shape in items {
        // Deleted inherited shapes are tombstones, not painting instructions.
        // https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
        if shape.sheet.attributes.get("Del").is_some_and(|d| d == "1") {
            continue;
        }
        // Guides are view-only: Visio's page export omits them (private corpus
        // field/vahegan-2026-10-26 reg.svg, MisterDiagram#107).
        // https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
        if shape.shape_type.as_deref() == Some("Guide") {
            continue;
        }
        if scene.ops.len() > 100_000 {
            return Err("render operation limit".into());
        }
        let scope = match parent {
            Scope::Page(page) => Scope::PageShape { page, shape: shape.id },
            Scope::Master(master) => Scope::MasterShape { master, shape: shape.id },
            _ => return Err("invalid shape scope".into()),
        };
        // Duplicate IDs retain the first source occurrence, as in the native imported drawing.
        if e.shape(scope).is_some_and(|indexed| !std::ptr::eq(indexed, shape)) {
            continue;
        }
        if !visible(e, scope, parent, options.print) {
            continue;
        }
        // https://learn.microsoft.com/en-us/office/client-developer/visio/displaymode-cell-group-properties-section
        // Group mode0 hides only the group, mode1 paints it behind members,
        // and mode2 paints it in front. Leaf shapes ignore this group property.
        let mode = if shape.shapes.is_empty() { 1. } else { number(e, scope, "DisplayMode", 1.) };
        if mode == 0. || mode == 2. {
            shapes(e, &shape.shapes, parent, part, scale, options, scene, depth + 1, budget)?;
        }
        if mode == 0. {
            continue;
        }
        let first_op = scene.ops.len();
        let transform = scale * e.shape_to_page(scope).map_err(|error| e.render_error(scope, "transform", error))?;
        let width = number(e, scope, "Width", 1.);
        let height = number(e, scope, "Height", 1.);
        let fill = Fill {
            pattern: number(e, scope, "FillPattern", 1.).max(0.) as u32,
            foreground: color(e, scope, "FillForegnd", "FillForegndTrans"),
            background: color(e, scope, "FillBkgnd", "FillBkgndTrans"),
        };
        let weight = line_weight(e, scope);
        let nominal_weight = weight.max(0.) / scale.determinant().abs().sqrt().max(1e-12);
        let pattern = number(e, scope, "LinePattern", 1.).max(0.) as u32;
        let line = Line {
            color: color(e, scope, "LineColor", "LineColorTrans"),
            weight: (if pattern > 1 && weight > 0. { weight } else { weight.max(1. / (96. * options.scale)) })
                / scale.determinant().abs().sqrt().max(1e-12),
            pattern,
            cap: number(e, scope, "LineCap", 0.).max(0.) as u32,
            join: number(e, scope, "LineJoin", 0.).max(0.) as u32,
            rounding: number(e, scope, "Rounding", 0.),
            compound: number(e, scope, "CompoundType", 0.).max(0.) as u32,
            begin_arrow: number(e, scope, "BeginArrow", 0.).max(0.) as u32,
            end_arrow: number(e, scope, "EndArrow", 0.).max(0.) as u32,
            begin_size: number(e, scope, "BeginArrowSize", 2.).max(0.) as u32,
            end_size: number(e, scope, "EndArrowSize", 2.).max(0.) as u32,
        };
        // Infinite lines only need to reach the page edges; clipping them there keeps a
        // dotted one inside the dash budget (MisterDiagram#107).
        let page_clip = match parent {
            Scope::Page(page) => {
                Some(Rect::new(0., 0., number(e, Scope::Page(page), "PageWidth", 0.), number(e, Scope::Page(page), "PageHeight", 0.)))
            }
            _ => None,
        };
        let clip = infinite_line_clip(e.shape_to_page(scope).map_err(|error| e.render_error(scope, "transform", error))?, page_clip);
        let geometries = e.geometry_paths(scope, clip).map_err(|error| e.render_error(scope, "geometry", error))?;
        // Visible Geometry sections form one fill, including oppositely wound holes.
        // https://learn.microsoft.com/en-us/office/client-developer/visio/geometry-section
        let mut combined_fill = BezPath::new();
        for geometry in &geometries {
            if geometry.filled() {
                combined_fill.extend(style::round_rectangle(&geometry.path, line.rounding).elements().iter().copied());
            }
        }
        if number(e, scope, "ShdwPattern", 0.) == 1. && number(e, scope, "ShapeShdwType", 1.) <= 1. {
            let mut silhouette = BezPath::new();
            let mut shadow_elements = 0usize;
            for geometry in &geometries {
                let path = style::round_rectangle(&geometry.path, line.rounding);
                if geometry.filled() && fill.pattern != 0 {
                    shadow_elements = shadow_elements.saturating_add(path.elements().len());
                }
                if geometry.stroked() && line.pattern != 0 {
                    let mut stroke = line.stroke();
                    stroke.width = nominal_weight;
                    shadow_elements = shadow_elements.saturating_add(dash_elements(&path, &stroke)?);
                }
                if shadow_elements > 4_000_000 {
                    return Err("scene path limit (shadow dash expansion)".into());
                }
            }
            for geometry in &geometries {
                let path = style::round_rectangle(&geometry.path, line.rounding);
                if geometry.filled() && fill.pattern != 0 {
                    silhouette.extend((transform * &path).elements().iter().copied());
                }
                if geometry.stroked() && line.pattern != 0 {
                    let mut stroke = line.stroke();
                    stroke.width = nominal_weight;
                    silhouette.extend(
                        (transform * kurbo::stroke(path.elements().iter().copied(), &stroke, &kurbo::StrokeOpts::default(), 1e-5))
                            .elements()
                            .iter()
                            .copied(),
                    );
                }
            }
            if !silhouette.elements().is_empty() {
                let offset = (number(e, scope, "ShapeShdwOffsetX", 0.), number(e, scope, "ShapeShdwOffsetY", 0.));
                let blur = number(e, scope, "ShapeShdwBlur", 0.).max(0.);
                let shade = color(e, scope, "ShdwForegnd", "ShdwForegndTrans");
                scene.ops.push(crate::shadow::outer(&silhouette, offset, blur, shade, 96. * options.scale)?);
            }
        }
        if fill.pattern != 0 && !combined_fill.elements().is_empty() {
            let gradient = gradient(e, scope);
            let paint = if fill.pattern == 1 && gradient.is_none() {
                Paint::Solid(fill.foreground)
            } else {
                Paint::Pattern { fill: fill.clone(), gradient, bounds: Rect::new(0., 0., width, height), transform }
            };
            scene.ops.push(Op::Path { path: transform * combined_fill, paint });
        }
        for geometry in geometries {
            let path = style::round_rectangle(&geometry.path, line.rounding);
            // Line ends belong to open paths: Visio draws none on a closed one, whatever
            // BeginArrow and EndArrow say (private corpus field/vahegan-2026-10-26
            // closed-arrow.vsdx, MisterDiagram#105).
            // https://learn.microsoft.com/en-us/office/client-developer/visio/endarrow-cell-line-format-section
            let closed = path.elements().iter().any(|el| matches!(el, kurbo::PathEl::ClosePath));
            let line = if closed { Line { begin_arrow: 0, end_arrow: 0, ..line.clone() } } else { line.clone() };
            if !path.elements().is_empty() && (geometry.filled() && fill.pattern != 0 || geometry.stroked() && line.pattern != 0) {
                let bounds = if geometry.stroked() && line.pattern != 0 {
                    let mut solid = line.stroke();
                    solid.width = nominal_weight;
                    solid.dash_pattern.clear();
                    solid.start_cap = kurbo::Cap::Round;
                    solid.end_cap = kurbo::Cap::Round;
                    if filled_marker(line.begin_arrow) {
                        solid.start_cap = kurbo::Cap::Butt;
                    }
                    if filled_marker(line.end_arrow) {
                        solid.end_cap = kurbo::Cap::Butt;
                    }
                    let b = (transform * kurbo::stroke(path.elements().iter().copied(), &solid, &kurbo::StrokeOpts::default(), 1e-5)).bounding_box();
                    if line.cap == 2 && !path.elements().iter().any(|e| matches!(e, kurbo::PathEl::ClosePath)) {
                        let extra = nominal_weight * (std::f64::consts::FRAC_1_SQRT_2 - 0.5);
                        b.inflate(extra, extra)
                    } else {
                        b
                    }
                } else {
                    (transform * &path).bounding_box()
                };
                scene.ops.push(Op::Bounds(bounds));
            }
            if geometry.stroked() && line.pattern != 0 && line.weight > 0. {
                let mut stroke = line.stroke();
                if filled_marker(line.begin_arrow) {
                    stroke.start_cap = kurbo::Cap::Butt;
                }
                if filled_marker(line.end_arrow) {
                    stroke.end_cap = kurbo::Cap::Butt;
                }
                let shaft = style::marker_shaft(
                    &path,
                    style::marker_setback(line.begin_arrow, line.begin_size, line.weight),
                    style::marker_setback(line.end_arrow, line.end_size, line.weight),
                );
                let physical_scale = scale.determinant().abs().sqrt();
                stroke.width *= physical_scale;
                stroke.dash_offset *= physical_scale;
                for dash in &mut stroke.dash_pattern {
                    *dash *= physical_scale;
                }
                if line.compound == 1 {
                    // Double lines have a transparent central third, not a white overpaint.
                    // https://learn.microsoft.com/en-us/office/client-developer/visio/compoundtype-cell-line-format-section
                    let shaft = transform * shaft;
                    if dash_elements(&shaft, &stroke)?.saturating_mul(2) > 4_000_000 {
                        return Err("scene path limit (compound dash expansion)".into());
                    }
                    let mut outline = kurbo::stroke(shaft.elements().iter().copied(), &stroke, &kurbo::StrokeOpts::default(), 1e-5);
                    stroke.width /= 3.;
                    let inner = kurbo::stroke(shaft.elements().iter().copied(), &stroke, &kurbo::StrokeOpts::default(), 1e-5).reverse_subpaths();
                    outline.extend(inner.elements().iter().copied());
                    scene.ops.push(Op::Path { path: outline, paint: Paint::Solid(line.color) });
                } else {
                    scene.ops.push(Op::Stroke { path: transform * shaft, stroke, color: line.color });
                }
                let endpoints: Vec<Point> = path
                    .elements()
                    .iter()
                    .filter_map(|el| match el {
                        kurbo::PathEl::MoveTo(p) | kurbo::PathEl::LineTo(p) | kurbo::PathEl::QuadTo(_, p) | kurbo::PathEl::CurveTo(_, _, p) => {
                            Some(*p)
                        }
                        _ => None,
                    })
                    .collect();
                for (kind, size, pair, reverse) in [
                    (line.begin_arrow, line.begin_size, endpoints.get(..2), true),
                    (line.end_arrow, line.end_size, endpoints.get(endpoints.len().saturating_sub(2)..), false),
                ] {
                    if let Some([a, b]) = pair {
                        let (tip, dir) = if reverse { (*a, (*a - *b).atan2()) } else { (*b, (*b - *a).atan2()) };
                        for arrow in style::arrow(kind, size, line.weight, tip, dir) {
                            if arrow.filled {
                                scene.ops.push(Op::Bounds((transform * &arrow.path).bounding_box()));
                                scene.ops.push(Op::Path { path: transform * &arrow.path, paint: Paint::Solid(line.color) });
                            }
                            if arrow.stroke {
                                let p = kurbo::stroke(
                                    arrow.path.elements().iter().copied(),
                                    &kurbo::Stroke::new(line.weight),
                                    &kurbo::StrokeOpts::default(),
                                    1e-5,
                                );
                                scene.ops.push(Op::Bounds((transform * &p).bounding_box()));
                                scene.ops.push(Op::Path { path: transform * p, paint: Paint::Solid(line.color) });
                            }
                        }
                    }
                }
            }
        }
        if let Some((source, foreign)) = foreign_source(e, scope)? {
            let owner = match source {
                Scope::MasterShape { master, .. } => {
                    e.document.masters.iter().find(|m| m.id == master).map(|m| m.part.as_str()).ok_or("foreign master part missing")?
                }
                _ => part,
            };
            foreign_image(e, scope, owner, foreign, transform, width, height, scene, options.cached_fallback)?;
        }
        if number(e, scope, "HideText", 0.) == 0. {
            let body = e.text_body(scope).map_err(|error| e.render_error(scope, "text", error))?;
            if body.paragraphs.iter().flat_map(|p| &p.runs).any(|r| !r.text.is_empty()) {
                let block = e.text_block(scope).map_err(|error| e.render_error(scope, "text block", error))?;
                let drawing_scale = scale.determinant().abs().sqrt().max(1e-12);
                let original_height = block.height * drawing_scale;
                let measured =
                    misterdiagram_model::text::TextHost::measure(&misterdiagram_text::Host, &block.body, Some(block.width * drawing_scale))
                        .map_err(|error| e.render_error(scope, "text metrics (TxtWidth/Char.Size)", error))?;
                let height = original_height.max(measured.1);
                let shift = match block.body.body.anchor.unwrap_or(misterdiagram_model::text::Anchor::Middle) {
                    misterdiagram_model::text::Anchor::Top => 0.,
                    misterdiagram_model::text::Anchor::Bottom => height - original_height,
                    _ => (height - original_height) * 0.5,
                };
                // W5: text counter-flips about its block center for X, Y and both axes.
                // https://learn.microsoft.com/en-us/office/client-developer/visio/flipx-cell-shape-transform-section
                // https://learn.microsoft.com/en-us/office/client-developer/visio/flipy-cell-shape-transform-section
                let flip_x = if number(e, scope, "FlipX", 0.) == 0. { 1. } else { -1. };
                let flip_y = if number(e, scope, "FlipY", 0.) == 0. { 1. } else { -1. };
                let counter_flip = Affine::translate((block.width * 0.5, block.height * 0.5))
                    * Affine::scale_non_uniform(flip_x, flip_y)
                    * Affine::translate((-block.width * 0.5, -block.height * 0.5));
                let text_transform = scale * block.to_page * counter_flip * Affine::scale(1. / drawing_scale) * Affine::translate((0., -shift));
                let layout = misterdiagram_text::layout_for_export(
                    &block.body,
                    &Opts { rect: Rect::new(0., 0., block.width * drawing_scale, height), fields: &NoFields, prompt_color: None, no_shrink: false },
                )
                .map_err(|error| e.render_error(scope, "text layout (TxtWidth/Char.Size)", error))?;
                if !layout.runs.is_empty() {
                    if height > original_height {
                        let x = block.width * drawing_scale * 0.5;
                        scene.ops.push(Op::Bounds(text_transform.transform_rect_bbox(Rect::new(x, 0., x, height))));
                    }
                    if line.pattern == 0 && fill.pattern == 0 {
                        // Text-only shapes reserve a full line advance on both sides
                        // of the baseline, rather than the face's typographic ascent.
                        // Witnesses: master/from-shape label and the SAN fabric labels.
                        // https://learn.microsoft.com/en-us/office/vba/api/visio.page.export
                        for line in &layout.lines {
                            let advance = line.bottom - line.top;
                            scene.ops.push(Op::Bounds(text_transform.transform_rect_bbox(Rect::new(
                                line.extent.0,
                                line.baseline - advance,
                                line.extent.1,
                                line.baseline + advance,
                            ))));
                        }
                    }
                    scene.ops.push(Op::Text { layout, transform: text_transform });
                }
            }
        }
        budget.charge_ops(scene.ops.get(first_op..).ok_or("scene operation range")?)?;
        if mode == 1. {
            shapes(e, &shape.shapes, parent, part, scale, options, scene, depth + 1, budget)?;
        }
    }
    Ok(())
}
/// ForeignData inherits from the corresponding master shape, with local data first.
/// Its relationship belongs to the provider part; placement cells belong to the instance.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/foreigndata-element-shapesheet_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/rel-element-foreigndata_type-complextypevisio-xml
fn foreign_source<'a>(e: &Evaluator<'a>, mut scope: Scope) -> Result<Option<(Scope, &'a misterdiagram_model::XmlElement)>, String> {
    for _ in 0..=128 {
        let shape = e.shape(scope).ok_or("foreign source shape missing")?;
        if let Some(foreign) = &shape.foreign_data {
            return Ok(Some((scope, foreign)));
        }
        let Some(master) = e.master_source(scope) else { return Ok(None) };
        scope = master;
    }
    Err("foreign master inheritance depth limit".into())
}
/// Foreign objects use their related media part; thumbnails remain opaque package data.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/foreigndata-element-shapesheet_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/imgwidth-cell-foreign-image-info-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/imgheight-cell-foreign-image-info-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/imgoffsetx-cell-foreign-image-info-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/imgoffsety-cell-foreign-image-info-section
fn foreign_image(
    e: &mut Evaluator<'_>,
    scope: Scope,
    part: &str,
    foreign: &misterdiagram_model::XmlElement,
    transform: Affine,
    width: f64,
    height: f64,
    scene: &mut Scene,
    recover: bool,
) -> Result<(), String> {
    let w = number(e, scope, "ImgWidth", width);
    let h = number(e, scope, "ImgHeight", height);
    let x = number(e, scope, "ImgOffsetX", 0.);
    let y = number(e, scope, "ImgOffsetY", 0.);
    if w == 0. || h == 0. || width == 0. || height == 0. {
        return Ok(());
    }
    let object_type: String = foreign.attr("ObjectType").unwrap_or("unspecified").chars().take(64).filter(|c| !c.is_control()).collect();
    let label = format!("{} ForeignData ObjectType={object_type}", e.scope_label(scope));
    let target = foreign_target(e.document, part, foreign);
    let result = match target.as_ref() {
        Ok(target) => paint_foreign(e, scope, target, transform, width, height, w, h, x, y, scene, false),
        Err(error) => Err(error.clone()),
    };
    let Err(error) = result else { return Ok(()) };
    if !recover || foreign_resource_failure(&error) {
        return Err(format!("{label}: {error}"));
    }
    // The object's own image relationship identifies the cached presentation.
    // https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/c5a5df0c-02e6-4e72-9042-d5766f614cf5
    let mut preview_limit = false;
    let mut preview_error = None;
    if let Ok(target) = target
        && let Some(object) = e.document.parts.get(target)
    {
        for (attempted, picture) in object
            .relationships
            .iter()
            .filter(|r| !r.external && r.type_uri == "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image")
            .enumerate()
        {
            if attempted == 32 {
                preview_limit = true;
                break;
            }
            match paint_foreign(e, scope, &picture.target, transform, width, height, w, h, x, y, scene, true) {
                Ok(()) => {
                    let kind = if matches!(foreign.attr("ShowAsIcon"), Some("1" | "true")) { "icon" } else { "picture" };
                    scene.warnings.push(format!("{label}: {error}; drew cached {kind}"));
                    return Ok(());
                }
                Err(error) => preview_error = Some(error.chars().take(256).collect::<String>()),
            }
        }
    }
    // The sized placeholder is an explicit product recovery policy, not a native import claim.
    // https://learn.microsoft.com/en-us/office/client-developer/visio/foreigndata-element-shapesheet_type-complextypevisio-xml
    let extent = |name, fallback| {
        foreign.attr(name).and_then(|s| s.parse::<f64>().ok()).filter(|v| v.is_finite() && (0. ..=1e6).contains(v)).unwrap_or(fallback)
    };
    let rect = Rect::new(0., 0., extent("ObjectWidth", width), extent("ObjectHeight", height));
    let path = transform * rect.to_path(1e-5);
    scene.ops.push(Op::Bounds(path.bounding_box()));
    scene.ops.push(Op::Stroke { path, stroke: kurbo::Stroke::new(1. / 96.), color: Rgba::BLACK });
    let reason = if preview_limit { "preview recovery limit" } else { "no decodable cached picture" };
    let detail = preview_error.map(|error| format!("; last cached picture error: {error}")).unwrap_or_default();
    scene.warnings.push(format!("{label}: {error}; drew sized placeholder ({reason}{detail})"));
    Ok(())
}
/// Resource refusals remain errors even when an undecodable object may recover.
/// These prefixes are emitted by our bounded decoders, not read from the file.
fn foreign_resource_failure(error: &str) -> bool {
    [
        "metafile operation limit",
        "metafile byte limit",
        "export scene memory limit",
        "export operation limit",
        "image byte limit",
        "image pixel limit",
        "image decoder limit:",
    ]
    .iter()
    .any(|prefix| error.starts_with(prefix))
}
fn foreign_target<'a>(document: &'a Document, part: &str, foreign: &misterdiagram_model::XmlElement) -> Result<&'a str, String> {
    let rel = foreign.child("Rel").and_then(foreign_relationship).ok_or("foreign object lacks relationship")?;
    document
        .parts
        .get(part)
        .and_then(|p| p.relationships.iter().find(|r| r.id == *rel && !r.external))
        .map(|r| r.target.as_str())
        .ok_or_else(|| "missing foreign relationship".into())
}
fn paint_foreign(
    e: &Evaluator<'_>,
    scope: Scope,
    target: &str,
    transform: Affine,
    width: f64,
    height: f64,
    w: f64,
    h: f64,
    x: f64,
    y: f64,
    scene: &mut Scene,
    cached_picture: bool,
) -> Result<(), String> {
    let bytes = &e.document.parts.get(target).ok_or("missing foreign part")?.bytes;
    if bytes.get(40..44) == Some(&b" EMF"[..]) {
        let imported = if cached_picture { crate::metafile::import_cached_picture(bytes)? } else { crate::metafile::import(bytes)? };
        let m = transform
            * Affine::translate((x, y + h))
            * Affine::scale_non_uniform(w / imported.frame.width().max(1e-12), -h / imported.frame.height().max(1e-12))
            * Affine::translate((-imported.frame.x0, -imported.frame.y0));
        scene.ops.push(Op::Clip { path: transform * Rect::new(0., 0., width, height).to_path(1e-5), even_odd: false });
        scene.ops.push(Op::Bounds(m.transform_rect_bbox(imported.frame)));
        if !imported.ops.iter().any(|op| matches!(op, Op::Path { .. } | Op::Stroke { .. } | Op::Image { .. } | Op::Text { .. })) {
            scene.warnings.push(format!("foreign object {scope:?} {} produced no drawing", target));
        }
        scene.warnings.extend(imported.warnings);
        for op in imported.ops {
            scene.ops.push(match op {
                Op::Path { path, paint } => Op::Path { path: m * path, paint },
                Op::Clip { path, even_odd } => Op::Clip { path: m * path, even_odd },
                Op::Pop => Op::Pop,
                Op::Stroke { path, mut stroke, color } => {
                    let scale = m.determinant().abs().sqrt();
                    stroke.width *= scale;
                    for dash in &mut stroke.dash_pattern {
                        *dash *= scale;
                    }
                    Op::Stroke { path: m * path, stroke, color }
                }
                Op::Bounds(b) => Op::Bounds(m.transform_rect_bbox(b)),
                Op::Image { pixels, transform } => Op::Image { pixels, transform: m * transform },
                Op::Text { layout, transform } => Op::Text { layout, transform: m * transform },
            });
        }
        scene.ops.push(Op::Pop);
        return Ok(());
    }
    let pixels = crate::raster::decode(bytes)?;
    let m = transform * Affine::translate((x, y + h)) * Affine::scale_non_uniform(w / f64::from(pixels.width()), -h / f64::from(pixels.height()));
    scene.ops.push(Op::Clip { path: transform * Rect::new(0., 0., width, height).to_path(1e-5), even_odd: false });
    scene.ops.push(Op::Image { pixels: Arc::new(pixels), transform: m });
    scene.ops.push(Op::Pop);
    Ok(())
}

fn filled_marker(kind: u32) -> bool {
    matches!(kind, 2 | 4 | 5 | 6 | 8 | 10 | 11 | 13 | 35..=39 | 42)
}

/// Relationship attributes are identified by expanded name, independent of prefix or storage order.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/rel-element-foreigndata_type-complextypevisio-xml
fn foreign_relationship(rel: &misterdiagram_model::XmlElement) -> Option<&String> {
    rel.attributes
        .iter()
        .find(|(name, _)| {
            name.rsplit(':').next() == Some("id")
                && rel.attribute_namespaces.get(name).is_some_and(|ns| ns == "http://schemas.openxmlformats.org/officeDocument/2006/relationships")
        })
        .map(|(_, value)| value)
}

/// Bound dash expansion with the existing four-million path-element budget.
/// The control polygon bounds curve length without recursive subdivision.
fn dash_elements(path: &BezPath, stroke: &kurbo::Stroke) -> Result<usize, String> {
    if stroke.dash_pattern.is_empty() {
        return Ok(0);
    }
    let mut length = 0.;
    let mut previous = Point::ZERO;
    let mut start = Point::ZERO;
    let mut moves = 0usize;
    for element in path.elements() {
        let mut segment = |next: Point| {
            length += previous.distance(next);
            previous = next;
        };
        match *element {
            kurbo::PathEl::MoveTo(point) => {
                previous = point;
                start = point;
                moves += 1;
            }
            kurbo::PathEl::LineTo(point) => segment(point),
            kurbo::PathEl::QuadTo(control, point) => {
                segment(control);
                segment(point);
            }
            kurbo::PathEl::CurveTo(a, b, point) => {
                segment(a);
                segment(b);
                segment(point);
            }
            kurbo::PathEl::ClosePath => segment(start),
        }
    }
    let period: f64 = stroke.dash_pattern.iter().sum();
    let elements = (length / period + moves as f64 + 1.).ceil() * stroke.dash_pattern.len() as f64 * 32.;
    if !elements.is_finite() || elements < 0. || elements > 4_000_000. {
        return Err("scene path limit (dash expansion)".into());
    }
    Ok(elements as usize)
}

/// Sheet inspection validates the selected foreign parts without using drawing recovery.
/// A full numeric dump is still available even when its opaque objects are unsupported.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/foreigndata-element-shapesheet_type-complextypevisio-xml
pub fn validate_foreign_parts(document: &Document, selection: misterdiagram_vsdx::dump::Selection) -> Result<(), String> {
    use misterdiagram_vsdx::dump::Selection;
    let e = Evaluator::new(document).map_err(|error| error.to_string())?;
    let page = if let Selection::Page(n) = selection {
        Some(
            document
                .pages
                .get(usize::try_from(n.checked_sub(1).ok_or("page ordinal must start at 1")?).map_err(|_| "page ordinal too large")?)
                .ok_or("page ordinal not found")?
                .id,
        )
    } else {
        None
    };
    for scope in e.scopes() {
        let selected = match (selection, scope) {
            (Selection::All | Selection::Page(_), Scope::MasterShape { .. }) => true,
            (Selection::All, Scope::PageShape { .. }) => true,
            (Selection::Page(_), Scope::PageShape { page: p, .. }) => page == Some(p),
            (Selection::Master(n), Scope::MasterShape { master, .. }) => n == master,
            (Selection::Shape(n), Scope::PageShape { shape, .. }) => n == shape,
            _ => false,
        };
        if !selected {
            continue;
        }
        let Some((source, foreign)) = foreign_source(&e, scope)? else { continue };
        let object_type: String = foreign.attr("ObjectType").unwrap_or("unspecified").chars().take(64).filter(|c| !c.is_control()).collect();
        let result = (|| -> Result<(), String> {
            let owner = match source {
                Scope::MasterShape { master, .. } => document.masters.iter().find(|m| m.id == master).map(|m| m.part.as_str()),
                Scope::PageShape { page, .. } => document.pages.iter().find(|p| p.id == page).map(|p| p.part.as_str()),
                _ => None,
            }
            .ok_or("foreign owner part missing")?;
            let target = foreign_target(document, owner, foreign)?;
            let bytes = &document.parts.get(target).ok_or("missing foreign part")?.bytes;
            if bytes.get(40..44) == Some(&b" EMF"[..]) {
                crate::metafile::import(bytes).map(|_| ())
            } else {
                crate::raster::decode(bytes).map(|_| ())
            }
        })();
        result.map_err(|error| format!("{} ForeignData ObjectType={object_type}: {error}", e.scope_label(scope)))?;
    }
    Ok(())
}

#[cfg(test)]
mod relationship_tests {
    #[test]
    fn foreign_relationship_ignores_extension_ids() {
        let rel = misterdiagram_vsdx::parse_xml(br#"<Rel xmlns='http://schemas.microsoft.com/office/visio/2012/main' xmlns:r='http://schemas.openxmlformats.org/officeDocument/2006/relationships' xmlns:x='urn:extension' r:id='correct' x:id='wrong' id='unqualified'/>"#).unwrap();
        assert_eq!(super::foreign_relationship(&rel).map(String::as_str), Some("correct"));
        let mut extension_only = rel;
        extension_only.attributes.remove("r:id");
        assert_eq!(super::foreign_relationship(&extension_only), None);
    }
}
