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
    pub scale: f64,
    pub print: bool,
    pub crop: bool,
    /// Page.Export uses visible geometry and a fixed canvas for empty documents.
    /// https://learn.microsoft.com/en-us/office/vba/api/visio.page.export
    pub visio_export: bool,
}
impl Default for Options {
    fn default() -> Self {
        Self { scale: 1., print: false, crop: true, visio_export: false }
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
fn number(e: &mut Evaluator<'_>, scope: Scope, name: &str, default: f64) -> f64 {
    e.evaluate(scope, name).numeric().ok().filter(|n| n.is_finite() && n.abs() <= 1e6).unwrap_or(default)
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
    let mut stops = Vec::new();
    for index in 0..10 {
        let suffix = if index == 0 { String::new() } else { format!("[{}]", index + 1) };
        let name = format!("FillGradientStops.GradientStopPosition{suffix}");
        let position = number(e, scope, &name, if index == 0 { 0. } else { 1. });
        let c =
            color(e, scope, &format!("FillGradientStops.GradientStopColor{suffix}"), &format!("FillGradientStops.GradientStopColorTrans{suffix}"));
        stops.push((position, c));
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
    let mut evaluator = Evaluator::new(document).map_err(|e| e.to_string())?.with_theme_host(&themes).with_text_host(&host);
    page_prepared_budgeted(&mut evaluator, index, options, budget)
}
/// Render a page using a reusable index; evaluation effects are reset at the boundary.
pub fn page_prepared(evaluator: &mut Evaluator<'_>, index: usize, options: Options) -> Result<Scene, String> {
    page_prepared_budgeted(evaluator, index, options, &mut SceneBudget::default())
}
fn page_prepared_budgeted(evaluator: &mut Evaluator<'_>, index: usize, options: Options, budget: &mut SceneBudget) -> Result<Scene, String> {
    if evaluator.text_metrics_deferred() {
        return Err("deferred text metrics cannot render".into());
    }
    evaluator.reset_evaluation();
    let document = evaluator.document;
    let p = document.pages.get(index).ok_or("page index out of range")?;
    let scope = Scope::Page(p.id);
    let ratio = number(evaluator, scope, "PageScale", 1.) / number(evaluator, scope, "DrawingScale", 1.).max(1e-12);
    let mut scene = Scene {
        warnings: vec![],
        ops: vec![],
        bounds: Rect::ZERO,
        page_size: (number(evaluator, scope, "PageWidth", 8.5) * ratio, number(evaluator, scope, "PageHeight", 11.) * ratio),
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
        let ratio = number(evaluator, scope, "PageScale", 1.) / number(evaluator, scope, "DrawingScale", 1.).max(1e-12);
        shapes(evaluator, &p.shapes, scope, &p.part, Affine::scale(ratio), options, &mut scene, 0, budget)?;
    }
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
    let mut evaluator = Evaluator::new(document).map_err(|e| e.to_string())?.with_theme_host(&themes).with_text_host(&host);
    master_prepared(&mut evaluator, index, options)
}
/// Render a master using a reusable index; no state leaks from earlier masters.
pub fn master_prepared(evaluator: &mut Evaluator<'_>, index: usize, options: Options) -> Result<Scene, String> {
    if evaluator.text_metrics_deferred() {
        return Err("deferred text metrics cannot render".into());
    }
    evaluator.reset_evaluation();
    let document = evaluator.document;
    let m = document.masters.get(index).ok_or("master index out of range")?;
    let mut scene = Scene { warnings: vec![], ops: vec![], bounds: Rect::ZERO, page_size: (1., 1.) };
    let scope = Scope::Master(m.id);
    let ratio = number(evaluator, scope, "PageScale", 1.) / number(evaluator, scope, "DrawingScale", 1.).max(1e-12);
    shapes(evaluator, &m.shapes, scope, &m.part, Affine::scale(ratio), options, &mut scene, 0, &mut SceneBudget::default())?;
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
    for op in &scene.ops {
        let b = match op {
            Op::Bounds(b) => *b,
            Op::Clip { .. } | Op::Pop => continue,
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
        bounds = Some(bounds.map_or(b, |old| old.union(b)));
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
    stroke.dash_offset *= dpi;
    for dash in &mut stroke.dash_pattern {
        *dash *= dpi;
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
        if scene.ops.len() > 100_000 {
            return Err("render operation limit".into());
        }
        let scope = match parent {
            Scope::Page(page) => Scope::PageShape { page, shape: shape.id },
            Scope::Master(master) => Scope::MasterShape { master, shape: shape.id },
            _ => return Err("invalid shape scope".into()),
        };
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
        let transform = scale * e.shape_to_page(scope).map_err(|e| format!("shape {} transform: {e}", shape.id))?;
        let width = number(e, scope, "Width", 1.);
        let height = number(e, scope, "Height", 1.);
        let fill = Fill {
            pattern: number(e, scope, "FillPattern", 1.).max(0.) as u32,
            foreground: color(e, scope, "FillForegnd", "FillForegndTrans"),
            background: color(e, scope, "FillBkgnd", "FillBkgndTrans"),
        };
        let nominal_weight = number(e, scope, "LineWeight", 0.01).max(0.) / scale.determinant().abs().sqrt().max(1e-12);
        let line = Line {
            color: color(e, scope, "LineColor", "LineColorTrans"),
            weight: number(e, scope, "LineWeight", 0.01).max(1. / (96. * options.scale)) / scale.determinant().abs().sqrt().max(1e-12),
            pattern: number(e, scope, "LinePattern", 1.).max(0.) as u32,
            cap: number(e, scope, "LineCap", 0.).max(0.) as u32,
            join: number(e, scope, "LineJoin", 0.).max(0.) as u32,
            rounding: number(e, scope, "Rounding", 0.),
            compound: number(e, scope, "CompoundType", 0.).max(0.) as u32,
            begin_arrow: number(e, scope, "BeginArrow", 0.).max(0.) as u32,
            end_arrow: number(e, scope, "EndArrow", 0.).max(0.) as u32,
            begin_size: number(e, scope, "BeginArrowSize", 2.).max(0.) as u32,
            end_size: number(e, scope, "EndArrowSize", 2.).max(0.) as u32,
        };
        let geometries = e.geometry_paths(scope, Rect::new(-1e4, -1e4, 1e4, 1e4)).map_err(|e| format!("shape {} geometry: {e}", shape.id))?;
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
            foreign_image(e, scope, owner, foreign, transform, width, height, scene)?;
        }
        if number(e, scope, "HideText", 0.) == 0. {
            let body = e.text_body(scope).map_err(|e| format!("shape {} text: {e}", shape.id))?;
            if body.paragraphs.iter().flat_map(|p| &p.runs).any(|r| !r.text.is_empty()) {
                let block = e.text_block(scope).map_err(|e| format!("shape {} text block: {e}", shape.id))?;
                let drawing_scale = scale.determinant().abs().sqrt().max(1e-12);
                let original_height = block.height * drawing_scale;
                let measured =
                    misterdiagram_model::text::TextHost::measure(&misterdiagram_text::Host, &block.body, Some(block.width * drawing_scale))?;
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
                .map_err(|e| e.to_string())?;
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
fn foreign_image(
    e: &mut Evaluator<'_>,
    scope: Scope,
    part: &str,
    foreign: &misterdiagram_model::XmlElement,
    transform: Affine,
    width: f64,
    height: f64,
    scene: &mut Scene,
) -> Result<(), String> {
    let rel = foreign.child("Rel").and_then(foreign_relationship).ok_or("foreign object lacks relationship")?;
    let target = e
        .document
        .parts
        .get(part)
        .and_then(|p| p.relationships.iter().find(|r| r.id == *rel && !r.external))
        .ok_or("missing foreign relationship")?;
    let bytes = &e.document.parts.get(&target.target).ok_or("missing foreign part")?.bytes;
    if bytes.get(40..44) == Some(&b" EMF"[..]) {
        let imported = crate::metafile::import(bytes)?;
        let m = transform
            * Affine::translate((0., height))
            * Affine::scale_non_uniform(width / imported.frame.width().max(1e-12), -height / imported.frame.height().max(1e-12))
            * Affine::translate((-imported.frame.x0, -imported.frame.y0));
        scene.ops.push(Op::Bounds(transform.transform_rect_bbox(Rect::new(0., 0., width, height))));
        if !imported.ops.iter().any(|op| matches!(op, Op::Path { .. } | Op::Stroke { .. } | Op::Image { .. } | Op::Text { .. })) {
            scene.warnings.push(format!("foreign object {scope:?} {} produced no drawing", target.target));
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
        return Ok(());
    }
    let pixels = crate::raster::decode(bytes)?;
    let w = number(e, scope, "ImgWidth", width);
    let h = number(e, scope, "ImgHeight", height);
    let x = number(e, scope, "ImgOffsetX", 0.);
    let y = number(e, scope, "ImgOffsetY", 0.);
    let m = transform * Affine::translate((x, y + h)) * Affine::scale_non_uniform(w / f64::from(pixels.width()), -h / f64::from(pixels.height()));
    scene.ops.push(Op::Image { pixels: Arc::new(pixels), transform: m });
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
