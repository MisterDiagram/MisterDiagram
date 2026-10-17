//! Inspect state supplied by the strict write/import round trip.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
use misterdiagram_engine::{EngineError, Session};
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
use serde_json::json;
fn inspect(doc: &misterdiagram_model::Document, label: &str) -> Result<(), Box<dyn std::error::Error>> {
    let page = doc.pages.first().ok_or("no page")?;
    let shape = page.shapes.first().ok_or("no shape")?;
    let mut e = Evaluator::new(doc).map_err(|e| e.to_string())?;
    let scope = Scope::PageShape { page: page.id, shape: shape.id };
    for name in ["Width", "Geometry1.X2", "FillForegnd"] {
        let value = e.evaluate(scope, name).text();
        let c = e.cell(scope, name).ok_or("missing cell")?;
        println!(
            "{}",
            json!({"phase":label,"name":name,"cached":c.value,"formula":c.formula,"evaluated":value,"attributes":format!("{:?}",c.attributes)})
        );
    }
    println!("{}", json!({"phase":label,"colors":doc.colors,"nodes":doc.retained_xml_nodes,"parts":doc.parts.len()}));
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut s = Session::new();
    s.execute("shape.draw.rect", &json!({"x":1,"y":1,"width":2,"height":1}))?;
    inspect(s.document(), "initial")?;
    s.edit_document(|d, _| {
        let shape = d.pages.first_mut().and_then(|p| p.shapes.first_mut()).ok_or_else(|| EngineError::Operation("no shape".into()))?;
        for c in &mut shape.sheet.cells {
            if c.name == "Width" {
                c.value = Some("8".into());
            }
        }
        shape.sheet.cells.push(misterdiagram_model::Cell {
            name: "FillForegnd".into(),
            value: Some("0".into()),
            formula: Some("RGB(12,34,56)".into()),
            ..Default::default()
        });
        inspect(d, "before_write_import").map_err(|e| EngineError::Operation(e.to_string()))
    })?;
    inspect(s.document(), "after_write_import")?;
    Ok(())
}
