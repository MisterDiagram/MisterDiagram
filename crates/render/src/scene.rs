//! Evaluated drawing operations in y-up page inches; output transforms flip Y once.
use crate::style::{self, Fill, Line};
use kurbo::Shape as _;
use misterdiagram_color::Rgba;
use misterdiagram_geom::{Affine, BezPath, Point, Rect};
use misterdiagram_model::{Document, Shape};
use misterdiagram_sheet::Value;
use misterdiagram_text::{NoFields, Opts, TextLayout};
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
use std::{collections::BTreeMap, sync::Arc};

mod budget;
pub mod evaluated;
pub use budget::SceneBudget;
pub use evaluated::{ConnectionSite, EvaluatedConnect, EvaluatedPage, EvaluatedShape, EvaluatedTextBlock};

pub const MAX_PIXELS: usize = 32 * 1024 * 1024;
#[derive(Clone, Debug)]
pub enum Paint {
    Solid(Rgba),
    Pattern { fill: Fill, gradient: Option<style::Gradient>, bounds: Rect, transform: Affine },
}
#[derive(Clone, Debug)]
pub enum Op {
    Bounds(Rect),
    Clip {
        path: BezPath,
        even_odd: bool,
    },
    Pop,
    Path {
        /// Alternate rule for shape Geometry fills; other paths retain nonzero.
        even_odd: bool,
        path: BezPath,
        paint: Paint,
    },
    Stroke {
        path: BezPath,
        stroke: kurbo::Stroke,
        color: Rgba,
    },
    /// Alternate vector geometry for hairline markers and compound stroke outlines.
    HairlineMarker {
        path: BezPath,
        svg_path: BezPath,
        color: Rgba,
    },
    HairlineStroke {
        path: BezPath,
        svg_path: BezPath,
        stroke: kurbo::Stroke,
        svg_width: f64,
        color: Rgba,
    },
    /// Already evaluated device-grid pixels. Raster QA places these without a second filter.
    RasterImage {
        pixels: Arc<image::RgbaImage>,
        transform: Affine,
    },
    Image {
        pixels: Arc<image::RgbaImage>,
        transform: Affine,
    },
    Text {
        /// Optional local text-only raster export extent, independent of ink and line advance.
        export_bounds: Option<Rect>,
        layout: TextLayout,
        transform: Affine,
    },
    TextBackground {
        rect: Rect,
        transform: Affine,
        color: Rgba,
    },
}
/// A PDF-only physical pen for a shape operation. Bounds and other backends
/// retain the original operation, including the raster pixel floor.
#[derive(Clone, Debug)]
pub struct PdfStroke {
    pub path: BezPath,
    pub stroke: kurbo::Stroke,
    pub compound: bool,
}
#[derive(Clone, Debug)]
pub struct Scene {
    /// A fitted page with no foreground content keeps its original canvas.
    pub fit_empty: bool,
    pub warnings: Vec<String>,
    pub ops: Vec<Op>,
    pub pdf_strokes: BTreeMap<usize, PdfStroke>,
    pub bounds: Rect,
    pub page_size: (f64, f64),
}
/// Family naming for editable SVG text.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FontNames {
    #[default]
    Fallback,
    Exact,
}
impl std::str::FromStr for FontNames {
    type Err = String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "fallback" => Ok(Self::Fallback),
            "exact" => Ok(Self::Exact),
            _ => Err("font_names must be fallback or exact".into()),
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Options {
    /// Use the foreground drawing crop for every export format.
    pub fit: bool,
    /// Additional physical inches on every side of a fitted drawing.
    pub margin: f64,
    /// SVG only: retain the historical glyph path serialization.
    pub text_outlines: bool,
    /// SVG only: exact names the document face alone; fallback adds its metric twin.
    pub font_names: FontNames,
    pub scale: f64,
    pub print: bool,
    pub crop: bool,
    /// SVG/PDF use the physical page; raster exports retain their drawing crop.
    pub page_canvas: bool,
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
        Self {
            fit: false,
            margin: 0.,
            text_outlines: false,
            font_names: FontNames::Fallback,
            scale: 1.,
            print: false,
            crop: true,
            page_canvas: false,
            visio_export: false,
            cached_fallback: false,
        }
    }
}
impl Options {
    /// Parse a bounded, nonnegative physical measurement using the ShapeSheet parser.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/about-units-of-measure-visio-shapesheet-reference
    pub fn with_fit(mut self, fit: bool, margin: Option<&str>) -> Result<Self, String> {
        if margin.is_some() && !fit {
            return Err("margin requires fit".into());
        }
        self.fit = fit;
        self.margin = if let Some(text) = margin {
            let misterdiagram_sheet::Expr::Number(value, unit) = misterdiagram_sheet::parse(text).map_err(|e| e.to_string())? else {
                return Err("margin must be a nonnegative length measurement".into());
            };
            let unit = unit.as_deref().unwrap_or("IN").to_ascii_uppercase();
            if !matches!(unit.as_str(), "IN" | "FT" | "YD" | "MI" | "MM" | "CM" | "M" | "KM" | "PT" | "P" | "D" | "C") {
                return Err("margin must use physical length units".into());
            }
            let inches = value * misterdiagram_sheet::units::factor(&unit).map_err(|e| e.to_string())?;
            if !inches.is_finite() || !(0. ..=1e6).contains(&inches) {
                return Err("margin outside [0,1000000] inches".into());
            }
            inches
        } else {
            0.
        };
        Ok(self)
    }
}
impl Scene {
    fn pdf_stroke(&mut self, pen: PdfStroke, budget: &mut SceneBudget) -> Result<(), String> {
        if self.ops.len() >= 200_000 {
            return Err("export operation limit".into());
        }
        // Include BTreeMap node overhead as well as retained path/dash storage.
        budget.charge(
            std::mem::size_of::<(usize, PdfStroke)>()
                .saturating_mul(4)
                .saturating_add(std::mem::size_of_val(pen.path.elements()).saturating_mul(2))
                .saturating_add(pen.stroke.dash_pattern.capacity().saturating_mul(std::mem::size_of::<f64>())),
        )?;
        self.pdf_strokes.insert(self.ops.len(), pen);
        Ok(())
    }
    pub fn canvas(&self, options: Options) -> Result<(u16, u16, Affine), String> {
        if options.fit {
            let (w, h, view) = self.fit_canvas(options)?;
            Ok((w.ceil() as u16, h.ceil() as u16, view))
        } else {
            self.canvas_bounds(options, self.bounds)
        }
    }
    /// Native PNG drawing extent, shared by all fitted output formats. Padding
    /// translates the existing drawing view without rescaling its geometry.
    /// https://learn.microsoft.com/en-us/office/vba/api/visio.page.export
    pub fn fit_canvas(&self, options: Options) -> Result<(f64, f64, Affine), String> {
        if !options.margin.is_finite() || !(0. ..=1e6).contains(&options.margin) {
            return Err("render margin limit".into());
        }
        let (w, h, view, pad) = if self.fit_empty {
            let (w, h, view) = self.page_canvas(Options { fit: false, ..options })?;
            (w, h, view, 0.)
        } else {
            if !options.scale.is_finite() || options.scale <= 0. || options.scale > 100. {
                return Err("render scale outside (0,100]".into());
            }
            // Freeze the native 96 dpi drawing crop before applying export scale.
            // Re-rounding and fitting at each DPI would change vector geometry.
            let (w, h, view) = self.canvas_bounds(Options { scale: 1., ..options }, self.bounds)?;
            (f64::from(w) * options.scale, f64::from(h) * options.scale, Affine::scale(options.scale) * view, options.margin * 96. * options.scale)
        };
        let w = w + 2. * pad;
        let h = h + 2. * pad;
        if w.ceil() > f64::from(u16::MAX) || h.ceil() > f64::from(u16::MAX) || w.ceil() * h.ceil() > MAX_PIXELS as f64 {
            return Err("render pixel limit".into());
        }
        Ok((w, h, Affine::translate((pad, pad)) * view))
    }
    /// Physical SVG/PDF page coordinates, without the raster crop's fitted scale.
    /// Native witnesses: field/export-canvas-2026-10-31.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/pagewidth-cell-page-properties-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/pagescale-cell-page-properties-section
    pub fn page_canvas(&self, options: Options) -> Result<(f64, f64, Affine), String> {
        let (w, h) = self.page_size;
        if !w.is_finite() || !h.is_finite() || w <= 0. || h <= 0. || w > 1e6 || h > 1e6 {
            return Err("vector page size limit".into());
        }
        // Preserve all operation, clip, coordinate and allocation guards.
        self.canvas_bounds(options, Rect::new(0., 0., w, h))?;
        let dpi = 96. * options.scale;
        Ok((w * dpi, h * dpi, Affine::new([dpi, 0., 0., -dpi, 0., h * dpi])))
    }
    pub(crate) fn svg_canvas(&self, options: Options) -> Result<(u16, u16, Affine), String> {
        let mut bounds = self.bounds;
        if options.crop {
            for op in &self.ops {
                match op {
                    Op::HairlineMarker { svg_path, .. } => bounds = bounds.union(svg_path.bounding_box()),
                    Op::HairlineStroke { svg_path, svg_width, .. } => {
                        bounds = bounds.union(svg_path.bounding_box().inflate(svg_width / 2., svg_width / 2.));
                    }
                    _ => {}
                }
            }
        }
        self.canvas_bounds(options, bounds)
    }
    /// Validate operations independently of a consumer-supplied raster canvas.
    /// The small dummy extent avoids allocating or imposing the source page's
    /// pixel dimensions while preserving every operation and clip-stack guard.
    pub fn validate_operations(&self, options: Options) -> Result<(), String> {
        self.canvas_bounds(Options { fit: false, ..options }, Rect::new(0., 0., 1. / 96., 1. / 96.)).map(|_| ())
    }
    fn canvas_bounds(&self, options: Options, bounds: Rect) -> Result<(u16, u16, Affine), String> {
        if !options.scale.is_finite() || options.scale <= 0. || options.scale > 100. {
            return Err("render scale outside (0,100]".into());
        }
        let mut depth = 0usize;
        let mut elements = 0usize;
        if self.ops.len() > 200_000 {
            return Err("scene operation limit".into());
        }
        for &index in self.pdf_strokes.keys() {
            if !matches!(
                self.ops.get(index),
                Some(Op::Stroke { .. } | Op::HairlineStroke { .. } | Op::HairlineMarker { .. } | Op::Path { paint: Paint::Solid(_), .. })
            ) {
                return Err("invalid PDF stroke operation".into());
            }
        }
        for (index, op) in self.ops.iter().enumerate() {
            let pdf = self.pdf_strokes.get(&index);
            match op {
                Op::Path { path, .. }
                | Op::Stroke { path, .. }
                | Op::Clip { path, .. }
                | Op::HairlineMarker { path, .. }
                | Op::HairlineStroke { path, .. } => {
                    let extra = match op {
                        Op::HairlineMarker { svg_path, .. } | Op::HairlineStroke { svg_path, .. } => Some(svg_path),
                        _ => None,
                    };
                    for path in std::iter::once(path).chain(extra).chain(pdf.map(|p| &p.path)) {
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
                }
                Op::Image { pixels, transform } | Op::RasterImage { pixels, transform } => {
                    if u64::from(pixels.width()) * u64::from(pixels.height()) > MAX_PIXELS as u64
                        || transform.as_coeffs().iter().any(|v| !v.is_finite())
                    {
                        return Err("scene image limit".into());
                    }
                }
                Op::TextBackground { rect, transform, .. } => {
                    let b = transform.transform_rect_bbox(*rect);
                    if [rect.x0, rect.y0, rect.x1, rect.y1, b.x0, b.y0, b.x1, b.y1].iter().any(|v| !v.is_finite() || v.abs() > 1e6)
                        || transform.as_coeffs().iter().any(|v| !v.is_finite())
                    {
                        return Err("scene text background limit".into());
                    }
                }
                Op::Text { layout, transform, export_bounds } => {
                    if let Some(rect) = export_bounds {
                        let b = transform.transform_rect_bbox(*rect);
                        if [rect.x0, rect.y0, rect.x1, rect.y1, b.x0, b.y0, b.x1, b.y1].iter().any(|v| !v.is_finite() || v.abs() > 1e6) {
                            return Err("scene text export bounds limit".into());
                        }
                    }
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
                        || layout.runs.iter().any(|r| {
                            r.svg.line >= layout.lines.len()
                                || !r.alpha.is_finite()
                                || !r.svg.spacing.is_finite()
                                || !r.svg.baseline_shift.is_finite()
                        })
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
            let original = match op {
                Op::Stroke { path, stroke, .. } | Op::HairlineStroke { path, stroke, .. } => Some((path, stroke, false)),
                _ => None,
            };
            for (path, stroke, compound) in original.into_iter().chain(pdf.map(|p| (&p.path, &p.stroke, p.compound))) {
                if !stroke.width.is_finite()
                    || !(0. ..=1e6).contains(&stroke.width)
                    || !stroke.miter_limit.is_finite()
                    || !(0. ..=1e6).contains(&stroke.miter_limit)
                    || !stroke.dash_offset.is_finite()
                    || stroke.dash_offset.abs() > 1e6
                    || stroke.dash_pattern.len() > 4096
                    || stroke.dash_pattern.iter().any(|n| !n.is_finite() || !(0. ..=1e6).contains(n))
                    || (!stroke.dash_pattern.is_empty() && stroke.dash_pattern.iter().sum::<f64>() <= 0.)
                {
                    return Err("scene stroke limit".into());
                }
                elements = elements.saturating_add(dash_elements(path, stroke)?.saturating_mul(if compound { 2 } else { 1 }));
                if elements > 4_000_000 {
                    return Err("scene path limit (dash expansion)".into());
                }
            }
            if let Op::HairlineStroke { svg_width, svg_path, stroke, .. } = op {
                if !svg_width.is_finite() || !(0. ..=1e6).contains(svg_width) {
                    return Err("scene stroke limit".into());
                }
                let svg_stroke = vector_stroke(stroke, *svg_width)?;
                elements = elements.saturating_add(dash_elements(svg_path, &svg_stroke)?);
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
        let b = bounds;
        if [b.x0, b.x1, b.y0, b.y1].iter().any(|n| !n.is_finite() || n.abs() > 1e6) {
            return Err("render bounds limit".into());
        }
        let dpi = 96. * options.scale;
        // Export extents share the rasterizer eighth-pixel device grid.
        // end-39.svg gives397.0133px; its PNG is397px, not398.
        let extent = |v: f64| ((v * dpi * 8.).round() / 8. - 1e-7).ceil().max(1.);
        // Text-driven edges use the nearest device pixel, including a downward
        // half-pixel tie. The native isolated 12/24 pt controls distinguish this
        // from the geometric eighth-pixel envelope above.
        let mut text_x = false;
        let mut text_y = false;
        if options.crop {
            for op in &self.ops {
                if let Op::Text { export_bounds: Some(rect), transform, .. } = op {
                    let text = transform.transform_rect_bbox(*rect);
                    text_x |= text.x0 == b.x0 || text.x1 == b.x1;
                    text_y |= text.y0 == b.y0 || text.y1 == b.y1;
                }
            }
        }
        let text_extent = |v: f64| (v * dpi + 0.5 - 1e-7).floor().max(1.);
        let w = if text_x { text_extent(b.width()) } else { extent(b.width()) };
        let h = if text_y { text_extent(b.height()) } else { extent(b.height()) };
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
    let [r, g, b] = e.color_rgb(scope, name).unwrap_or([0, 0, 0]);
    let mut c = Rgba::rgb(r, g, b);
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
    let rows = e.section_row_indices(scope, "FillGradient", 0);
    let direction = number(e, scope, "FillGradientDir", 0.);
    let effective_rows = if legacy_two_stored_stops(e, scope) { 2 } else { rows.len() };
    let legacy_effect = number(e, scope, "User.msvThemeEffects", 0.) == 16.;
    let legacy_endpoint = legacy_effect
        && e.effective_formula(scope, "FillGradientStops.GradientStopPosition[3]")
            .and_then(|formula| misterdiagram_sheet::parse(&formula).ok())
            .is_some_and(|expr| legacy_third_stop(&expr, 0));
    for index in rows {
        let one = index.checked_add(1)?;
        let suffix = if index == 0 { String::new() } else { format!("[{one}]") };
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
        if !themed || themed_count.is_none_or(|count| usize::try_from(index).is_ok_and(|index| index < count)) {
            stops.push((position, c));
        }
    }
    if stops.is_empty() {
        stops = fallback;
    }
    if stops.is_empty() {
        return None;
    }
    stops.sort_by(|a, b| a.0.total_cmp(&b.0));
    // Identical coincident stops do not add a blend segment. Native legacy
    // fills can retain a third row repeating the first endpoint verbatim.
    let before = stops.len();
    if legacy_endpoint {
        stops.dedup();
    }
    let effective_rows = effective_rows.saturating_sub(before.saturating_sub(stops.len()));
    let smooth = effective_rows == 2 && (0. ..=9.).contains(&direction) && direction.fract() == 0.;
    Some(style::Gradient {
        stops,
        geometry: style::GradientGeometry::from_direction(direction),
        angle: number(e, scope, "FillGradientAngle", 0.),
        smooth,
    })
}
/// Legacy fill formulas with two stored rows use the native two-stop ramp.
/// Scoped modern default slots do not add rows to these legacy fills.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/fillgradient-section
fn legacy_two_stored_stops(e: &Evaluator<'_>, scope: Scope) -> bool {
    let mut rows = [false; 2];
    let mut legacy = false;
    for named in e.named_cells(scope) {
        let (Some(section), Some(row)) = (named.section, named.row) else { continue };
        if !section.name.eq_ignore_ascii_case("FillGradient") || section.index.unwrap_or(0) != 0 {
            continue;
        }
        let index = row.index.or_else(|| section.rows.iter().position(|r| std::ptr::eq(r, row)).and_then(|i| u32::try_from(i).ok()));
        let Some(present) = index.and_then(|i| usize::try_from(i).ok()).and_then(|i| rows.get_mut(i)) else { return false };
        *present = true;
        legacy |= named.cell.name == "GradientStopColor"
            && e.effective_formula(scope, &named.name)
                .and_then(|f| misterdiagram_sheet::parse(&f).ok())
                .is_some_and(|expr| legacy_fill_color_call(&expr, 0));
    }
    rows == [true, true] && legacy
}
fn legacy_fill_color_call(expr: &misterdiagram_sheet::Expr, depth: usize) -> bool {
    use misterdiagram_sheet::Expr;
    if depth > 64 {
        return false;
    }
    match expr {
        Expr::Call { name, args } => {
            name == "THEME"
                && args.len() == 1
                && matches!(args.first(), Some(Expr::Text(property)) if property.eq_ignore_ascii_case("FillColor") || property.eq_ignore_ascii_case("FillColor2"))
                || args.iter().any(|arg| legacy_fill_color_call(arg, depth + 1))
        }
        Expr::Unary(_, value) | Expr::Qualified(value, _) => legacy_fill_color_call(value, depth + 1),
        Expr::Binary(_, a, b) => legacy_fill_color_call(a, depth + 1) || legacy_fill_color_call(b, depth + 1),
        _ => false,
    }
}
/// Recognize the legacy third endpoint without treating arbitrary authored rows as unused.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/theme-function
fn legacy_third_stop(expr: &misterdiagram_sheet::Expr, depth: usize) -> bool {
    use misterdiagram_sheet::Expr;
    if depth > 64 {
        return false;
    }
    match expr {
        Expr::Call { name, args } if name == "THEME" && args.len() == 1 => {
            matches!(args.first(), Some(Expr::Text(property)) if property.eq_ignore_ascii_case("FillGradStop3Position"))
        }
        Expr::Call { name, args } if matches!(name.as_str(), "GUARD" | "THEMEGUARD") && args.len() == 1 => {
            args.first().is_some_and(|arg| legacy_third_stop(arg, depth + 1))
        }
        _ => false,
    }
}
/// Background pages paint before the foreground. PageScale/DrawingScale convert drawing units.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/page-element-pages_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pagescale-cell-page-properties-section
/// The default export selects the first foreground page; background pages are composited by BackPage.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/page-element-pages_type-complextypevisio-xml
pub fn default_page_index(document: &Document) -> Result<usize, String> {
    document.page_order().next().filter(|(_, page)| !page.is_background()).map(|(index, _)| index).ok_or_else(|| "no foreground pages".into())
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

/// Compose foreground PDF pages with one retained-scene budget and a bounded page count.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.document.exportasfixedformat
pub fn foreground_pages_budgeted(document: &Document, options: Options, budget: &mut SceneBudget) -> Result<Vec<Scene>, String> {
    let foreground = |p: &&misterdiagram_model::Page| !p.attributes.get("Background").is_some_and(|s| s == "1" || s == "true");
    if document.pages.iter().filter(foreground).take(4097).count() > 4096 {
        return Err("export page limit".into());
    }
    let mut scenes = Vec::new();
    for (index, page) in document.pages.iter().enumerate() {
        if foreground(&page) {
            scenes.push(page_budgeted(document, index, options, budget)?);
        }
    }
    Ok(scenes)
}
/// Cells the evaluator could not compute and filled from the file's saved values.
fn fallback_warnings(evaluator: &mut Evaluator<'_>, scene: &mut Scene) {
    let (mut messages, mut count) = misterdiagram_vsdx::import_warnings(evaluator.document);
    let (fallbacks, fallback_count) = evaluator.take_fallback_warnings();
    count = count.saturating_add(fallback_count);
    messages.extend(fallbacks.into_iter().take(100usize.saturating_sub(messages.len())));
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
    Ok(page_evaluated_prepared(evaluator, index, options, budget, false)?.scene)
}
fn page_evaluated_prepared(
    evaluator: &mut Evaluator<'_>,
    index: usize,
    options: Options,
    budget: &mut SceneBudget,
    retain: bool,
) -> Result<EvaluatedPage, String> {
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
        fit_empty: false,
        warnings: vec![],
        ops: vec![],
        pdf_strokes: BTreeMap::new(),
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
    let mut foreground_start = 0;
    let mut evaluated = Vec::new();
    let mut connects = Vec::new();
    for id in pages.into_iter().rev() {
        if id == p.id {
            foreground_start = scene.ops.len();
        }
        let p = document.pages.iter().find(|p| p.id == id).ok_or("missing page")?;
        if retain {
            for c in &p.connects {
                let end = match c.from_cell.as_deref() {
                    Some("BeginX") => false,
                    Some("EndX") => true,
                    _ => continue,
                };
                if connects.len() >= 100_000 {
                    return Err("export connection count limit".into());
                }
                let to_cell = c.to_cell.as_deref().unwrap_or("");
                if to_cell.len() > 65536 {
                    return Err("export connection name limit".into());
                }
                budget.charge(std::mem::size_of::<EvaluatedConnect>() * 2 + to_cell.len())?;
                connects.push(EvaluatedConnect {
                    from: Scope::PageShape { page: id, shape: c.from_sheet },
                    to: Scope::PageShape { page: id, shape: c.to_sheet },
                    end,
                    to_cell: to_cell.into(),
                });
            }
        }
        let scope = Scope::Page(id);
        let ratio = evaluator.render_number(scope, "PageScale")? / evaluator.render_number(scope, "DrawingScale")?.max(1e-12);
        let nodes = shapes(evaluator, &p.shapes, scope, &p.part, Affine::scale(ratio), options, 0, budget, retain)?;
        for node in nodes {
            if retain {
                node.charge_copy(budget)?;
                node.clone().append_to(&mut scene)?;
                evaluated.push(node);
            } else {
                node.append_to(&mut scene)?;
            }
        }
    }
    for node in &mut evaluated {
        node.resolve_connections(evaluator, budget, 0)?;
    }
    fallback_warnings(evaluator, &mut scene);
    if options.fit {
        let content = finish_bounds_from(&mut scene, true, foreground_start);
        if !content {
            scene.fit_empty = true;
            scene.warnings.push("fit: empty foreground page; original page size retained".into());
        }
    } else {
        finish_bounds(&mut scene, options.crop);
    }
    if !options.fit && options.visio_export && p.shapes.is_empty() && !p.attributes.contains_key("BackPage") {
        scene.bounds = Rect::new(0., 0., 5., 3.);
    }
    budget.charge(std::mem::size_of::<Scene>())?;
    budget.charge(scene.warnings.capacity().saturating_mul(std::mem::size_of::<String>()))?;
    for warning in &scene.warnings {
        budget.charge(warning.capacity())?;
    }
    if let Err(error) = scene.canvas(options) {
        // Retained vector evaluation may target a differently sized canvas.
        // All scene guards run before this final pixel allocation check; the
        // consumer must validate its own bounded raster before painting.
        if !retain || error != "render pixel limit" {
            return Err(error);
        }
    }
    Ok(EvaluatedPage { scene, shapes: evaluated, connects })
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
    let mut scene = Scene { fit_empty: false, warnings: vec![], ops: vec![], pdf_strokes: BTreeMap::new(), bounds: Rect::ZERO, page_size: (1., 1.) };
    let scope = Scope::Master(m.id);
    let ratio = evaluator.render_number(scope, "PageScale")? / evaluator.render_number(scope, "DrawingScale")?.max(1e-12);
    for node in shapes(evaluator, &m.shapes, scope, &m.part, Affine::scale(ratio), options, 0, &mut SceneBudget::default(), false)? {
        node.append_to(&mut scene)?;
    }
    fallback_warnings(evaluator, &mut scene);
    finish_bounds(&mut scene, true);
    if options.visio_export && m.shapes.is_empty() {
        scene.bounds = Rect::new(0., 0., 5., 3.);
    }
    // A master has no raster allocation until its contact-cell scale is known.
    // raster::render validates the fitted canvas before allocating pixels.
    Ok(scene)
}
/// Text-only raster export uses occupied font cells with half-em horizontal
/// padding and asymmetric vertical padding (quarter em above, half em below).
/// Margins, empty trailing lines and line spacing do not enlarge an isolated
/// occupied line; spacing only moves the next occupied line. Native controls:
/// field/export-canvas-2026-10-31/native-text and native-font-box.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.page.export
/// https://learn.microsoft.com/en-us/office/vba/api/visio.shape.boundingbox
fn text_export_bounds(layout: &TextLayout) -> Option<Rect> {
    let mut bounds: Option<Rect> = None;
    for run in &layout.runs {
        let Some(line) = layout.lines.get(run.svg.line) else { continue };
        let ascent = run.face.ascent / run.face.upem * run.size;
        let descent = run.face.descent / run.face.upem * run.size;
        for (_, _, baseline) in &run.glyphs {
            let rect = Rect::new(
                line.extent.0 - run.size * 0.5,
                baseline - ascent - run.size * 0.25,
                line.extent.1 + run.size * 0.5,
                baseline + descent + run.size * 0.5,
            );
            bounds = Some(bounds.map_or(rect, |old| old.union(rect)));
        }
    }
    bounds
}
fn finish_bounds(scene: &mut Scene, crop: bool) {
    finish_bounds_from(scene, crop, 0);
}
fn finish_bounds_from(scene: &mut Scene, crop: bool, start: usize) -> bool {
    let mut bounds: Option<Rect> = None;
    let mut clips: Vec<Rect> = Vec::new();
    for op in scene.ops.get(start..).unwrap_or_default() {
        let b = match op {
            Op::Bounds(b) => *b,
            Op::TextBackground { rect, transform, .. } => transform.transform_rect_bbox(*rect),
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
            Op::Path { .. } | Op::Stroke { .. } | Op::HairlineMarker { .. } | Op::HairlineStroke { .. } => continue,
            Op::Image { pixels, transform } | Op::RasterImage { pixels, transform } => {
                transform.transform_rect_bbox(Rect::new(0., 0., f64::from(pixels.width()), f64::from(pixels.height())))
            }
            Op::Text { export_bounds: Some(bounds), transform, .. } => transform.transform_rect_bbox(*bounds),
            Op::Text { layout, transform, .. } => {
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
    bounds.is_some()
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
/// Physical PDF pen, observed in field/pdf-strokes-2026-10-31 (Visio 2021).
/// Single pens have a positive minimum; double outlines use their own native
/// subminimum cap envelope. Native malformed subminimum double bands remain
/// documented separately rather than being described as matching geometry.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/lineweight-cell-line-format-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/compoundtype-cell-line-format-section
fn pdf_pen(weight: f64, compound: bool) -> f64 {
    if weight == 0. {
        0.375 / 72.
    } else if compound {
        if weight <= 0.12 / 72. { (weight * 72. / 0.06).ceil().max(1.) * 0.06 / 72. } else { weight }
    } else {
        weight.max(0.14 / 72.)
    }
}

/// SVG dash lengths follow the vector pen, including the fixed zero-weight pen.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/linepattern-cell-line-format-section
pub(crate) fn vector_stroke(stroke: &kurbo::Stroke, width: f64) -> Result<kurbo::Stroke, String> {
    if !width.is_finite() || !(0. ..=1e6).contains(&width) {
        return Err("scene stroke limit".into());
    }
    let mut out = stroke.clone();
    if !out.dash_pattern.is_empty() && stroke.width > 0. {
        let ratio = width / stroke.width;
        out.dash_offset *= ratio;
        for dash in &mut out.dash_pattern {
            *dash *= ratio;
        }
        if !out.dash_offset.is_finite()
            || out.dash_offset.abs() > 1e6
            || out.dash_pattern.iter().any(|d| !d.is_finite() || !(0. ..=1e6).contains(d))
            || out.dash_pattern.iter().sum::<f64>() <= 0.
        {
            return Err("scene stroke limit".into());
        }
    }
    out.width = width;
    Ok(out)
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
    depth: usize,
    budget: &mut SceneBudget,
    retain: bool,
) -> Result<Vec<EvaluatedShape>, String> {
    if depth > 128 {
        return Err("shape depth limit".into());
    }
    let mut result = Vec::new();
    for shape in items {
        // Deleted inherited shapes are tombstones, not painting instructions.
        // https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
        if shape.sheet.attributes.get("Del").is_some_and(|d| d == "1" || d == "true") {
            continue;
        }
        // Guides are view-only: Visio's page export omits them (private corpus
        // field/vahegan-2026-10-26 reg.svg, MisterDiagram#107).
        // https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
        if shape.shape_type.as_deref() == Some("Guide") {
            continue;
        }
        let scope = match parent {
            Scope::Page(page) => Scope::PageShape { page, shape: shape.id },
            Scope::Master(master) => Scope::MasterShape { master, shape: shape.id },
            _ => return Err("invalid shape scope".into()),
        };
        // Native field/duplicate-ids-2026-10-31 keeps the first depth-first occurrence.
        // Imported trees are repaired already; caller-built models use the same display rule.
        if !e.shape(scope).is_some_and(|indexed| std::ptr::eq(indexed, shape)) {
            continue;
        }
        if !visible(e, scope, parent, options.print) {
            continue;
        }
        // https://learn.microsoft.com/en-us/office/client-developer/visio/displaymode-cell-group-properties-section
        // Group mode0 hides only the group, mode1 paints it behind members,
        // and mode2 paints it in front. Leaf shapes ignore this group property.
        let mode = if shape.shapes.is_empty() { 1. } else { number(e, scope, "DisplayMode", 1.) };
        let mut children =
            if mode == 0. || mode == 2. { shapes(e, &shape.shapes, parent, part, scale, options, depth + 1, budget, retain)? } else { Vec::new() };
        let mut node = if mode == 0. {
            EvaluatedShape::hidden(scope, shape, scale, budget)?
        } else {
            evaluate_shape(e, shape, scope, parent, part, scale, options, mode, budget, retain)?
        };
        if mode == 1. {
            children = shapes(e, &shape.shapes, parent, part, scale, options, depth + 1, budget, retain)?;
        }
        node.children = children;
        result.push(node);
    }
    Ok(result)
}
fn evaluate_shape(
    e: &mut Evaluator<'_>,
    shape: &Shape,
    scope: Scope,
    parent: Scope,
    part: &str,
    scale: Affine,
    options: Options,
    mode: f64,
    budget: &mut SceneBudget,
    retain: bool,
) -> Result<EvaluatedShape, String> {
    let mut scene =
        Scene { fit_empty: false, warnings: Vec::new(), ops: Vec::new(), pdf_strokes: BTreeMap::new(), bounds: Rect::ZERO, page_size: (1., 1.) };
    let mut text_body = None;
    let mut text_block = None;
    let mut paint = None;
    let mut has_shadow = false;
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
        Scope::Page(page) => Some(Rect::new(0., 0., number(e, Scope::Page(page), "PageWidth", 0.), number(e, Scope::Page(page), "PageHeight", 0.))),
        _ => None,
    };
    let clip = infinite_line_clip(e.shape_to_page(scope).map_err(|error| e.render_error(scope, "transform", error))?, page_clip);
    let geometries = e.geometry_paths(scope, clip).map_err(|error| e.render_error(scope, "geometry", error))?;
    // Eligible Geometry sections form one alternate fill, independent of winding.
    // Native witness: field/flip-fill-2026-11-01. The reference defines section flags;
    // cross-section parity is an observed rendering rule.
    // https://learn.microsoft.com/en-us/office/client-developer/visio/geometry-section
    let mut combined_fill = BezPath::new();
    for geometry in &geometries {
        if geometry.filled() {
            combined_fill.extend(style::round_rectangle(&geometry.path, line.rounding).elements().iter().copied());
        }
    }
    if number(e, scope, "ShdwPattern", 0.) == 1. && number(e, scope, "ShapeShdwType", 1.) <= 1. {
        let mut shadow_strokes = BezPath::new();
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
        let shadow_fill = if fill.pattern != 0 { transform * &combined_fill } else { BezPath::new() };
        for geometry in &geometries {
            let path = style::round_rectangle(&geometry.path, line.rounding);
            if geometry.stroked() && line.pattern != 0 {
                let mut stroke = line.stroke();
                stroke.width = nominal_weight;
                shadow_strokes.extend(
                    (transform * kurbo::stroke(path.elements().iter().copied(), &stroke, &kurbo::StrokeOpts::default(), 1e-5))
                        .elements()
                        .iter()
                        .copied(),
                );
            }
        }
        if !shadow_fill.elements().is_empty() || !shadow_strokes.elements().is_empty() {
            let offset = if options.fit && number(e, scope, "ShapeShdwType", 1.) == 0. {
                // FT-01: type zero inherits the page shadow, not dormant local offsets.
                (number(e, parent, "ShdwOffsetX", 0.), number(e, parent, "ShdwOffsetY", 0.))
            } else {
                (number(e, scope, "ShapeShdwOffsetX", 0.), number(e, scope, "ShapeShdwOffsetY", 0.))
            };
            let blur = number(e, scope, "ShapeShdwBlur", 0.).max(0.);
            let shade = color(e, scope, "ShdwForegnd", "ShdwForegndTrans");
            has_shadow = true;
            scene.ops.push(crate::shadow::outer(&shadow_fill, &shadow_strokes, offset, blur, shade, 96. * options.scale)?);
        }
    }
    if fill.pattern != 0 && !combined_fill.elements().is_empty() {
        let gradient = gradient(e, scope);
        let resolved_paint = if fill.pattern == 1 && gradient.is_none() {
            Paint::Solid(fill.foreground)
        } else {
            Paint::Pattern { fill: fill.clone(), gradient, bounds: Rect::new(0., 0., width, height), transform }
        };
        scene.ops.push(Op::Path { even_odd: true, path: transform * combined_fill, paint: resolved_paint.clone() });
        paint = Some(resolved_paint);
    }
    for geometry in &geometries {
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
            // Marker units and setbacks are physical page lengths. The nominal
            // pen remains independent of page scale and the raster hairline floor.
            let physical_path = transform * &path;
            let shaft = style::marker_shaft(
                &physical_path,
                style::marker_begin_setback(line.begin_arrow, line.begin_size, weight, scale.determinant().abs().sqrt()),
                style::marker_setback(line.end_arrow, line.end_size, weight),
            );
            let physical_scale = scale.determinant().abs().sqrt();
            stroke.width *= physical_scale;
            stroke.dash_offset *= physical_scale;
            for dash in &mut stroke.dash_pattern {
                *dash *= physical_scale;
            }
            let pdf_weight = pdf_pen(weight, line.compound == 1);
            if line.compound == 1 || pdf_weight != stroke.width {
                scene.pdf_stroke(
                    PdfStroke { path: shaft.clone(), stroke: vector_stroke(&stroke, pdf_weight)?, compound: line.compound == 1 },
                    budget,
                )?;
            }
            if line.compound == 1 {
                // Double lines have a transparent central third, not a white overpaint.
                // https://learn.microsoft.com/en-us/office/client-developer/visio/compoundtype-cell-line-format-section

                if dash_elements(&shaft, &stroke)?.saturating_mul(2) > 4_000_000 {
                    return Err("scene path limit (compound dash expansion)".into());
                }
                let svg_stroke = if weight > 0. && stroke.width == weight {
                    None
                } else {
                    Some(vector_stroke(&stroke, if weight > 0. { weight } else { 0.75 / 72. })?)
                };
                let mut outline = kurbo::stroke(shaft.elements().iter().copied(), &stroke, &kurbo::StrokeOpts::default(), 1e-5);
                stroke.width /= 3.;
                let inner = kurbo::stroke(shaft.elements().iter().copied(), &stroke, &kurbo::StrokeOpts::default(), 1e-5).reverse_subpaths();
                outline.extend(inner.elements().iter().copied());
                // SVG keeps the nominal pen even when raster output needs a pixel floor.
                // Compound strokes are already outlines, so retain a separate vector outline.
                if let Some(mut svg_stroke) = svg_stroke {
                    if dash_elements(&shaft, &svg_stroke)?.saturating_mul(2) > 4_000_000 {
                        return Err("scene path limit (compound SVG dash expansion)".into());
                    }
                    let mut svg_path = kurbo::stroke(shaft.elements().iter().copied(), &svg_stroke, &kurbo::StrokeOpts::default(), 1e-5);
                    svg_stroke.width /= 3.;
                    let svg_inner =
                        kurbo::stroke(shaft.elements().iter().copied(), &svg_stroke, &kurbo::StrokeOpts::default(), 1e-5).reverse_subpaths();
                    svg_path.extend(svg_inner.elements().iter().copied());
                    scene.ops.push(Op::HairlineMarker { path: outline, svg_path, color: line.color });
                } else {
                    scene.ops.push(Op::Path { even_odd: false, path: outline, paint: Paint::Solid(line.color) });
                }
            } else if weight == 0. && (line.begin_arrow != 0 || line.end_arrow != 0) {
                let svg_weight = 0.75 / 72.;
                let svg_path = style::marker_shaft(
                    &physical_path,
                    style::marker_begin_setback(line.begin_arrow, line.begin_size, svg_weight, physical_scale),
                    style::marker_setback(line.end_arrow, line.end_size, svg_weight),
                );
                scene.ops.push(Op::HairlineStroke { path: shaft, svg_path, stroke, svg_width: svg_weight, color: line.color });
            } else if weight <= 0. || stroke.width != weight {
                // The SVG override is independent of the export scale's raster floor.
                // https://learn.microsoft.com/en-us/office/client-developer/visio/lineweight-cell-line-format-section
                let svg_width = if weight > 0. { weight } else { 0.75 / 72. };
                scene.ops.push(Op::HairlineStroke { path: shaft.clone(), svg_path: shaft, stroke, svg_width, color: line.color });
            } else {
                scene.ops.push(Op::Stroke { path: shaft, stroke, color: line.color });
            }
            let endpoints: Vec<Point> = physical_path
                .elements()
                .iter()
                .filter_map(|el| match el {
                    kurbo::PathEl::MoveTo(p) | kurbo::PathEl::LineTo(p) | kurbo::PathEl::QuadTo(_, p) | kurbo::PathEl::CurveTo(_, _, p) => Some(*p),
                    _ => None,
                })
                .collect();
            for (kind, size, pair, reverse) in [
                (line.begin_arrow, line.begin_size, endpoints.get(..2), true),
                (line.end_arrow, line.end_size, endpoints.get(endpoints.len().saturating_sub(2)..), false),
            ] {
                if let Some([a, b]) = pair {
                    let (tip, dir) = if reverse { (*a, (*a - *b).atan2()) } else { (*b, (*b - *a).atan2()) };
                    let svg_arrows = if weight == 0. { style::arrow(kind, size, 0.75 / 72., tip, dir) } else { Vec::new() };
                    for (index, arrow) in style::arrow(kind, size, weight, tip, dir).into_iter().enumerate() {
                        let outline = |arrow: &style::ArrowPart, weight| {
                            if arrow.stroke {
                                kurbo::stroke(
                                    arrow.path.elements().iter().copied(),
                                    &kurbo::Stroke::new(style::marker_weight(weight)).with_caps(kurbo::Cap::Round).with_join(kurbo::Join::Round),
                                    &kurbo::StrokeOpts::default(),
                                    1e-5,
                                )
                            } else {
                                arrow.path.clone()
                            }
                        };
                        if arrow.filled || arrow.stroke {
                            let path = outline(&arrow, weight);
                            // Native export bounds use the logical butt-ended outline;
                            // the painted open marker retains its round caps and joins.
                            let bounds = if arrow.stroke {
                                kurbo::stroke(
                                    arrow.path.elements().iter().copied(),
                                    &kurbo::Stroke::new(style::marker_weight(weight)),
                                    &kurbo::StrokeOpts::default(),
                                    1e-5,
                                )
                                .bounding_box()
                            } else {
                                path.bounding_box()
                            };
                            scene.ops.push(Op::Bounds(bounds));
                            if arrow.stroke {
                                scene.pdf_stroke(
                                    PdfStroke {
                                        path: arrow.path.clone(),
                                        stroke: kurbo::Stroke::new(pdf_pen(weight, false)).with_caps(kurbo::Cap::Round).with_join(kurbo::Join::Round),
                                        compound: false,
                                    },
                                    budget,
                                )?;
                            }
                            if let Some(svg_arrow) = svg_arrows.get(index) {
                                scene.ops.push(Op::HairlineMarker { path, svg_path: outline(svg_arrow, 0.75 / 72.), color: line.color });
                            } else {
                                scene.ops.push(Op::Path { even_odd: false, path, paint: Paint::Solid(line.color) });
                            }
                        }
                    }
                }
            }
        }
    }
    let content_start = scene.ops.len();
    if let Some((source, foreign)) = foreign_source(e, scope)? {
        let owner = match source {
            Scope::MasterShape { master, .. } => {
                e.document.masters.iter().find(|m| m.id == master).map(|m| m.part.as_str()).ok_or("foreign master part missing")?
            }
            _ => part,
        };
        foreign_image(e, scope, owner, foreign, transform, width, height, &mut scene, ForeignMode::Render(options.cached_fallback))?;
    }
    let foreign_end = scene.ops.len();
    if number(e, scope, "HideText", 0.) == 0. {
        let body = e.text_body(scope).map_err(|error| e.render_error(scope, "text", error))?;
        if body.paragraphs.iter().flat_map(|p| &p.runs).any(|r| !r.text.is_empty()) {
            let result = (|| -> Result<(), String> {
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
                // W5 and grouped witnesses: cancel reflection parity across the full ancestry.
                // Keep the transformed off-centre block center fixed, including TxtAngle.
                // https://learn.microsoft.com/en-us/office/client-developer/visio/flipx-cell-shape-transform-section
                // https://learn.microsoft.com/en-us/office/client-developer/visio/flipy-cell-shape-transform-section
                let (flip_x, flip_y) = e.text_reflections(scope).map_err(|error| e.render_error(scope, "text reflections", error))?;
                let flip_x = if flip_x { -1. } else { 1. };
                let flip_y = if flip_y { -1. } else { 1. };
                let counter_flip = Affine::translate((block.width * 0.5, block.height * 0.5))
                    * Affine::scale_non_uniform(flip_x, flip_y)
                    * Affine::translate((-block.width * 0.5, -block.height * 0.5));
                let text_transform = scale * block.to_page * counter_flip * Affine::scale(1. / drawing_scale) * Affine::translate((0., -shift));
                let layout = misterdiagram_text::layout_for_export(
                    &block.body,
                    &Opts { rect: Rect::new(0., 0., block.width * drawing_scale, height), fields: &NoFields, prompt_color: None, no_shrink: false },
                )
                .map_err(|error| e.render_error(scope, "text layout (TxtWidth/Char.Size)", error))?;
                text_block = Some(EvaluatedTextBlock {
                    rect: Rect::new(0., 0., block.width * drawing_scale, original_height),
                    to_page: text_transform * Affine::translate((0., shift)),
                    layout_rect: Rect::new(0., 0., block.width * drawing_scale, height),
                });
                text_body = Some(block.body);
                if !layout.runs.is_empty() {
                    let text_only = line.pattern == 0 && fill.pattern == 0;
                    if !text_only && height > original_height {
                        let x = block.width * drawing_scale * 0.5;
                        scene.ops.push(Op::Bounds(text_transform.transform_rect_bbox(Rect::new(x, 0., x, height))));
                    }
                    let export_bounds = if text_only { text_export_bounds(&layout) } else { None };
                    if let Some((rect, color)) = text_background(e, scope, &layout) {
                        scene.ops.push(Op::TextBackground { rect, transform: text_transform, color });
                    }
                    scene.ops.push(Op::Text { layout, transform: text_transform, export_bounds });
                }
                Ok(())
            })();
            if let Err(error) = result {
                if !retain {
                    return Err(error);
                }
                // Preserve bounded saved/evaluated runs when layout fails. Normal
                // rendering still returns its original error; only the retained
                // vector form can carry text without a raster layout.
                let drawing_scale = scale.determinant().abs().sqrt().max(1e-12);
                let rect = Rect::new(0., 0., width * 72. * drawing_scale, height * 72. * drawing_scale);
                let mut to_page =
                    transform * Affine::translate((0., height)) * Affine::scale_non_uniform(1. / (72. * drawing_scale), -1. / (72. * drawing_scale));
                if to_page.determinant() > 0. {
                    to_page *= Affine::translate((rect.width(), 0.)) * Affine::scale_non_uniform(-1., 1.);
                }
                text_block = Some(EvaluatedTextBlock { rect, to_page, layout_rect: rect });
                text_body = Some(body);
                scene.warnings.push(format!("shape {}: text layout failed; saved text retained for editable export ({error})", shape.id));
            }
        }
    }
    budget.charge_ops(&scene.ops)?;
    let mut sections = Vec::new();
    for geometry in geometries {
        let path = transform * style::round_rectangle(&geometry.path, line.rounding);
        budget.charge(
            std::mem::size_of_val(path.elements()).saturating_mul(2).saturating_add(std::mem::size_of::<misterdiagram_geom::GeometryPath>() * 2),
        )?;
        sections.push(misterdiagram_geom::GeometryPath { path, flags: geometry.flags });
    }
    let bounds = transform.transform_rect_bbox(Rect::new(0., 0., width, height));
    let physical_scale = scale.determinant().abs().sqrt();
    let mut stroke = line.stroke();
    stroke.width *= physical_scale;
    stroke.dash_offset *= physical_scale;
    for dash in &mut stroke.dash_pattern {
        *dash *= physical_scale;
    }
    let connections = Vec::new();
    let one_d = shape.shape_type.as_deref() != Some("Foreign") && e.cell(scope, "BeginX").is_some();
    let node = EvaluatedShape {
        scope,
        id: shape.id,
        master: shape.master,
        master_shape: shape.master_shape,
        group: !shape.shapes.is_empty(),
        children_first: mode == 2.,
        hidden_own: false,
        sections,
        paint,
        line: Some(line),
        stroke: Some(stroke),
        nominal_weight: weight,
        bounds,
        transform,
        drawing_scale: physical_scale,
        one_d,
        connections,
        text_body,
        text_block,
        has_shadow,
        content_start,
        foreign_end,
        drawing: scene,
        children: Vec::new(),
    };
    node.charge_metadata(budget)?;
    Ok(node)
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
enum ForeignMode {
    Render(bool),
    Inspect,
}
fn foreign_image(
    e: &mut Evaluator<'_>,
    scope: Scope,
    part: &str,
    foreign: &misterdiagram_model::XmlElement,
    transform: Affine,
    width: f64,
    height: f64,
    scene: &mut Scene,
    mode: ForeignMode,
) -> Result<(), String> {
    let inspect = matches!(mode, ForeignMode::Inspect);
    let recover = !matches!(mode, ForeignMode::Render(false));
    let object_type: String = foreign.attr("ObjectType").unwrap_or("unspecified").chars().take(64).filter(|c| !c.is_control()).collect();
    let label = format!("{} ForeignData ObjectType={object_type}", e.scope_label(scope));
    if inspect {
        let target = foreign_target(e.document, part, foreign).map_err(|error| format!("{label}: {error}"))?;
        if !e.document.parts.contains_key(target) {
            return Err(format!("{label}: missing foreign part"));
        }
    }
    // Inspection decodes even an invisible object. These temporary dimensions
    // only place discarded diagnostic operations, never the rendered drawing.
    let extent = |v| if inspect && v == 0. { 1. } else { v };
    let width = extent(width);
    let height = extent(height);
    let w = extent(number(e, scope, "ImgWidth", width));
    let h = extent(number(e, scope, "ImgHeight", height));
    let x = number(e, scope, "ImgOffsetX", 0.);
    let y = number(e, scope, "ImgOffsetY", 0.);
    if w == 0. || h == 0. || width == 0. || height == 0. {
        return Ok(());
    }
    let clip = foreign_clip(e, scope, width, height, transform)?;
    let target = foreign_target(e.document, part, foreign);
    let result = match target.as_ref() {
        Ok(target) => paint_foreign(e, scope, target, transform, &clip, w, h, x, y, scene, false),
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
            match paint_foreign(e, scope, &picture.target, transform, &clip, w, h, x, y, scene, true) {
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
/// Only local Geometry section paths may clip an image; invalid references use its border.
/// Native PNG witness: field/image-crop-2026-10-30/visio-header, build 20527.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/clippingpath-cell-foreign-image-info-section
fn foreign_clip(e: &mut Evaluator<'_>, scope: Scope, width: f64, height: f64, transform: Affine) -> Result<BezPath, String> {
    let border = Rect::new(0., 0., width, height);
    let index = match e.evaluate(scope, "ClippingPath") {
        Value::Text(name) if name.len() <= 64 => {
            let lower = name.to_ascii_lowercase();
            lower
                .strip_prefix("geometry")
                .and_then(|s| s.strip_suffix(".path"))
                .filter(|s| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()))
                .and_then(|s| s.parse::<u32>().ok())
                .and_then(|n| n.checked_sub(1))
        }
        _ => None,
    };
    if let Some(index) = index {
        match e.geometry_clip_path(scope, index, border) {
            Ok(Some(geometry)) if !geometry.path.elements().is_empty() => return Ok(transform * geometry.path),
            Err(misterdiagram_sheet::Error::Limit) => return Err(e.render_error(scope, "ClippingPath", misterdiagram_sheet::Error::Limit)),
            _ => {}
        }
    }
    Ok(transform * border.to_path(1e-5))
}

fn paint_foreign(
    e: &Evaluator<'_>,
    scope: Scope,
    target: &str,
    transform: Affine,
    clip: &BezPath,
    w: f64,
    h: f64,
    x: f64,
    y: f64,
    scene: &mut Scene,
    cached_picture: bool,
) -> Result<(), String> {
    let bytes = &e.document.parts.get(target).ok_or("missing foreign part")?.bytes;
    if crate::metafile::sniff(bytes).is_some() {
        let imported = crate::metafile::import_placed(bytes, transform, w, h, cached_picture)?;
        let m = transform
            * Affine::translate((x, y + h))
            * Affine::scale_non_uniform(w / imported.frame.width().max(1e-12), -h / imported.frame.height().max(1e-12))
            * Affine::translate((-imported.frame.x0, -imported.frame.y0));
        scene.ops.push(Op::Clip { path: clip.clone(), even_odd: false });
        scene.ops.push(Op::Bounds(m.transform_rect_bbox(imported.frame)));
        if !imported.ops.iter().any(|op| matches!(op, Op::Path { .. } | Op::Stroke { .. } | Op::Image { .. } | Op::Text { .. })) {
            scene.warnings.push(format!("foreign object {scope:?} {} produced no drawing", target));
        }
        scene.warnings.extend(imported.warnings.into_iter().map(|warning| {
            if warning == "WMF comment EMF incomplete, playing the WMF fallback" {
                match scope {
                    Scope::PageShape { shape, .. } | Scope::MasterShape { shape, .. } => format!("shape {shape}: {warning}"),
                    _ => warning,
                }
            } else {
                warning
            }
        }));
        for op in imported.ops {
            scene.ops.push(match op {
                Op::Path { even_odd, path, paint } => Op::Path { even_odd, path: m * path, paint },
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
                Op::HairlineMarker { path, svg_path, color } => Op::HairlineMarker { path: m * path, svg_path: m * svg_path, color },
                Op::HairlineStroke { path, svg_path, mut stroke, svg_width, color } => {
                    let scale = m.determinant().abs().sqrt();
                    stroke.width *= scale;
                    for dash in &mut stroke.dash_pattern {
                        *dash *= scale;
                    }
                    Op::HairlineStroke { path: m * path, svg_path: m * svg_path, stroke, svg_width: svg_width * scale, color }
                }
                Op::Bounds(b) => Op::Bounds(m.transform_rect_bbox(b)),
                Op::Image { pixels, transform } => Op::Image { pixels, transform: m * transform },
                Op::RasterImage { pixels, transform } => Op::RasterImage { pixels, transform: m * transform },
                Op::Text { layout, transform, export_bounds } => Op::Text { layout, transform: m * transform, export_bounds },
                Op::TextBackground { rect, transform, color } => Op::TextBackground { rect, transform: m * transform, color },
            });
        }
        scene.ops.push(Op::Pop);
        return Ok(());
    }
    let pixels = crate::raster::decode(bytes)?;
    let m = transform * Affine::translate((x, y + h)) * Affine::scale_non_uniform(w / f64::from(pixels.width()), -h / f64::from(pixels.height()));
    scene.ops.push(Op::Clip { path: clip.clone(), even_odd: false });
    scene.ops.push(Op::Image { pixels: Arc::new(pixels), transform: m });
    scene.ops.push(Op::Pop);
    Ok(())
}

/// A single background spans the logical line extents, excluding text margins.
/// Native synthetic witness: field/textbkgnd-2026-10-30, build 20527.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/textbkgnd-cell-text-block-format-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/textbkgndtrans-cell-text-block-format-section
fn text_background(e: &mut Evaluator<'_>, scope: Scope, layout: &TextLayout) -> Option<(Rect, Rgba)> {
    let [r, g, b] = e.color_rgb(scope, "TextBkgnd")?;
    let mut color = Rgba::rgb(r, g, b);
    color.a = ((1. - number(e, scope, "TextBkgndTrans", 0.).clamp(0., 1.)) * 255.).round() as u8;
    let rect = layout.lines.iter().map(|line| Rect::new(line.extent.0, line.top, line.extent.1, line.bottom)).reduce(|a, b| a.union(b))?;
    Some((rect, color))
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

/// Sheet inspection validates the selected foreign parts and returns the same recovery diagnostics as rendering.
/// A full numeric dump is still available even when its opaque objects are unsupported.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/foreigndata-element-shapesheet_type-complextypevisio-xml
pub fn validate_foreign_parts(document: &Document, selection: misterdiagram_vsdx::dump::Selection) -> Result<Vec<String>, String> {
    use misterdiagram_vsdx::dump::Selection;
    let mut e = Evaluator::new(document).map_err(|error| error.to_string())?;
    let page = if let Selection::Page(n) = selection {
        Some(
            document
                .pages
                .get(document.page_index(usize::try_from(n).map_err(|_| "page ordinal too large")?).ok_or("page ordinal not found")?)
                .ok_or("page ordinal not found")?
                .id,
        )
    } else {
        None
    };
    e = e.with_cached_fallback();
    let scopes: Vec<_> = e.scopes().collect();
    let mut warnings = Vec::new();
    let mut budget = SceneBudget::default();
    for scope in scopes {
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
        let owner = match source {
            Scope::MasterShape { master, .. } => document.masters.iter().find(|m| m.id == master).map(|m| m.part.as_str()),
            Scope::PageShape { page, .. } => document.pages.iter().find(|p| p.id == page).map(|p| p.part.as_str()),
            _ => None,
        }
        .ok_or("foreign owner part missing")?;
        let width = number(&mut e, scope, "Width", 1.);
        let height = number(&mut e, scope, "Height", 1.);
        let mut scene =
            Scene { fit_empty: false, warnings: vec![], ops: vec![], pdf_strokes: BTreeMap::new(), bounds: Rect::ZERO, page_size: (1., 1.) };
        foreign_image(&mut e, scope, owner, foreign, Affine::IDENTITY, width, height, &mut scene, ForeignMode::Inspect)?;
        // Discard each decoded picture immediately. Retained diagnostics share the
        // renderer's bounded budget; decoder resource refusals remain errors.
        for warning in scene.warnings {
            budget.charge(warning.capacity().saturating_add(std::mem::size_of::<String>()))?;
            warnings.push(warning);
        }
    }
    Ok(warnings)
}

#[cfg(test)]
mod relationship_tests {
    #[test]
    fn embedded_allocation_failure_cannot_be_recovered_as_a_placeholder() {
        assert!(super::foreign_resource_failure(crate::metafile::EMBEDDED_ALLOCATION_LIMIT));
    }
    #[test]
    fn foreign_relationship_ignores_extension_ids() {
        let rel = misterdiagram_vsdx::parse_xml(br#"<Rel xmlns='http://schemas.microsoft.com/office/visio/2012/main' xmlns:r='http://schemas.openxmlformats.org/officeDocument/2006/relationships' xmlns:x='urn:extension' r:id='correct' x:id='wrong' id='unqualified'/>"#).unwrap();
        assert_eq!(super::foreign_relationship(&rel).map(String::as_str), Some("correct"));
        let mut extension_only = rel;
        extension_only.attributes.remove("r:id");
        assert_eq!(super::foreign_relationship(&extension_only), None);
    }
}
