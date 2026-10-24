//! Retargeted from vectorcraft crates/metafile/src/import.rs,
//! 806b1a6972288319577d75d097c140f9bf530ce0, MIT OR Apache-2.0.
//! The metafile reader: the format parsers (`emf`, `wmf`) play their records on a [`Player`], a
//! small model of the device context they draw into (objects, transforms, the current position,
//! the path bracket, the clip), which turns what they draw into document objects.
//!
//! Fills and strokes become paths with a fill and a stroke; lines drawn one after another with the
//! same pen join into one path. Clips become clipping groups around what they clip. Images keep
//! their pixels (as PNG), text becomes point type in the font the file names. Records the reader
//! doesn't know are counted and skipped, with one warning.

mod bytes;
mod dib;
mod emf;
use crate::scene::{Op, Paint, SceneBudget};
use dib::Rgba;
use kurbo::{Affine, BezPath, PathEl, Point, Rect, Shape as _, Vec2};
use misterdiagram_color::Rgba as Color;
use std::sync::Arc;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Emf,
    Wmf,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FillRule {
    EvenOdd,
    NonZero,
}
#[derive(Clone, Debug)]
struct Clip {
    path: BezPath,
    rule: FillRule,
}
const MAX_NEST: usize = 128;
pub struct Imported {
    pub ops: Vec<Op>,
    pub frame: Rect,
    pub warnings: Vec<String>,
}
/// Most objects an import makes (a hostile file can't exhaust memory with them).
const MAX_NODES: usize = 100_000;
/// Objects and clips reaching further than this from the origin (points, well inside the canvas)
/// are left out: they come from damaged or hostile coordinates.
const LIMIT: f64 = 1e6;
/// Most nested saved device contexts.
const MAX_SAVED: usize = 1024;
/// Width of a cosmetic pen (one pixel of a 96 ppi screen), points.
const HAIRLINE: f64 = 0.75;
/// Curves made for arcs and ellipses are this close to the true curve (logical units are
/// transformed after, so it is relative).
const ARC_TOLERANCE: f64 = 0.01;

/// Raster operations.
pub(crate) const SRCCOPY: u32 = 0x00CC_0020;
const PATCOPY: u32 = 0x00F0_0021;
const WHITENESS: u32 = 0x00FF_0062;
const BLACKNESS: u32 = 0x0000_0042;

const SKIPPED_ROPS: &str = "images drawn with raster operations other than a plain copy (masks, inversions) were left out";
const HATCHES: &str = "hatched brushes are filled with their colour";
const PATTERN_BRUSHES: &str = "pattern brushes are filled grey";
const EXCLUDED_CLIPS: &str = "clip areas cut out of the clip (excluded rectangles, differences) are ignored";
const BAD_IMAGE: &str = "an image that couldn't be read was left out";
const FAR_AWAY: &str = "objects far outside the picture were left out";

/// Is `r` finite and within [`LIMIT`]?
fn sane(r: Rect) -> bool {
    [r.x0, r.y0, r.x1, r.y1].iter().all(|v| v.is_finite() && v.abs() <= LIMIT)
}

/// A pen: its style flags (`PS_*`), width in logical units, colour and dash entries.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PenObj {
    pub style: u32,
    pub width: f64,
    pub color: [u8; 3],
    /// PS_USERSTYLE entries, logical units.
    pub entries: Vec<f64>,
}

impl PenObj {
    pub const NULL: PenObj = PenObj { style: PS_NULL, width: 0.0, color: [0; 3], entries: vec![] };
    pub fn solid(color: [u8; 3]) -> Self {
        Self { style: 0, width: 0.0, color, entries: vec![] }
    }
}

pub(crate) const PS_NULL: u32 = 5;
const PS_GEOMETRIC: u32 = 0x0001_0000;

/// A brush.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum BrushObj {
    Null,
    Solid([u8; 3]),
    Hatched([u8; 3]),
    /// A bitmap pattern (written grey).
    Pattern,
}

impl BrushObj {
    /// From a LOGBRUSH's style and colour.
    pub fn from_log(style: u32, color: [u8; 3]) -> Self {
        match style {
            0 => Self::Solid(color),
            1 => Self::Null,
            2 => Self::Hatched(color),
            _ => Self::Pattern,
        }
    }
}

/// A font: its height in logical units (negative: the character height, positive: the cell
/// height), escapement (tenths of a degree), weight, italics and face.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct FontObj {
    pub height: f64,
    pub escapement: f64,
    pub weight: i32,
    pub italic: bool,
    pub face: String,
}

/// A GDI object.
#[derive(Clone, Debug)]
pub(crate) enum Obj {
    Pen(PenObj),
    Brush(BrushObj),
    Font(FontObj),
    /// Palettes, regions, colour spaces: they take a slot but don't change the drawing.
    Other,
}

/// The state records change, saved and restored as a whole.
#[derive(Clone, Debug)]
struct Dc {
    pen: PenObj,
    brush: BrushObj,
    font: FontObj,
    text_color: [u8; 3],
    text_align: u32,
    fill_rule: FillRule,
    miter: f64,
    /// Arcs run counterclockwise (the default).
    arc_ccw: bool,
    /// Logical → page (EMF world transform).
    world: Affine,
    map_mode: u32,
    win_org: Vec2,
    win_ext: Vec2,
    vp_org: Vec2,
    vp_ext: Vec2,
    clips: Vec<Arc<Clip>>,
}

