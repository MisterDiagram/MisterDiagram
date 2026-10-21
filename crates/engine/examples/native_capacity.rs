//! Contributor-original capacity fixtures, generated through the model API.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
use misterdiagram_model::{Cell, skeleton};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    let count = args.get(1).ok_or("shape count")?.parse::<u32>()?;
    let links = args.get(2).ok_or("connector count")?.parse::<u32>()?;
    let path = args.get(3).ok_or("output path")?;
    if count > 20000 || links > 500 || count < links.saturating_mul(2) {
        return Err("fixture size limit".into());
    }
    let grid = args.get(4).is_some_and(|s| s == "grid");
    let dense = args.get(4).is_some_and(|s| s == "dense");
    let mut doc = skeleton::document(false);
    if dense {
        let mut shape = skeleton::rectangle(5, 0., 0., 1., 1., "")?;
        shape.line_style = None;
        shape.fill_style = None;
        shape.text_style = None;
        shape.sheet.cells.push(Cell { name: "ObjType".into(), value: Some("1".into()), ..Default::default() });
        doc.masters.push(misterdiagram_model::Master {
            id: 3,
            name: "Capacity rectangle".into(),
            name_u: "Capacity rectangle".into(),
            unique_id: Some("{9F299701-6A3C-467D-8001-310042000032}".into()),
            base_id: Some("{9F299701-6A3C-467D-8001-310042000033}".into()),
            shapes: vec![shape].into(),
            part: "visio/masters/master2.xml".into(),
            ..Default::default()
        });
    }
    let page = doc.pages.first_mut().ok_or("page")?;
    for id in 1..=count {
        let pair = (id - 1) / 2;
        let x = 1. + f64::from(pair % 50) * 2.;
        let y = 1. + f64::from(pair / 50) * 3. + if id % 2 == 1 { 1.5 } else { 0. };
        let (x, y, w, h) = if grid {
            match id {
                1 => (0., 0., 1., 1.),
                2 => (1000., 0., 1., 1.),
                _ => (f64::from(id) * 4., -1. - f64::from(id) * 0.001, 1., 3. + f64::from(id) * 0.002),
            }
        } else if dense {
            match id {
                1 => (0., 0., 1., 1.),
                2 => (10., 0., 1., 1.),
                _ => (4., 0., 1., 1.),
            }
        } else {
            (x, y, 0.5, 0.5)
        };
        let mut shape = skeleton::rectangle(id, x, y, w, h, "")?;
        shape.sheet.cells.push(Cell { name: "ObjType".into(), value: Some("1".into()), ..Default::default() });
        if dense {
            shape.master = Some(3);
            shape.line_style = None;
            shape.fill_style = None;
            shape.text_style = None;
            shape.sheet.cells.retain(|c| matches!(c.name.as_str(), "PinX" | "PinY"));
            shape.sheet.sections.clear();
        }
        page.shapes.push(shape);
    }
    for i in 0..links {
        let x = 1.25 + f64::from(i % 50) * 2.;
        let y = 1. + f64::from(i / 50) * 3.;
        let (targets, ends) = if grid {
            ([1, 2], [[1., 0.5], [1000., 0.5]])
        } else if dense {
            ([1, 2], [[1., 0.5], [10., 0.5]])
        } else {
            ([i * 2 + 1, i * 2 + 2], [[x, y + 1.5], [x, y + 0.5]])
        };
        skeleton::glue_dynamic(page, count + i + 1, 2, targets, ends)?;
    }
    let written = misterdiagram_vsdx::write(&doc)?;
    std::fs::write(path, written.bytes)?;
    println!("placeable={count} connectors={links}");
    Ok(())
}
