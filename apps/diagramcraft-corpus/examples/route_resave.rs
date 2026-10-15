//! Verify all SAN connector Geometry and XForm values after Visio saves the file.
//! https://learn.microsoft.com/en-us/office/vba/api/visio.document.saveas
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
use diagramcraft_vsdx::evaluate::{Evaluator, Scope};
use std::collections::{BTreeMap, BTreeSet};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    let before = diagramcraft_vsdx::read(root.join("_state/m4b/accept-inputs/p4b__san-topology.vsdx"))?;
    let after = diagramcraft_vsdx::read(root.join("_vm/out/accept/p4b__san-topology/saved.vsdx"))?;
    let page = before.pages.first().ok_or("missing SAN page")?;
    let ids = page.connects.iter().map(|c| c.from_sheet).collect::<BTreeSet<_>>();
    if ids.len() != 40 || page.connects.len() != 80 {
        return Err("SAN glue count".into());
    }
    let mut a = Evaluator::new(&before).map_err(|e| e.to_string())?;
    let mut b = Evaluator::new(&after).map_err(|e| e.to_string())?;
    let mut diff = String::new();
    let mut cells = 0;
    for id in &ids {
        let scope = Scope::PageShape { page: page.id, shape: *id };
        let kinds = |e: &Evaluator<'_>| {
            e.resolved_named_cells(scope)
                .into_iter()
                .filter(|n| n.section.is_some_and(|s| s.name == "Geometry") && n.cell.name == "X")
                .filter_map(|n| {
                    n.row.filter(|r| !r.deleted).map(|r| ((n.section.and_then(|s| s.index).unwrap_or(0), r.index.unwrap_or(0)), r.row_type.clone()))
                })
                .collect::<BTreeMap<_, _>>()
        };
        if kinds(&a) != kinds(&b) {
            diff.push_str(&format!("{id}\tGeometry row kinds differ\n"));
        }
        let names = a
            .resolved_named_cells(scope)
            .into_iter()
            .chain(b.resolved_named_cells(scope))
            .map(|n| n.name)
            .filter(|n| {
                n.starts_with("Geometry")
                    || matches!(
                        n.as_str(),
                        "BeginX"
                            | "BeginY"
                            | "EndX"
                            | "EndY"
                            | "PinX"
                            | "PinY"
                            | "Width"
                            | "Height"
                            | "LocPinX"
                            | "LocPinY"
                            | "Angle"
                            | "FlipX"
                            | "FlipY"
                    )
            })
            .collect::<BTreeSet<_>>();
        for name in names {
            let x = a.evaluate(scope, &name);
            let y = b.evaluate(scope, &name);
            cells += 1;
            let same = match (x.numeric(), y.numeric()) {
                (Ok(x), Ok(y)) => x.is_finite() && y.is_finite() && (x - y).abs() <= 1e-6,
                _ => x == y,
            };
            if !same {
                diff.push_str(&format!("{id}\t{name}\t{}\t{}\n", x.text(), y.text()));
            }
        }
    }
    std::fs::write(root.join("_state/m4b/san-resave.diff"), &diff)?;
    println!("SAN re-save: {} connectors, {cells} cells, {} differences", ids.len(), diff.lines().count());
    if !diff.is_empty() {
        return Err("Visio changed SAN routes".into());
    }
    Ok(())
}
