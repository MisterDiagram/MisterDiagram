//! Evaluated drawing operations in y-up page inches; output transforms flip Y once.
use crate::style::{self, Fill, Line};
use diagramcraft_color::Rgba;
use diagramcraft_geom::{Affine, BezPath, Point, Rect};
use diagramcraft_model::{Document, Shape};
use diagramcraft_sheet::Value;
use diagramcraft_text::{NoFields, Opts, TextLayout};
use diagramcraft_vsdx::evaluate::{Evaluator, Scope};
use kurbo::Shape as _;
use std::sync::Arc;

pub const MAX_PIXELS: usize = 32 * 1024 * 1024;
#[derive(Clone, Debug)]
pub enum Paint {
    Solid(Rgba),
    Pattern { fill: Fill, bounds: Rect, transform: Affine },
}
#[derive(Clone, Debug)]
pub enum Op {
    Clip { path: BezPath, even_odd: bool },
    Pop,
    Path { path: BezPath, paint: Paint },
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
}
impl Default for Options {
    fn default() -> Self {
        Self { scale: 1., print: false, crop: true }
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
                Op::Path { path, .. } | Op::Clip { path, .. } => {
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
                Op::Pop => {}
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
        let w = (b.width() * dpi).ceil().max(1.);
        let h = (b.height() * dpi).ceil().max(1.);
        if w > f64::from(u16::MAX) || h > f64::from(u16::MAX) || w * h > MAX_PIXELS as f64 {
            return Err("render pixel limit".into());
        }
        Ok((w as u16, h as u16, Affine::scale_non_uniform(dpi, -dpi) * Affine::translate((-b.x0, -b.y1))))
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
                let [r, g, b] = diagramcraft_sheet::functions::default_color(n as u32).unwrap_or([0, 0, 0]);
                Rgba::rgb(r, g, b)
            })
        }
        _ => Rgba::BLACK,
    };
    c.a = ((1. - number(e, scope, trans, 0.).clamp(0., 1.)) * 255.).round() as u8;
    c
}
/// Background pages paint before the foreground. PageScale/DrawingScale convert drawing units.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/page-element-pages_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pagescale-cell-page-properties-section
pub fn page(document: &Document, index: usize, options: Options) -> Result<Scene, String> {
    let themes = diagramcraft_theme::Themes::from_document(document).map_err(|e| e.to_string())?;
    let host = diagramcraft_text::Host;
    let mut evaluator = Evaluator::new(document).map_err(|e| e.to_string())?.with_theme_host(&themes).with_text_host(&host);
    let p = document.pages.get(index).ok_or("page index out of range")?;
    let mut scene = Scene {
        warnings: vec![],
        ops: vec![],
        bounds: Rect::ZERO,
        page_size: (number(&mut evaluator, Scope::Page(p.id), "PageWidth", 8.5), number(&mut evaluator, Scope::Page(p.id), "PageHeight", 11.)),
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
        let ratio = number(&mut evaluator, scope, "PageScale", 1.) / number(&mut evaluator, scope, "DrawingScale", 1.).max(1e-12);
        shapes(&mut evaluator, &p.shapes, scope, &p.part, Affine::scale(ratio), options, &mut scene, 0)?;
    }
    finish_bounds(&mut scene, options.crop);
    scene.canvas(options)?;
    Ok(scene)
}
pub fn master(document: &Document, index: usize, options: Options) -> Result<Scene, String> {
    let themes = diagramcraft_theme::Themes::from_document(document).map_err(|e| e.to_string())?;
    let host = diagramcraft_text::Host;
    let mut evaluator = Evaluator::new(document).map_err(|e| e.to_string())?.with_theme_host(&themes).with_text_host(&host);
    let m = document.masters.get(index).ok_or("master index out of range")?;
    let mut scene = Scene { warnings: vec![], ops: vec![], bounds: Rect::ZERO, page_size: (1., 1.) };
    shapes(&mut evaluator, &m.shapes, Scope::Master(m.id), &m.part, Affine::IDENTITY, options, &mut scene, 0)?;
    finish_bounds(&mut scene, true);
    scene.canvas(options)?;
    Ok(scene)
}
fn finish_bounds(scene: &mut Scene, crop: bool) {
    let mut bounds: Option<Rect> = None;
    for op in &scene.ops {
        let b = match op {
            Op::Clip { .. } | Op::Pop => continue,
            Op::Path { path, .. } => path.bounding_box(),
            Op::Image { pixels, transform } => {
                transform.transform_rect_bbox(Rect::new(0., 0., f64::from(pixels.width()), f64::from(pixels.height())))
            }
            Op::Text { layout, transform } => {
                let mut b: Option<Rect> = None;
                for (path, _) in text_paths(layout, *transform) {
                    let r = path.bounding_box();
                    b = Some(b.map_or(r, |b| b.union(r)));
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
pub fn text_paths(layout: &TextLayout, transform: Affine) -> Vec<(BezPath, Rgba)> {
    let db = diagramcraft_fonts::FontDb::global();
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
fn shapes(
    e: &mut Evaluator<'_>,
    items: &[Shape],
    parent: Scope,
    part: &str,
    scale: Affine,
    options: Options,
    scene: &mut Scene,
    depth: usize,
) -> Result<(), String> {
    if depth > 128 {
        return Err("shape depth limit".into());
    }
    for shape in items {
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
        let transform = scale * e.shape_to_page(scope).map_err(|e| e.to_string())?;
        let width = number(e, scope, "Width", 1.);
        let height = number(e, scope, "Height", 1.);
        let fill = Fill {
            pattern: number(e, scope, "FillPattern", 1.).max(0.) as u32,
            foreground: color(e, scope, "FillForegnd", "FillForegndTrans"),
            background: color(e, scope, "FillBkgnd", "FillBkgndTrans"),
        };
        let line = Line {
            color: color(e, scope, "LineColor", "LineColorTrans"),
            weight: number(e, scope, "LineWeight", 0.01).max(0.) / scale.determinant().abs().sqrt().max(1e-12),
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
        for geometry in e.geometry_paths(scope, Rect::new(-1e4, -1e4, 1e4, 1e4)).map_err(|e| e.to_string())? {
            let path = style::round_rectangle(&geometry.path, line.rounding);
            if geometry.filled() && fill.pattern != 0 {
                let paint = if fill.pattern == 1 {
                    Paint::Solid(fill.foreground)
                } else {
                    Paint::Pattern { fill: fill.clone(), bounds: Rect::new(0., 0., width, height), transform }
                };
                scene.ops.push(Op::Path { path: transform * &path, paint });
            }
            if geometry.stroked() && line.pattern != 0 && line.weight > 0. {
                let outline = kurbo::stroke(path.elements().iter().copied(), &line.stroke(), &kurbo::StrokeOpts::default(), 1e-5);
                scene.ops.push(Op::Path { path: transform * outline, paint: Paint::Solid(line.color) });
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
                                scene.ops.push(Op::Path { path: transform * &arrow.path, paint: Paint::Solid(line.color) });
                            }
                            if arrow.stroke {
                                let p = kurbo::stroke(
                                    arrow.path.elements().iter().copied(),
                                    &kurbo::Stroke::new(line.weight),
                                    &kurbo::StrokeOpts::default(),
                                    1e-5,
                                );
                                scene.ops.push(Op::Path { path: transform * p, paint: Paint::Solid(line.color) });
                            }
                        }
                    }
                }
            }
        }
        if let Some(foreign) = &shape.foreign_data {
            foreign_image(e, scope, part, foreign, transform, width, height, scene)?;
        }
        shapes(e, &shape.shapes, parent, part, scale, options, scene, depth + 1)?;
        if number(e, scope, "HideText", 0.) == 0. {
            let block = e.text_block(scope).map_err(|e| e.to_string())?;
            let drawing_scale = scale.determinant().abs().sqrt().max(1e-12);
            let layout = diagramcraft_text::layout(
                &block.body,
                &Opts {
                    rect: Rect::new(0., 0., block.width * drawing_scale, block.height * drawing_scale),
                    fields: &NoFields,
                    prompt_color: None,
                    no_shrink: false,
                },
            )
            .map_err(|e| e.to_string())?;
            if !layout.runs.is_empty() {
                scene.ops.push(Op::Text { layout, transform: scale * block.to_page * Affine::scale(1. / drawing_scale) });
            }
        }
    }
    Ok(())
}
/// Foreign objects use their related media part; thumbnails remain opaque package data.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/foreigndata-element-shapesheet_type-complextypevisio-xml
fn foreign_image(
    e: &mut Evaluator<'_>,
    scope: Scope,
    part: &str,
    foreign: &diagramcraft_model::XmlElement,
    transform: Affine,
    width: f64,
    height: f64,
    scene: &mut Scene,
) -> Result<(), String> {
    let rel = foreign
        .child("Rel")
        .and_then(|r| r.attributes.iter().find(|(k, _)| k.rsplit(':').next() == Some("id")).map(|(_, v)| v))
        .ok_or("foreign object lacks relationship")?;
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
            * Affine::scale_non_uniform(width / imported.frame.width().max(1e-12), -height / imported.frame.height().max(1e-12));
        scene.warnings.extend(imported.warnings);
        for op in imported.ops {
            scene.ops.push(match op {
                Op::Path { path, paint } => Op::Path { path: m * path, paint },
                Op::Clip { path, even_odd } => Op::Clip { path: m * path, even_odd },
                Op::Pop => Op::Pop,
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