impl Default for Dc {
    fn default() -> Self {
        Self {
            pen: PenObj::solid([0; 3]),
            brush: BrushObj::Solid([255; 3]),
            font: FontObj::default(),
            text_color: [0; 3],
            text_align: 0,
            fill_rule: FillRule::EvenOdd,
            miter: 10.0,
            arc_ccw: true,
            world: Affine::IDENTITY,
            map_mode: 1,
            win_org: Vec2::ZERO,
            win_ext: Vec2::new(1.0, 1.0),
            vp_org: Vec2::ZERO,
            vp_ext: Vec2::new(1.0, 1.0),
            clips: vec![],
        }
    }
}

/// How an arc closes.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum ArcKind {
    Open,
    Chord,
    Pie,
}

pub(crate) struct Player {
    kind: Kind,
    /// Device units (EMF) or logical units (WMF) → document points.
    dev_to_doc: Affine,
    /// Millimetres per device pixel (EMF's fixed mapping modes).
    mm_per_px: f64,
    dc: Dc,
    saved: Vec<Dc>,
    objects: Vec<Option<Obj>>,
    /// The lowest object slot that may be free (WMF numbering).
    /// The path being recorded (document space), whether a figure is open in it and where that
    /// figure started (logical).
    path: Option<BezPath>,
    figure: bool,
    figure_start: Point,
    /// The artboard (document space): clips holding all of it clip nothing worth keeping.
    frame: Rect,
    /// The current position (logical).
    pos: Point,
    /// Lines drawn one after another outside a path bracket, not drawn yet.
    pending: BezPath,
    out: Vec<Op>,
    budget: SceneBudget,
    failure: Option<String>,
    operation_limit: usize,
    warnings: Vec<String>,
    skipped: usize,
    /// The file holds EMF+ records (which aren't read).
    pub emf_plus: bool,
}

fn rgb([r, g, b]: [u8; 3]) -> Color {
    Color::rgb(r, g, b)
}
/// The arc of the ellipse in `r` from the ray through `start` to the ray through `end`.
pub(crate) fn arc_path(r: Rect, start: Point, end: Point, ccw: bool, kind: ArcKind) -> BezPath {
    let r = r.abs();
    let c = r.center();
    let radii = Vec2::new(r.width() / 2.0, r.height() / 2.0);
    let mut bp = BezPath::new();
    if radii.x <= 0.0 || radii.y <= 0.0 {
        return bp;
    }
    let angle = |p: Point| ((p.y - c.y) / radii.y).atan2((p.x - c.x) / radii.x);
    let (a0, a1) = (angle(start), angle(end));
    let tau = std::f64::consts::TAU;
    // Counterclockwise on a y-down device is towards smaller angles.
    let sweep = if ccw {
        let s = (a0 - a1).rem_euclid(tau);
        -(if s == 0.0 { tau } else { s })
    } else {
        let s = (a1 - a0).rem_euclid(tau);
        if s == 0.0 { tau } else { s }
    };
    let arc = kurbo::Arc { center: c, radii, start_angle: a0, sweep_angle: sweep, x_rotation: 0.0 };
    let tol = ARC_TOLERANCE * radii.x.max(radii.y).max(1e-9);
    if kind == ArcKind::Pie {
        bp.move_to(c);
        let first = c + Vec2::new(radii.x * a0.cos(), radii.y * a0.sin());
        bp.line_to(first);
        arc.append_iter(tol).for_each(|el| bp.push(el));
    } else {
        kurbo::Shape::path_elements(&arc, tol).for_each(|el| bp.push(el));
    }
    if kind != ArcKind::Open {
        bp.close_path();
    }
    bp
}

/// A rectangle with elliptical corners `rx` × `ry`.
pub(crate) fn round_rect(r: Rect, rx: f64, ry: f64) -> BezPath {
    let r = r.abs();
    let (rx, ry) = ((rx / 2.0).abs().min(r.width() / 2.0), (ry / 2.0).abs().min(r.height() / 2.0));
    if rx <= 0.0 || ry <= 0.0 {
        return r.to_path(0.01);
    }
    // A rectangle with round corners of radius `ry`, squeezed across.
    let s = ry / rx;
    let squeezed = Rect::new(r.x0 * s, r.y0, r.x1 * s, r.y1);
    let mut bp = kurbo::RoundedRect::from_rect(squeezed, ry).to_path(0.01);
    bp.apply_affine(Affine::scale_non_uniform(1.0 / s, 1.0));
    bp
}

impl Player {
    fn new(kind: Kind, dev_to_doc: Affine, mm_per_px: f64, artboard: Rect) -> Self {
        Self {
            kind,
            dev_to_doc,
            mm_per_px,
            dc: Dc::default(),
            saved: vec![],
            objects: vec![],
            path: None,
            figure: false,
            figure_start: Point::ZERO,
            frame: artboard,
            pos: Point::ZERO,
            pending: BezPath::new(),
            out: vec![],
            budget: SceneBudget::default(),
            failure: None,
            operation_limit: MAX_NODES,
            warnings: vec![],
            skipped: 0,
            emf_plus: false,
        }
    }

