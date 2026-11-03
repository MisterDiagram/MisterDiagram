//! Shared evaluated shape trees. Projection preserves source paint order.
use super::*;
use misterdiagram_geom::GeometryPath;
use misterdiagram_model::text::{Bullet, ParaProps, RunKind, RunProps, TextBody};

/// A resolved Connection row in source row order, in y-up page inches.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/connection-points-section
#[derive(Clone, Debug)]
pub struct ConnectionSite {
    pub name: String,
    pub position: Point,
    pub normal: misterdiagram_geom::Vec2,
}
/// Unexpanded text rectangle in layout points and its y-up page transform.
/// The raster layout may overflow this rectangle; exporters keep the saved box.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/txtwidth-cell-text-transform-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/txtheight-cell-text-transform-section
#[derive(Clone, Copy, Debug)]
pub struct EvaluatedTextBlock {
    pub rect: Rect,
    pub to_page: Affine,
    pub layout_rect: Rect,
}
/// The renderer and exporters consume this form without evaluating cells again.
/// Geometry sections retain flags and baked group transforms. TextLayout and
/// decoded foreign pixels are retained in drawing.ops beside their transforms.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
#[derive(Clone, Debug)]
pub struct EvaluatedShape {
    pub scope: Scope,
    pub id: u32,
    pub master: Option<u32>,
    pub master_shape: Option<u32>,
    pub group: bool,
    pub children_first: bool,
    pub hidden_own: bool,
    pub sections: Vec<GeometryPath>,
    pub paint: Option<Paint>,
    pub line: Option<Line>,
    pub stroke: Option<kurbo::Stroke>,
    pub nominal_weight: f64,
    pub bounds: Rect,
    pub transform: Affine,
    pub drawing_scale: f64,
    pub one_d: bool,
    pub connections: Vec<ConnectionSite>,
    pub text_body: Option<TextBody>,
    pub text_block: Option<EvaluatedTextBlock>,
    pub has_shadow: bool,
    /// Foreign content and text follow native geometry in the own-shape projection.
    pub content_start: usize,
    pub foreign_end: usize,
    pub drawing: Scene,
    pub children: Vec<EvaluatedShape>,
}
/// A BeginX/EndX Connect row, qualified by its source page, in table order.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/connect-element-connects_type-complextypevisio-xml
#[derive(Clone, Debug)]
pub struct EvaluatedConnect {
    pub from: Scope,
    pub to: Scope,
    pub end: bool,
    pub to_cell: String,
}
#[derive(Clone, Debug)]
pub struct EvaluatedPage {
    pub scene: Scene,
    pub shapes: Vec<EvaluatedShape>,
    pub connects: Vec<EvaluatedConnect>,
}
impl EvaluatedShape {
    pub(super) fn resolve_connections(&mut self, e: &mut Evaluator<'_>, budget: &mut SceneBudget, depth: usize) -> Result<(), String> {
        if depth > 128 {
            return Err("connection shape depth limit".into());
        }
        if self.hidden_own {
            self.transform *= e.shape_to_page(self.scope).map_err(|e| e.to_string())?;
        }
        self.connections = connection_sites(e, self.scope, self.transform, budget)?;
        for child in &mut self.children {
            child.resolve_connections(e, budget, depth + 1)?;
        }
        Ok(())
    }

