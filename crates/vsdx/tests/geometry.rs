//! Geometry witnesses stay in the private corpus, never copied into this repository.
use misterdiagram_geom::{GeometryFlags, GeometryRow, Rect, RowKind};
use misterdiagram_model::Shape;
use std::{collections::BTreeSet, path::Path};

#[test]
fn every_geometry_row_has_a_real_corpus_witness() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus/ms-shipped/1033");
    if !Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus").exists() {
        eprintln!("local corpus absent: geometry witnesses skipped");
        return;
    }
    let mut found = BTreeSet::new();
    for file in ["MATRIXDOT_U.vssx", "BLDCOR_M.vssx", "AZUREMENU2_U.vssx", "IDEF0_M.vssx", "HIERARCHICALORGCHART_U.vstx"] {
        let document = misterdiagram_vsdx::import(&std::fs::read(root.join(file)).unwrap()).unwrap();
        let mut shapes: Vec<&Shape> = document.masters.iter().flat_map(|m| &m.shapes).chain(document.pages.iter().flat_map(|p| &p.shapes)).collect();
        while let Some(shape) = shapes.pop() {
            shapes.extend(&shape.shapes);
            let num = |name: &str| {
                shape.sheet.cells.iter().find(|c| c.name == name).and_then(|c| c.value.as_deref()).and_then(|v| v.parse::<f64>().ok()).unwrap_or(1.)
            };
            for section in &shape.sheet.sections {
                if section.name != "Geometry" {
                    continue;
                }
                let mut rows = Vec::new();
                let mut kinds = Vec::new();
                let mut complete = true;
                for r in &section.rows {
                    if r.deleted {
                        continue;
                    }
                    let Some(kind) = r.row_type.as_ref().and_then(|k| k.parse::<RowKind>().ok()) else {
                        complete = false;
                        break;
                    };
                    let value = |name: &str| r.cells.iter().find(|c| c.name == name).and_then(|c| c.value.as_deref()).unwrap_or("0");
                    let number = |name: &str| value(name).parse::<f64>().unwrap_or(0.);
                    let data = match kind {
                        RowKind::PolylineTo => value("A").into(),
                        RowKind::NURBSTo => value("E").into(),
                        _ => String::new(),
                    };
                    rows.push(GeometryRow {
                        kind,
                        x: number("X"),
                        y: number("Y"),
                        a: if kind == RowKind::PolylineTo { 0. } else { number("A") },
                        b: number("B"),
                        c: number("C"),
                        d: number("D"),
                        data,
                    });
                    kinds.push(format!("{kind:?}"));
                }
                if !complete || rows.is_empty() {
                    continue;
                }
                // Only complete, valid local sections constitute a witness. Inherited
                // partial sections are covered by the separate evaluator test below.
                if let Ok(path) = misterdiagram_geom::geometry(
                    &rows,
                    num("Width"),
                    num("Height"),
                    Rect::new(-1000., -1000., 1000., 1000.),
                    GeometryFlags::default(),
                ) {
                    assert!(!path.path.elements().is_empty(), "{file} shape{}", shape.id);
                    found.extend(kinds);
                }
            }
        }
    }
    for kind in [
        "MoveTo",
        "LineTo",
        "ArcTo",
        "Ellipse",
        "EllipticalArcTo",
        "InfiniteLine",
        "PolylineTo",
        "NURBSTo",
        "SplineStart",
        "SplineKnot",
        "RelMoveTo",
        "RelLineTo",
        "RelCubBezTo",
        "RelQuadBezTo",
        "RelEllipticalArcTo",
    ] {
        assert!(found.contains(kind), "missing corpus witness {kind}; found {found:?}");
    }
}
#[test]
fn evaluated_master_and_group_paths_use_inherited_rows() {
    use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus/generated");
    if !Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus").exists() {
        eprintln!("local corpus absent: geometry witnesses skipped");
        return;
    }
    let d = misterdiagram_vsdx::import(&std::fs::read(root.join("master/basic-rect.vsdx")).unwrap()).unwrap();
    let mut e = Evaluator::new(&d).unwrap();
    let scope = Scope::PageShape { page: 0, shape: 1 };
    let paths = e.geometry_paths(scope, Rect::new(0., 0., 100., 100.)).unwrap();
    assert!(!paths.is_empty());
    assert!(paths.iter().any(|p| p.filled()));
    assert!(e.shape_to_page(scope).unwrap().as_coeffs().iter().all(|n| n.is_finite()));
}