    pub fn warn(&mut self, w: &str) {
        if !self.warnings.iter().any(|x| x == w) {
            self.warnings.push(w.to_string());
        }
    }

    /// A record of a kind the reader doesn't know.
    pub fn skip(&mut self) {
        self.skipped += 1;
    }

    // ---------- mapping ----------

    /// Logical → document.
    fn xf(&self) -> Affine {
        if self.kind == Kind::Wmf {
            return self.dev_to_doc;
        }
        let d = &self.dc;
        let page_to_dev = match d.map_mode {
            // Fixed modes: millimetres a unit, y up.
            2..=6 => {
                let mm = [0.1, 0.01, 0.254, 0.0254, 25.4 / 1440.0][(d.map_mode - 2) as usize];
                let s = mm / self.mm_per_px.max(1e-12);
                Affine::translate(d.vp_org) * Affine::scale_non_uniform(s, -s) * Affine::translate(-d.win_org)
            }
            7 | 8 => {
                let ratio = |v: f64, w: f64| if w.abs() > 1e-12 && v.is_finite() { v / w } else { 1.0 };
                let (mut sx, mut sy) = (ratio(d.vp_ext.x, d.win_ext.x), ratio(d.vp_ext.y, d.win_ext.y));
                if d.map_mode == 7 {
                    // Isotropic: the same scale both ways, the smaller one.
                    let s = sx.abs().min(sy.abs());
                    (sx, sy) = (s.copysign(sx), s.copysign(sy));
                }
                Affine::translate(d.vp_org) * Affine::scale_non_uniform(sx, sy) * Affine::translate(-d.win_org)
            }
            _ => Affine::translate(d.vp_org - d.win_org),
        };
        self.dev_to_doc * page_to_dev * d.world
    }

    /// How much logical lengths grow in the document.
    fn scale(&self) -> f64 {
        let s = self.xf().determinant().abs().sqrt();
        if s.is_finite() { s } else { 0.0 }
    }

    pub fn set_map_mode(&mut self, m: u32) {
        self.dc.map_mode = m;
    }
    pub fn set_window_org(&mut self, x: f64, y: f64) {
        self.dc.win_org = Vec2::new(x, y);
    }
    pub fn set_window_ext(&mut self, x: f64, y: f64) {
        self.dc.win_ext = Vec2::new(x, y);
    }
    pub fn set_viewport_org(&mut self, x: f64, y: f64) {
        self.dc.vp_org = Vec2::new(x, y);
    }
    pub fn set_viewport_ext(&mut self, x: f64, y: f64) {
        self.dc.vp_ext = Vec2::new(x, y);
    }
    pub fn scale_ext(&mut self, window: bool, x: (f64, f64), y: (f64, f64)) {
        let e = if window { &mut self.dc.win_ext } else { &mut self.dc.vp_ext };
        if x.1 != 0.0 && y.1 != 0.0 {
            *e = Vec2::new(e.x * x.0 / x.1, e.y * y.0 / y.1);
        }
    }
    /// `SetWorldTransform` / `ModifyWorldTransform` (`mode`: 1 identity, 2 left, 3 right, 4 set).
    pub fn world(&mut self, x: Affine, mode: u32) {
        if !x.as_coeffs().iter().all(|v| v.is_finite()) {
            return;
        }
        self.dc.world = match mode {
            1 => Affine::IDENTITY,
            2 => self.dc.world * x,
            3 => x * self.dc.world,
            _ => x,
        };
    }

    // ---------- state ----------

    pub fn set_fill_mode(&mut self, alternate: bool) {
        self.dc.fill_rule = if alternate { FillRule::EvenOdd } else { FillRule::NonZero };
    }
    pub fn set_miter(&mut self, m: f64) {
        if m.is_finite() && m >= 1.0 {
            self.dc.miter = m;
        }
    }
    pub fn set_arc_ccw(&mut self, ccw: bool) {
        self.dc.arc_ccw = ccw;
    }
    pub fn set_text_color(&mut self, c: [u8; 3]) {
        self.dc.text_color = c;
    }
    pub fn set_text_align(&mut self, a: u32) {
        self.dc.text_align = a;
    }

    pub fn save(&mut self) {
        if self.saved.len() < MAX_SAVED {
            self.saved.push(self.dc.clone());
        }
    }

    /// `RestoreDC(n)`: a negative `n` counts back from the latest save, a positive one is a save's
    /// number.
    pub fn restore(&mut self, n: i32) {
        let len = self.saved.len();
        let keep = if n < 0 { len.checked_sub(n.unsigned_abs() as usize) } else { (n as usize).checked_sub(1).filter(|k| *k < len) };
        if let Some(k) = keep {
            self.saved.truncate(k + 1);
            if let Some(dc) = self.saved.pop() {
                self.dc = dc;
            }
        }
    }

    // ---------- objects ----------

