//! Original layout commands built on public ShapeSheet cells and witnessed glue.
use crate::{Result, Session, edit::*, operation};
use misterdiagram_geom::Point;
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
use serde_json::{Value, json};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Side {
    Left,
    Right,
    Bottom,
    Top,
}
fn number(e: &mut Evaluator<'_>, scope: Scope, name: &str) -> Result<f64> {
    let value = e.evaluate(scope, name).numeric().map_err(operation)?;
    if value.is_finite() && value.abs() <= 1e6 { Ok(value) } else { Err(operation(format!("invalid {name}"))) }
}
fn facing(doc: &misterdiagram_model::Document, page: usize, id: u32, target: u32) -> Result<Side> {
    let page = doc.pages.get(page).ok_or_else(|| operation("page missing"))?;
    let mut e = Evaluator::new(doc).map_err(operation)?;
    let scope = Scope::PageShape { page: page.id, shape: id };
    let other = Scope::PageShape { page: page.id, shape: target };
    for scope in [scope, other] {
        if e.shape(scope).is_none() || e.cell(scope, "BeginX").is_some() {
            return Err(operation("layout endpoints must be two-dimensional shapes"));
        }
    }
    let (w, h) = (number(&mut e, scope, "Width")?, number(&mut e, scope, "Height")?);
    if w <= 0. || h <= 0. {
        return Err(operation("layout endpoint must have positive bounds"));
    }
    let other_center =
        e.shape_to_page(other).map_err(operation)? * Point::new(number(&mut e, other, "Width")? / 2., number(&mut e, other, "Height")? / 2.);
    let transform = e.shape_to_page(scope).map_err(operation)?;
    if transform.determinant().abs() < 1e-12 {
        return Err(operation("singular shape transform"));
    }
    let local = transform.inverse() * other_center;
    let dx = (local.x - w / 2.) / w;
    let dy = (local.y - h / 2.) / h;
    if dx.abs() >= dy.abs() { Ok(if dx >= 0. { Side::Right } else { Side::Left }) } else { Ok(if dy >= 0. { Side::Top } else { Side::Bottom }) }
}
/// Reuse an inbound point at the requested side position, or append the seven
/// canonical cells observed in P5a route/side-glue and connector/static-glue.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/connection-points-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/x-cell-connection-points-section
fn point(doc: &mut misterdiagram_model::Document, page: usize, id: u32, side: Side, fraction: f64) -> Result<String> {
    let page_id = doc.pages.get(page).ok_or_else(|| operation("page missing"))?.id;
    let scope = Scope::PageShape { page: page_id, shape: id };
    let mut e = Evaluator::new(doc).map_err(operation)?;
    let w = number(&mut e, scope, "Width")?;
    let h = number(&mut e, scope, "Height")?;
    let (x, y, xf, yf) = match side {
        Side::Left => (0., h * fraction, "0".to_string(), format!("Height*{fraction}")),
        Side::Right => (w, h * fraction, "Width".to_string(), format!("Height*{fraction}")),
        Side::Bottom => (w * fraction, 0., format!("Width*{fraction}"), "0".to_string()),
        Side::Top => (w * fraction, h, format!("Width*{fraction}"), "Height".to_string()),
    };
    let all = e.resolved_named_cells(scope);
    let points = all
        .iter()
        .filter(|n| n.cell.name == "X" && n.section.is_some_and(|s| s.name == "Connection"))
        .map(|n| {
            let companion = |column: &str| {
                all.iter()
                    .find(|v| {
                        v.cell.name == column
                            && v.section.is_some_and(|s| s.name == "Connection")
                            && v.row.zip(n.row).is_some_and(|(a, b)| a.index == b.index && a.name == b.name)
                    })
                    .map(|v| v.name.clone())
                    .ok_or_else(|| operation("incomplete connection row"))
            };
            Ok((n.name.clone(), companion("Y")?, companion("Type")?))
        })
        .collect::<Result<Vec<_>>>()?;
    for (name, y_name, type_name) in points {
        let kind = e.evaluate(scope, &type_name).numeric().map_err(operation)?;
        if kind != 0. && kind != 2. {
            continue;
        }
        if (number(&mut e, scope, &name)? - x).abs() < 1e-9 && (number(&mut e, scope, &y_name)? - y).abs() < 1e-9 {
            return Ok(name);
        }
    }
    drop(e);
    let coordinate = |name: &str, value: f64, formula: &str| cell(name, &value.to_string(), (formula != "0").then_some(formula));
    misterdiagram_route::add_connection(
        doc,
        page,
        id,
        misterdiagram_route::ConnectionPoint {
            name: None,
            x: coordinate("X", x, &xf),
            y: coordinate("Y", y, &yf),
            direction: [0., 0.],
            kind: 0,
            auto_gen: false,
            prompt: String::new(),
        },
    )
    .map_err(operation)
}
/// Glue both ends of an existing connector to facing side centers. Connection
/// rows and static PAR(PNT(...)) glue match the original route/side-glue witness.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.cell.glueto
pub(crate) fn sides(s: &mut Session, p: &Value) -> Result<Value> {
    let (index, id, _) = shape_scope(s, p)?;
    let source = uint(p, "source")?;
    let target = uint(p, "target")?;
    if source == target || id == source || id == target {
        return Err(operation("connector and endpoint shapes must be distinct"));
    }
    let begin = facing(&s.document, index, source, target)?;
    let end = facing(&s.document, index, target, source)?;
    s.edit_document(|doc, _| {
        let a = point(doc, index, source, begin, 0.5)?;
        let b = point(doc, index, target, end, 0.5)?;
        misterdiagram_route::connect_static(doc, index, id, misterdiagram_route::End::Begin, source, &a).map_err(operation)?;
        misterdiagram_route::connect_static(doc, index, id, misterdiagram_route::End::End, target, &b).map_err(operation)?;
        Ok(json!({"id":id,"begin":{"shape":source,"point":a},"end":{"shape":target,"point":b}}))
    })
}