    pub(super) fn hidden(scope: Scope, shape: &Shape, scale: Affine, budget: &mut SceneBudget) -> Result<Self, String> {
        budget.charge(std::mem::size_of::<Self>() * 2)?;
        Ok(Self {
            scope,
            id: shape.id,
            master: shape.master,
            master_shape: shape.master_shape,
            group: true,
            children_first: true,
            hidden_own: true,
            sections: Vec::new(),
            paint: None,
            line: None,
            stroke: None,
            nominal_weight: 0.,
            bounds: Rect::ZERO,
            transform: scale,
            drawing_scale: scale.determinant().abs().sqrt(),
            one_d: false,
            connections: Vec::new(),
            text_body: None,
            text_block: None,
            has_shadow: false,
            content_start: 0,
            foreign_end: 0,
            drawing: Scene {
                fit_empty: false,
                warnings: Vec::new(),
                ops: Vec::new(),
                pdf_strokes: BTreeMap::new(),
                bounds: Rect::ZERO,
                page_size: (1., 1.),
            },
            children: Vec::new(),
        })
    }
    pub(super) fn append_to(self, scene: &mut Scene) -> Result<(), String> {
        let mut children = self.children;
        if self.children_first {
            for child in children.drain(..) {
                child.append_to(scene)?;
            }
        }
        let offset = scene.ops.len();
        if offset.checked_add(self.drawing.ops.len()).is_none_or(|n| n > 200_000) {
            return Err("render operation limit".into());
        }
        for (index, pen) in self.drawing.pdf_strokes {
            scene.pdf_strokes.insert(index.checked_add(offset).ok_or("PDF operation index limit")?, pen);
        }
        scene.ops.extend(self.drawing.ops);
        scene.warnings.extend(self.drawing.warnings);
        for child in children {
            child.append_to(scene)?;
        }
        Ok(())
    }
    pub(super) fn charge_metadata(&self, budget: &mut SceneBudget) -> Result<(), String> {
        budget.charge(std::mem::size_of::<Self>() * 2)?;
        if let Some(stroke) = &self.stroke {
            budget.charge(stroke.dash_pattern.capacity().saturating_mul(8))?;
        }
        if let Some(Paint::Pattern { gradient: Some(g), .. }) = &self.paint {
            budget.charge(g.stops.capacity().saturating_mul(std::mem::size_of::<(f64, Rgba)>()))?;
        }
        if let Some(body) = &self.text_body {
            charge_text(body, budget)?;
        }
        Ok(())
    }
    pub(super) fn charge_copy(&self, budget: &mut SceneBudget) -> Result<(), String> {
        self.charge_metadata(budget)?;
        budget.charge(self.drawing.warnings.capacity().saturating_mul(std::mem::size_of::<String>()))?;
        for warning in &self.drawing.warnings {
            budget.charge(warning.capacity())?;
        }
        budget.charge_ops(&self.drawing.ops)?;
        for pen in self.drawing.pdf_strokes.values() {
            budget.charge(
                std::mem::size_of::<(usize, PdfStroke)>() * 4
                    + std::mem::size_of_val(pen.path.elements()) * 2
                    + pen.stroke.dash_pattern.capacity() * 8,
            )?;
        }
        for section in &self.sections {
            budget.charge(std::mem::size_of::<GeometryPath>() * 2 + std::mem::size_of_val(section.path.elements()) * 2)?;
        }
        for site in &self.connections {
            budget.charge(std::mem::size_of::<ConnectionSite>() * 2 + site.name.capacity())?;
        }
        for child in &self.children {
            child.charge_copy(budget)?;
        }
        Ok(())
    }
}
/// Evaluate one foreground or explicitly selected page using the same traversal
/// and paint projection as scene::page. All retained copies share the budget.
/// A pixel-only canvas refusal is deferred to the consumer: vector exports can
/// uniformly scale an oversized page before allocating their bounded raster.
pub fn page(document: &Document, index: usize, options: Options, budget: &mut SceneBudget) -> Result<EvaluatedPage, String> {
    let themes = misterdiagram_theme::Themes::from_document(document).map_err(|e| e.to_string())?;
    let host = misterdiagram_text::Host;
    let evaluator = Evaluator::new(document).map_err(|e| e.to_string())?.with_theme_host(&themes).with_text_host(&host);
    let mut evaluator = if options.cached_fallback { evaluator.with_cached_fallback() } else { evaluator };
    super::page_evaluated_prepared(&mut evaluator, index, options, budget, true)
}
pub fn foreground_pages(document: &Document, options: Options, budget: &mut SceneBudget) -> Result<Vec<EvaluatedPage>, String> {
    let mut pages = Vec::new();
    for (index, source) in document.pages.iter().enumerate() {
        if source.attributes.get("Background").is_some_and(|s| s == "1" || s == "true") {
            continue;
        }
        if pages.len() >= 4096 {
            return Err("export page limit".into());
        }
        pages.push(page(document, index, options, budget)?);
    }
    Ok(pages)
}
pub(super) fn connection_sites(
    e: &mut Evaluator<'_>,
    scope: Scope,
    transform: Affine,
    budget: &mut SceneBudget,
) -> Result<Vec<ConnectionSite>, String> {
    let mut sites = Vec::new();
    let mut rows: Vec<_> = e
        .resolved_named_cells(scope)
        .into_iter()
        .filter(|named| named.cell.name == "X" && named.section.is_some_and(|s| s.name == "Connection"))
        .collect();
    rows.sort_by_key(|named| {
        named.row.and_then(|row| row.index).map(u64::from).unwrap_or_else(|| {
            named
                .section
                .zip(named.row)
                .and_then(|(section, row)| section.rows.iter().position(|r| std::ptr::eq(r, row)))
                .map_or(u64::MAX, |i| i as u64)
        })
    });
    for named in rows {
        if sites.len() >= 16384 {
            return Err("connection point count limit".into());
        }
        let row = named.row.ok_or("connection row missing")?;
        let section = named.section.ok_or("connection section missing")?;
        let name = named.name.clone();
        let mut values = Vec::new();
        for column in ["X", "Y", "DirX", "DirY"] {
            let cell = misterdiagram_model::Cell { name: column.into(), ..Default::default() };
            let address = misterdiagram_vsdx::evaluate::cell_name(section, row, 0, &cell).map_err(|e| e.to_string())?;
            values.push(e.evaluate_snapshot(scope, &address).numeric().unwrap_or(0.));
        }
        let value = |i| values.get(i).copied().unwrap_or(0.);
        let position = transform * Point::new(value(0), value(1));
        let normal = transform * Point::new(value(2), value(3)) - transform * Point::ORIGIN;
        budget.charge(std::mem::size_of::<ConnectionSite>() * 2 + name.capacity())?;
        sites.push(ConnectionSite { name, position, normal });
    }
    Ok(sites)
}
fn charge_text(body: &TextBody, budget: &mut SceneBudget) -> Result<(), String> {
    let string = |s: &Option<String>| s.as_ref().map_or(0, String::capacity);
    let run = |r: &RunProps| -> usize {
        [&r.font, &r.font_ea, &r.font_cs, &r.font_sym, &r.underline, &r.link, &r.lang].into_iter().fold(0usize, |n, s| n.saturating_add(string(s)))
    };
    let para = |p: &ParaProps| -> usize {
        let mut n = string(&p.bullet_font).saturating_add(string(&p.font_align));
        if let Some(b) = &p.bullet {
            n = n.saturating_add(match b {
                Bullet::None => 0,
                Bullet::Char { char: s } | Bullet::AutoNum { scheme: s, .. } | Bullet::Picture { media: s } => s.capacity(),
            });
        }
        if let Some(tabs) = &p.tabs {
            n = n.saturating_add(tabs.capacity().saturating_mul(std::mem::size_of::<misterdiagram_model::text::TabStop>()));
            for tab in tabs {
                n = n.saturating_add(tab.align.capacity());
            }
        }
        n
    };
    budget.charge(string(&body.body.warp))?;
    budget.charge(body.paragraphs.capacity().saturating_mul(std::mem::size_of::<misterdiagram_model::text::Paragraph>()))?;
    for p in &body.paragraphs {
        budget.charge(para(&p.props))?;
        budget.charge(run(&p.end_props))?;
        budget.charge(p.runs.capacity().saturating_mul(std::mem::size_of::<misterdiagram_model::text::Run>()))?;
        for r in &p.runs {
            budget.charge(r.text.capacity().saturating_add(run(&r.props)))?;
            match &r.kind {
                RunKind::Field { field } => budget.charge(field.capacity())?,
                RunKind::Math { omml } => budget.charge(omml.capacity())?,
                _ => {}
            }
        }
    }
    budget.charge(body.list_style.levels.capacity().saturating_mul(std::mem::size_of::<Option<misterdiagram_model::text::LevelStyle>>()))?;
    for level in body.list_style.levels.iter().flatten() {
        budget.charge(para(&level.para).saturating_add(run(&level.run)))?;
    }
    Ok(())
}