    /// Put `obj` in slot `index` (EMF numbers its objects).
    pub fn create_at(&mut self, index: u32, obj: Obj) {
        let i = index as usize;
        if i == 0 || i > u16::MAX as usize {
            return;
        }
        if self.objects.len() <= i {
            self.objects.resize(i + 1, None);
        }
        if let Some(s) = self.objects.get_mut(i) {
            *s = Some(obj);
        }
    }

    pub fn delete(&mut self, index: u32) {
        if let Some(s) = self.objects.get_mut(index as usize) {
            *s = None;
        }
    }

    /// Select object `index`, or a stock object (EMF: the high bit set).
    pub fn select(&mut self, index: u32) {
        if self.kind == Kind::Emf && index & 0x8000_0000 != 0 {
            match index & 0x7fff_ffff {
                0 | 18 => self.dc.brush = BrushObj::Solid([255; 3]),
                1 => self.dc.brush = BrushObj::Solid([0xc0; 3]),
                2 => self.dc.brush = BrushObj::Solid([0x80; 3]),
                3 => self.dc.brush = BrushObj::Solid([0x40; 3]),
                4 => self.dc.brush = BrushObj::Solid([0; 3]),
                5 => self.dc.brush = BrushObj::Null,
                6 => self.dc.pen = PenObj::solid([255; 3]),
                7 | 19 => self.dc.pen = PenObj::solid([0; 3]),
                8 => self.dc.pen = PenObj::NULL,
                10..=17 => self.dc.font = FontObj::default(),
                _ => {}
            }
            return;
        }
        match self.objects.get(index as usize).cloned().flatten() {
            Some(Obj::Pen(p)) => self.dc.pen = p,
            Some(Obj::Brush(b)) => self.dc.brush = b,
            Some(Obj::Font(f)) => self.dc.font = f,
            _ => {}
        }
    }

    // ---------- output ----------

    fn push(&mut self, op: Op) {
        if self.failure.is_some() {
            return;
        }
        let count = self.dc.clips.len().saturating_mul(2).saturating_add(1);
        if self.out.len().saturating_add(count) > self.operation_limit {
            self.failure = Some("metafile operation limit".into());
            return;
        }
        if let Err(error) = self.budget.charge_ops(std::slice::from_ref(&op)) {
            self.failure = Some(error);
            return;
        }
        for clip in &self.dc.clips {
            let operation = Op::Clip { path: clip.path.clone(), even_odd: clip.rule == FillRule::EvenOdd };
            if let Err(error) = self.budget.charge_ops(std::slice::from_ref(&operation)) {
                self.failure = Some(error);
                return;
            }
            self.out.push(operation);
        }
        self.out.push(op);
        for _ in &self.dc.clips {
            if let Err(error) = self.budget.charge_ops(&[Op::Pop]) {
                self.failure = Some(error);
                return;
            }
            self.out.push(Op::Pop);
        }
    }
    fn fill_paint(&mut self) -> Option<Color> {
        match self.dc.brush.clone() {
            BrushObj::Null => None,
            BrushObj::Solid(c) => Some(rgb(c)),
            BrushObj::Hatched(c) => {
                self.warn(HATCHES);
                Some(rgb(c))
            }
            BrushObj::Pattern => {
                self.warn(PATTERN_BRUSHES);
                Some(rgb([0x80; 3]))
            }
        }
    }

    fn stroke_layer(&self) -> Option<kurbo::Stroke> {
        let p = &self.dc.pen;
        let kind = p.style & 0xf;
        if kind == PS_NULL {
            return None;
        }
        // Geometric pens, and other pens wider than 1, have a width in logical units; the others
        // are one pixel wide.
        let w = p.width * self.scale();
        let width = if w > 0.0 && w.is_finite() && (p.style & PS_GEOMETRIC != 0 || p.width > 1.0) { w } else { HAIRLINE };
        let mut st = kurbo::Stroke::new(width);
        // Round caps and joins unless the style asks for others.
        st.start_cap = match p.style & 0xf00 {
            0x100 => kurbo::Cap::Square,
            0x200 => kurbo::Cap::Butt,
            _ => kurbo::Cap::Round,
        };
        st.end_cap = st.start_cap;
        st.join = match p.style & 0xf000 {
            0x1000 => kurbo::Join::Bevel,
            0x2000 => kurbo::Join::Miter,
            _ => kurbo::Join::Round,
        };
        st.miter_limit = self.dc.miter;
        let unit = width.max(HAIRLINE);
        let pattern: Vec<f64> = match kind {
            1 => vec![3.0 * unit, unit],
            2 => vec![unit, unit],
            3 => vec![3.0 * unit, unit, unit, unit],
            4 => vec![3.0 * unit, unit, unit, unit, unit, unit],
            7 => p.entries.iter().map(|e| e * self.scale()).collect(),
            _ => vec![],
        };
        st = st.with_dashes(0., pattern);
        Some(st)
    }

    /// Bound expanded dash and text work before stroke construction or glyph shaping.
    /// https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-emf/
    fn check_expansion(&mut self, count: usize) -> bool {
        if self.failure.is_some() {
            return false;
        }
        let found = count.saturating_mul(self.dc.clips.len().saturating_mul(2).saturating_add(1)).saturating_add(self.out.len());
        if found > self.operation_limit {
            self.failure = Some(format!("metafile operation limit: found {found}, limit {}", self.operation_limit));
            return false;
        }
        true
    }

