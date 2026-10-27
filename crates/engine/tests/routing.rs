//! Transactional route invalidation from obstacle and page layout edits.
use misterdiagram_engine::Session;
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
use serde_json::json;
use std::path::Path;
fn compare(session: &Session, expected: &Path, expected_id: u32) {
    let expected_path = expected;
    let oracle = misterdiagram_vsdx::read(expected).unwrap();
    let mut a = Evaluator::new(session.document()).unwrap();
    let mut b = Evaluator::new(&oracle).unwrap();
    let actual = Scope::PageShape { page: session.document().pages[0].id, shape: 4 };
    let expected = Scope::PageShape { page: oracle.pages[0].id, shape: expected_id };
    let names = b
        .resolved_named_cells(expected)
        .into_iter()
        .chain(a.resolved_named_cells(actual))
        .map(|n| n.name)
        .filter(|n| {
            n.starts_with("Geometry")
                || matches!(
                    n.as_str(),
                    "BeginX" | "BeginY" | "EndX" | "EndY" | "PinX" | "PinY" | "Width" | "Height" | "LocPinX" | "LocPinY" | "Angle"
                )
        })
        .collect::<std::collections::BTreeSet<_>>();
    for name in names {
        let av = a.evaluate(actual, &name);
        let bv = b.evaluate(expected, &name);
        assert!(
            match (av.numeric(), bv.numeric()) {
                (Ok(a), Ok(b)) => (a - b).abs() <= 1e-6,
                _ => av == bv,
            },
            "{} {name}: {av:?} != {bv:?}",
            expected_path.display()
        );
    }
}
#[test]
fn explicit_reroute_after_obstacle_out_and_back_restores_the_observed_routes() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus/generated/connector");
    if !root.exists() {
        eprintln!("skip: private corpus absent");
        return;
    }
    let mut s = Session::new();
    s.execute("file.open", &json!({"path":root.join("obstacle.vsdx")})).unwrap();
    s.execute("shape.move", &json!({"id":3,"x":3.5,"y":2.})).unwrap();
    s.execute("route.reroute", &json!({})).unwrap();
    compare(&s, &root.join("dynamic-glue.vsdx"), 3);
    s.execute("shape.move", &json!({"id":3,"x":3.5,"y":6.5})).unwrap();
    s.execute("route.reroute", &json!({})).unwrap();
    compare(&s, &root.join("obstacle.vsdx"), 4);
}
#[test]
fn page_layout_and_explicit_permeability_reroute() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus/generated/connector");
    if !root.exists() {
        eprintln!("skip: private corpus absent");
        return;
    }
    let mut s = Session::new();
    s.execute("file.open", &json!({"path":root.join("obstacle.vsdx")})).unwrap();
    s.execute("sheet.set", &json!({"scope":"page","name":"RouteStyle","value":2})).unwrap();
    compare(&s, &root.join("dynamic-glue.vsdx"), 3);
    s.execute("sheet.set", &json!({"scope":"page","name":"RouteStyle","value":1})).unwrap();
    compare(&s, &root.join("obstacle.vsdx"), 4);
    for name in ["ShapePermeableX", "ShapePermeableY"] {
        s.execute("sheet.set", &json!({"id":3,"name":name,"formula":"TRUE"})).unwrap();
    }
    // Shape-local permeability no longer requests a page-wide automatic reroute.
    s.execute("route.reroute", &json!({})).unwrap();
    compare(&s, &root.join("dynamic-glue.vsdx"), 3);
}
