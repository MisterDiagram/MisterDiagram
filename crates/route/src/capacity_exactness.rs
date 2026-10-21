//! Large contributor-original pages; full-set routing exists only in this test.
use super::*;
use misterdiagram_model::{Cell, Document, skeleton};
fn set(shape: &mut misterdiagram_model::Shape, name: &str, value: f64) {
    shape.sheet.cells.retain(|c| c.name != name);
    shape.sheet.cells.push(Cell { name: name.into(), value: Some(value.to_string()), ..Default::default() });
}
fn generated(count: u32, layout: &str) -> Document {
    let mut doc = skeleton::document(false);
    let page = &mut doc.pages[0];
    for id in 1..=count {
        let i = id - 1;
        let (x, y, w, h) = match layout {
            "grid" => (f64::from(i % 20) * 3., f64::from(i / 20) * 3., 0.5, 0.5),
            "rows-with-gaps" => (f64::from(i % 30) * 2. + f64::from(i % 30 / 5) * 2., f64::from(i / 30) * 4., 1., 1.),
            "nested-clusters" => (f64::from(i % 10) * 1.5 + f64::from(i / 100) * 30., f64::from(i / 10 % 10) * 1.5, 0.5, 0.5),
            "far-barrier" => match id {
                1 => (0., 0., 1., 1.),
                2 => (8., 0., 1., 1.),
                3 => (4., -50., 0.5, 70.),
                4 => (4., 22., 0.5, 58.),
                5 => (0., 10., 4.2, 10.4),
                _ => (100. + f64::from(i % 25) * 3., 100. + f64::from(i / 25) * 3., 0.5, 0.5),
            },
            _ => (0., 0., 0.5, 0.5),
        };
        let mut shape = skeleton::rectangle(id, x, y, w, h, "").unwrap();
        set(&mut shape, "ObjType", 1.);
        page.shapes.push(shape);
    }
    for i in 0..50 {
        let targets = if layout == "far-barrier" { [1, 2] } else { [i + 1, count - i] };
        skeleton::glue_dynamic(page, count + i + 1, 2, targets, [[0., 0.], [1., 1.]]).unwrap();
    }
    doc
}
fn routed(source: &Document, full: bool) -> (Document, usize) {
    FULL_OBSTACLES.set(full);
    CORRIDOR_GROWTHS.set(0);
    let mut doc = source.clone();
    let mut host = crate::GlueHost::prepare(source, &mut doc).unwrap();
    assert_eq!(crate::reroute(source, &mut doc, &mut host, Some((0, None))).unwrap(), 50);
    FULL_OBSTACLES.set(false);
    assert!(host.outcomes().values().all(Option::is_none), "capacity fallback is not an exact route: {:?}", host.outcomes());
    (doc, CORRIDOR_GROWTHS.get())
}
#[test]
fn pruned_routes_equal_full_obstacles_on_large_generated_pages() {
    for count in [300, 600, 1000] {
        for layout in ["far-barrier", "grid", "rows-with-gaps", "nested-clusters"] {
            println!("checking exactness {layout}-{count}");
            let source = generated(count, layout);
            let (pruned, growths) = routed(&source, false);
            let (full, _) = routed(&source, true);
            let mut a = Evaluator::new(&pruned).unwrap();
            let mut b = Evaluator::new(&full).unwrap();
            for id in count + 1..=count + 50 {
                let scope = Scope::PageShape { page: 0, shape: id };
                let names = a
                    .resolved_named_cells(scope)
                    .into_iter()
                    .chain(b.resolved_named_cells(scope))
                    .map(|n| n.name)
                    .filter(|n| n.starts_with("Geometry") || matches!(n.as_str(), "BeginX" | "BeginY" | "EndX" | "EndY"))
                    .collect::<std::collections::BTreeSet<_>>();
                for name in names {
                    let av = a.evaluate(scope, &name);
                    let bv = b.evaluate(scope, &name);
                    match (av.numeric(), bv.numeric()) {
                        (Ok(x), Ok(y)) => assert!((x - y).abs() <= 1e-9, "{count} {layout} connector {id} {name}: {x} != {y}"),
                        _ => assert_eq!(av, bv, "{count} {layout} connector {id} {name}"),
                    }
                }
            }
            if layout == "far-barrier" {
                assert!(growths > 0, "far barrier must require corridor growth");
            }
            println!("exactness page={layout}-{count} placeable={count} connectors=50 equal=50 corridor_growths={growths}");
        }
    }
}

#[test]
fn far_barrier_obstacle_outside_initial_corridor_changes_route() {
    let start = Point::new(1., 0.5);
    let end = Point::new(8., 0.5);
    let barrier = Rect::new(4., -50., 4.5, 20.);
    let top = Rect::new(4., 22., 4.5, 80.);
    let outside = Rect::new(0., 10., 4.2, 20.4);
    let corridor = Rect::new(0., 0., 9., 1.).inflate(0.375, 0.375);
    assert!(!intersects(corridor, outside));
    let first = path(start, end, &clearance(&[barrier], (0.1875, 0.1875), [start, end]), Some(0), Some(0)).unwrap();
    let full = path(start, end, &clearance(&[barrier, top, outside], (0.1875, 0.1875), [start, end]), Some(0), Some(0)).unwrap();
    assert_ne!(first, full, "outside obstacle must change the actual route, not only the work count");
}

#[test]
fn obstacle_memo_invalidates_after_formula_assignments() {
    let mut doc = generated(300, "grid");
    doc.pages[0].shapes[0].sheet.cells.push(Cell { name: "User.Assign".into(), formula: Some("SETF(GETREF(Width),3)".into()), ..Default::default() });
    let scope = Scope::PageShape { page: 0, shape: 1 };
    let mut e = Evaluator::new(&doc).unwrap();
    let mut memo = ObstacleBounds::default();
    assert_eq!(memo.get(&mut e, scope).unwrap().unwrap().width(), 0.5);
    e.evaluate(scope, "User.Assign");
    assert_eq!(memo.get(&mut e, scope).unwrap().unwrap().width(), 3.);
}