    fn check_dashes(&mut self, path: &BezPath, stroke: &kurbo::Stroke) -> bool {
        if stroke.dash_pattern.is_empty() {
            return true;
        }
        let period: f64 = stroke.dash_pattern.iter().sum();
        // The control polygon bounds curve length and does not require subdivision.
        let length: f64 = path
            .segments()
            .map(|segment| match segment {
                kurbo::PathSeg::Line(s) => (s.p1 - s.p0).hypot(),
                kurbo::PathSeg::Quad(s) => (s.p1 - s.p0).hypot() + (s.p2 - s.p1).hypot(),
                kurbo::PathSeg::Cubic(s) => (s.p1 - s.p0).hypot() + (s.p2 - s.p1).hypot() + (s.p3 - s.p2).hypot(),
            })
            .sum();
        let count = (length / period).ceil() * stroke.dash_pattern.len().div_ceil(2) as f64 + path.elements().len() as f64;
        if !count.is_finite() || count < 0. || count > self.operation_limit as f64 {
            self.failure = Some(format!("metafile operation limit: found {count} dash pieces, limit {}", self.operation_limit));
            return false;
        }
        self.check_expansion(count as usize)
    }

    /// Paint `bp` (document space) with the brush and the pen.
    fn draw(&mut self, bp: BezPath, fill: bool, stroke: bool) {
        if bp.elements().is_empty() {
            return;
        }
        if !sane(bp.bounding_box()) || bp.elements().len() > 100_000 {
            self.warn(FAR_AWAY);
            return;
        }
        if fill && let Some(color) = self.fill_paint() {
            let clip = Clip { path: bp.clone(), rule: self.dc.fill_rule };
            self.dc.clips.push(Arc::new(clip));
            self.push(Op::Path { path: bp.bounding_box().to_path(0.01), paint: Paint::Solid(color) });
            self.dc.clips.pop();
        }
        if stroke && let Some(st) = self.stroke_layer() {
            if !self.check_dashes(&bp, &st) {
                return;
            }
            let outline = kurbo::stroke(bp.elements().iter().copied(), &st, &kurbo::StrokeOpts::default(), 0.01);
            self.push(Op::Path { path: outline, paint: Paint::Solid(rgb(self.dc.pen.color)) });
        }
    }

    /// Draw the lines drawn one after another so far.
    pub fn flush(&mut self) {
        if !self.pending.elements().is_empty() {
            let bp = std::mem::take(&mut self.pending);
            self.draw(bp, false, true);
        }
    }

    /// A figure in logical units: added to the path being recorded, else drawn (filled when it is
    /// an area).
    pub fn figure(&mut self, mut bp: BezPath, area: bool) {
        bp.apply_affine(self.xf());
        match &mut self.path {
            Some(path) => {
                path.extend(bp);
                self.figure = false;
            }
            None => self.draw(bp, area, true),
        }
    }

    /// Polygon / Polyline (and Bézier ones): `pts` in logical units.
    pub fn poly(&mut self, pts: &[Point], closed: bool, bezier: bool) {
        let Some((first, rest)) = pts.split_first() else { return };
        let mut bp = BezPath::new();
        bp.move_to(*first);
        if bezier {
            for c in rest.as_chunks::<3>().0 {
                bp.curve_to(c[0], c[1], c[2]);
            }
        } else {
            rest.iter().for_each(|p| bp.line_to(*p));
        }
        if closed {
            bp.close_path();
        }
        self.figure(bp, closed);
    }

    /// Several polygons (one area) or polylines.
    pub fn poly_poly(&mut self, polys: &[Vec<Point>], closed: bool) {
        let mut bp = BezPath::new();
        for p in polys {
            let Some((first, rest)) = p.split_first() else { continue };
            bp.move_to(*first);
            rest.iter().for_each(|q| bp.line_to(*q));
            if closed {
                bp.close_path();
            }
        }
        self.figure(bp, closed);
    }

    pub fn move_to(&mut self, p: Point) {
        self.pos = p;
        self.figure = false;
    }

    /// LineTo / PolylineTo / PolyBezierTo: from the current position through `pts` (logical).
    pub fn poly_to(&mut self, pts: &[Point], bezier: bool) {
        let Some(last) = pts.last().copied() else { return };
        let xf = self.xf();
        let start = xf * self.pos;
        let target = match &mut self.path {
            Some(path) => {
                if !self.figure {
                    path.move_to(start);
                    self.figure = true;
                    self.figure_start = self.pos;
                }
                path
            }
            None => {
                if self.pending.elements().last().and_then(|e| e.end_point()) != Some(start) {
                    self.pending.move_to(start);
                }
                &mut self.pending
            }
        };
        if bezier {
            for c in pts.as_chunks::<3>().0 {
                target.curve_to(xf * c[0], xf * c[1], xf * c[2]);
            }
        } else {
            pts.iter().for_each(|p| target.line_to(xf * *p));
        }
        self.pos = last;
    }

    pub fn arc(&mut self, r: Rect, start: Point, end: Point, kind: ArcKind) {
        let bp = arc_path(r, start, end, self.dc.arc_ccw, kind);
        self.figure(bp, kind != ArcKind::Open);
    }

    // ---------- paths and clips ----------

    pub fn begin_path(&mut self) {
        self.path = Some(BezPath::new());
        self.figure = false;
    }
    pub fn end_path(&mut self) {
        // The path stays until it is filled, stroked or made the clip.
    }
    pub fn close_figure(&mut self) {
        if let Some(p) = &mut self.path
            && self.figure
        {
            p.close_path();
            self.figure = false;
            self.pos = self.figure_start;
        }
    }
    pub fn abort_path(&mut self) {
        self.path = None;
    }

    /// FillPath / StrokePath / StrokeAndFillPath.
    pub fn paint_path(&mut self, fill: bool, stroke: bool) {
        if let Some(mut bp) = self.path.take() {
            // Filling closes the open figure.
            if fill && self.figure {
                bp.close_path();
            }
            self.draw(bp, fill, stroke);
        }
    }

    fn push_clip(&mut self, path: BezPath, rule: FillRule, replace: bool) {
        if replace {
            self.dc.clips.clear();
        }
        if !sane(path.bounding_box()) {
            return self.warn(FAR_AWAY);
        }
        // A rectangle around the whole picture (what writers clip to first) clips nothing in it.
        let lines = path.elements().iter().all(|e| !matches!(e, PathEl::QuadTo(..) | PathEl::CurveTo(..)));
        let around = path.bounding_box().inflate(0.5, 0.5);
        if lines && path.elements().len() <= 6 && around.contains(self.frame.origin()) && around.contains(Point::new(self.frame.x1, self.frame.y1)) {
            return;
        }
        if self.dc.clips.len() >= MAX_NEST {
            return self.warn("clip depth limit");
        }
        self.dc.clips.push(Arc::new(Clip { path, rule }));
    }

    /// SelectClipPath (`mode`: 1 and, 5 copy; the others cut out, which isn't read).
    pub fn clip_path(&mut self, mode: u32) {
        let Some(bp) = self.path.take() else { return };
        match mode {
            1 => self.push_clip(bp, self.dc.fill_rule, false),
            5 => self.push_clip(bp, self.dc.fill_rule, true),
            _ => self.warn(EXCLUDED_CLIPS),
        }
    }

    /// IntersectClipRect (logical units).
    pub fn clip_rect(&mut self, r: Rect) {
        let mut bp = r.abs().to_path(0.01);
        bp.apply_affine(self.xf());
        self.push_clip(bp, FillRule::NonZero, false);
    }

    /// A clip region of `rects` in device units (`mode` as [`Self::clip_path`]); none with mode
    /// copy removes the clip.
    pub fn clip_region(&mut self, rects: &[Rect], mode: u32) {
        if rects.is_empty() {
            if mode == 5 {
                self.dc.clips.clear();
            }
            return;
        }
        let mut bp = BezPath::new();
        for r in rects {
            bp.extend(r.abs().to_path(0.01));
        }
        bp.apply_affine(self.dev_to_doc);
        match mode {
            1 => self.push_clip(bp, FillRule::NonZero, false),
            5 => self.push_clip(bp, FillRule::NonZero, true),
            _ => self.warn(EXCLUDED_CLIPS),
        }
    }

    pub fn exclude_clip(&mut self) {
        self.warn(EXCLUDED_CLIPS);
    }

    // ---------- blits, images and text ----------

    /// A raster operation without a source over `dest` (logical): the brush, white or black.
    /// https://learn.microsoft.com/en-us/windows/win32/gdi/ternary-raster-operations
    pub fn pattern_blit(&mut self, dest: Rect, rop: u32) {
        if rop == 0x00AA_0029 {
            return;
        } // D: retain the destination, no source required.
        let brush = match rop {
            PATCOPY => self.dc.brush.clone(),
            WHITENESS => BrushObj::Solid([255; 3]),
            BLACKNESS => BrushObj::Solid([0; 3]),
            _ => return self.skip(),
        };
        let pen = std::mem::replace(&mut self.dc.pen, PenObj::NULL);
        let outer = std::mem::replace(&mut self.dc.brush, brush);
        let mut bp = dest.abs().to_path(0.01);
        bp.apply_affine(self.xf());
        self.draw(bp, true, false);
        self.dc.pen = pen;
        self.dc.brush = outer;
    }

    /// An image whose source rectangle `src` (pixels, from the top-left) is drawn into `dest`
    /// (logical: origin and signed size) with raster operation `rop`, at `opacity`.
    pub fn image(&mut self, img: Result<Rgba, String>, src: Rect, dest: Rect, rop: u32, opacity: f64) {
        if rop != SRCCOPY {
            self.warn(SKIPPED_ROPS);
            return;
        }
        let img = match img {
            Ok(i) => i,
            Err(_) => return self.warn(BAD_IMAGE),
        };
        let img = crop(img, src);
        if img.width == 0 || img.height == 0 {
            return;
        }
        let (w, h) = (f64::from(img.width), f64::from(img.height));
        let xf = self.xf() * Affine::translate((dest.x0, dest.y0)) * Affine::scale_non_uniform(dest.width() / w, dest.height() / h);
        if !xf.as_coeffs().iter().all(|v| v.is_finite()) || xf.determinant().abs() < 1e-12 {
            return;
        }
        let Some(mut pixels) = image::RgbaImage::from_raw(img.width, img.height, img.pixels) else {
            return;
        };
        for pixel in pixels.pixels_mut() {
            let [r, g, b, a] = pixel.0;
            pixel.0 = [r, g, b, (f64::from(a) * opacity.clamp(0., 1.)).round() as u8];
        }
        self.push(Op::Image { pixels: Arc::new(pixels), transform: xf });
    }

    /// Text at `at` (logical; the current position with TA_UPDATECP) in the current font.
    pub fn text(&mut self, at: Point, s: &str) {
        if !self.check_expansion(s.chars().count()) {
            return;
        }
        let s: String = s.chars().filter(|c| *c != '\0').collect();
        if s.trim().is_empty() {
            return;
        }
        let align = self.dc.text_align;
        let at = if align & 1 != 0 { self.pos } else { at };
        let f = self.dc.font.clone();
        let k = self.scale();
        // A negative height is the em; a positive one the cell (the em and the internal leading).
        let em = if f.height < 0.0 { -f.height } else { f.height * 0.85 };
        let size = if em > 0.0 && (em * k).is_finite() { (em * k).clamp(0.5, 5000.0) } else { 12.0 };
        let db = misterdiagram_fonts::FontDb::global();
        let face = db.face(
            if f.face.trim().is_empty() { "Arial" } else { f.face.trim() },
            match (f.weight >= 600, f.italic) {
                (true, true) => "Bold Italic",
                (true, false) => "Bold",
                (false, true) => "Italic",
                _ => "Regular",
            },
        );
        let glyphs = misterdiagram_fonts::shape(&face, &s, &[], |c| c);
        let advance: f64 = glyphs.iter().map(|g| f64::from(g.x_advance)).sum::<f64>() * size / face.upem.max(1.);
        let x = match align & 6 {
            6 => -advance * 0.5,
            2 => -advance,
            _ => 0.,
        };
        let drop = match align & 0x18 {
            0x18 => 0.,
            0x08 => -0.2 * size,
            _ => 0.8 * size,
        };
        let origin = self.xf() * at;
        let angle = -(f.escapement / 10.).to_radians();
        let m = Affine::translate(origin.to_vec2())
            * Affine::rotate(if angle.is_finite() { angle } else { 0. })
            * Affine::translate((x, drop))
            * Affine::scale(size / face.upem.max(1.));
        let mut x = 0.;
        for glyph in glyphs {
            if self.failure.is_some() {
                break;
            }
            let path = m * Affine::translate((x + f64::from(glyph.x_offset), -f64::from(glyph.y_offset))) * db.outline(&face, glyph.gid).as_ref();
            self.push(Op::Path { path, paint: Paint::Solid(rgb(self.dc.text_color)) });
            x += f64::from(glyph.x_advance);
        }
    }

    // ---------- the document ----------

    /// The document: what was drawn on one layer, clipped art in clipping groups.
    fn finish(mut self) -> Imported {
        self.flush();
        if self.skipped > 0 {
            let n = self.skipped;
            self.warnings.push(format!(
                "{n} record{} of kinds MisterDiagram doesn't read {} skipped",
                if n == 1 { "" } else { "s" },
                if n == 1 { "was" } else { "were" }
            ));
        }
        let out = std::mem::take(&mut self.out);
        if self.emf_plus && !out.is_empty() {
            self.warnings.push("EMF+ comment payloads are not rendered; classic EMF fallback retained".into());
        }
        if out.is_empty() && self.emf_plus {
            self.warnings.push("the picture is drawn with EMF+ records, which aren't read yet: nothing was imported".into());
        }
        Imported { ops: out, frame: self.frame, warnings: self.warnings }
    }
}

/// `img` cut to `src` (pixels from the top-left, within the image).
fn crop(img: Rgba, src: Rect) -> Rgba {
    let (w, h) = (img.width, img.height);
    let s = src.abs();
    let clamp = |v: f64, max: u32| if v.is_finite() { v.round().clamp(0.0, f64::from(max)) as u32 } else { 0 };
    let (x0, y0, x1, y1) = (clamp(s.x0, w), clamp(s.y0, h), clamp(s.x1, w), clamp(s.y1, h));
    if (x0, y0, x1, y1) == (0, 0, w, h) || x1 <= x0 || y1 <= y0 {
        return img;
    }
    let mut pixels = Vec::with_capacity(((x1 - x0) * (y1 - y0) * 4) as usize);
    for y in y0..y1 {
        let row = (y * w) as usize * 4;
        if let Some(p) = img.pixels.get(row + x0 as usize * 4..row + x1 as usize * 4) {
            pixels.extend_from_slice(p);
        }
    }
    Rgba { width: x1 - x0, height: y1 - y0, pixels }
}

/// Retargeted EMF record reader. Unknown records are skipped, malformed sizes cannot loop.
/// https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-emf/
pub fn import(bytes: &[u8]) -> Result<Imported, String> {
    if bytes.len() > 128 * 1024 * 1024 {
        return Err("metafile byte limit".into());
    }
    if bytes.get(40..44) != Some(&b" EMF"[..]) {
        return Err("invalid EMF header".into());
    }
    emf::play(bytes)
}

/// Cached OLE presentations may combine a binary mask with XOR/OR colour pixels.
/// The ordinary strict importer retains its established unsupported-operation policy.
/// https://learn.microsoft.com/en-us/windows/win32/gdi/ternary-raster-operations
pub(crate) fn import_cached_picture(bytes: &[u8]) -> Result<Imported, String> {
    if bytes.len() > 128 * 1024 * 1024 {
        return Err("metafile byte limit".into());
    }
    if bytes.get(40..44) != Some(&b" EMF"[..]) {
        return Err("invalid EMF header".into());
    }
    let imported = emf::play_cached_picture(bytes)?;
    if imported.warnings.iter().any(|warning| warning == SKIPPED_ROPS || warning == BAD_IMAGE) {
        return Err("cached EMF has unsupported or damaged bitmap drawing".into());
    }
    if !imported.ops.iter().any(|op| matches!(op, Op::Path { .. } | Op::Stroke { .. } | Op::Image { .. } | Op::Text { .. })) {
        return Err("cached EMF presentation produced no drawing".into());
    }
    Ok(imported)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn anisotropic_window_and_viewport_origins_map_once() {
        let mut p = Player::new(Kind::Emf, Affine::scale(0.75), 25.4 / 96., Rect::new(0., 0., 10., 10.));
        p.set_map_mode(8);
        p.set_window_org(100., -200.);
        p.set_window_ext(600., 300.);
        p.set_viewport_org(20., 30.);
        p.set_viewport_ext(200., 100.);
        assert!((p.xf() * Point::new(100., -200.) - Point::new(15., 22.5)).hypot() < 1e-10);
        assert!((p.xf() * Point::new(700., 100.) - Point::new(165., 97.5)).hypot() < 1e-10);
    }
    #[test]
    fn base_header_description_is_not_optional_micrometre_dimensions() {
        let mut b = vec![0u8; 116];
        for (offset, value) in [
            (0, 1u32),
            (4, 116),
            (24, 0),
            (28, 0),
            (32, 2540),
            (36, 2540),
            (40, 0x464d4520),
            (60, 13),
            (64, 88),
            (72, 1024),
            (76, 768),
            (80, 320),
            (84, 240),
            (100, 7_929_939),
            (104, 7_602_291),
        ] {
            b[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        }
        let imported = import(&b).unwrap();
        assert!((imported.frame.width() - 72.).abs() < 1e-9);
        // Header description does not alter physical device mapping.
        let (_, millimetres_per_pixel, _) = emf::frame(&b).unwrap();
        assert!((millimetres_per_pixel - 320. / 1024.).abs() < 1e-12);
        b.extend_from_slice(&[0; 8]);
        assert!(import(&b).is_ok());
    }
    #[test]
    fn destination_only_raster_operation_is_a_noop() {
        let mut p = Player::new(Kind::Emf, Affine::IDENTITY, 25.4 / 96., Rect::new(0., 0., 1., 1.));
        p.pattern_blit(Rect::new(0., 0., 1., 1.), 0x00AA_0029);
        let result = p.finish();
        assert!(result.ops.is_empty());
        assert!(result.warnings.is_empty());
    }
    #[test]
    fn hostile_records_dib_and_unknown_record_skip() {
        for len in 0..128 {
            assert!(import(&vec![0; len]).is_err());
        }
        assert!(dib::decode(&[], &[], dib::Alpha::Ignore).is_err());
        let mut header = vec![0u8; 88];
        header.get_mut(0..4).unwrap().copy_from_slice(&1u32.to_le_bytes());
        header.get_mut(4..8).unwrap().copy_from_slice(&88u32.to_le_bytes());
        header.get_mut(40..44).unwrap().copy_from_slice(b" EMF");
        let mut unknown = header.clone();
        unknown.extend_from_slice(&0xffffu32.to_le_bytes());
        unknown.extend_from_slice(&8u32.to_le_bytes());
        unknown.extend_from_slice(&14u32.to_le_bytes());
        unknown.extend_from_slice(&8u32.to_le_bytes());
        let imported = import(&unknown).unwrap();
        assert!(imported.warnings.iter().any(|s| s.contains("skipped")));
        for n in [0, 1, 7, u32::MAX] {
            let mut bytes = header.clone();
            bytes.extend_from_slice(&42u32.to_le_bytes());
            bytes.extend_from_slice(&n.to_le_bytes());
            assert!(import(&bytes).is_ok());
        }
        let mut bitmap = vec![0; 40];
        bitmap.get_mut(0..4).unwrap().copy_from_slice(&40u32.to_le_bytes());
        bitmap.get_mut(4..8).unwrap().copy_from_slice(&i32::MAX.to_le_bytes());
        bitmap.get_mut(8..12).unwrap().copy_from_slice(&i32::MAX.to_le_bytes());
        assert!(dib::decode(&bitmap, &[], dib::Alpha::Ignore).is_err());
    }
}
