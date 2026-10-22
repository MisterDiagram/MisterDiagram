//! Contributor-original generated drawings and numeric M5.39 observations.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/shaperoutestyle-cell-shape-layout-section
use misterdiagram_geom::{Point, Rect};
use misterdiagram_model::{Cell, Document, Sheet, skeleton};
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
fn set(sheet: &mut Sheet, name: &str, value: f64) {
    sheet.cells.retain(|c| c.name != name);
    sheet.cells.push(Cell { name: name.into(), value: Some(value.to_string()), ..Default::default() });
}
fn box_(doc: &mut Document, x: f64, y: f64) -> u32 {
    let id = doc.pages[0].shapes.len() as u32 + 1;
    let mut s = skeleton::rectangle(id, x, y, 0.5, 0.5, "").unwrap();
    for (name, value) in [("ObjType", 1.), ("ShapePermeableX", 0.), ("ShapePermeableY", 0.)] {
        set(&mut s.sheet, name, value);
    }
    doc.pages[0].shapes.push(s);
    id
}
fn setup(style: f64, spacing: f64) -> Document {
    let mut doc = skeleton::document(false);
    for (name, value) in [("RouteStyle", style), ("LineToLineX", spacing), ("LineToLineY", spacing), ("LineJumpCode", 0.)] {
        set(&mut doc.pages[0].sheet, name, value);
    }
    doc
}
fn link(doc: &mut Document, a: u32, b: u32) -> u32 {
    let id = doc.pages[0].shapes.len() as u32 + 1;
    skeleton::glue_dynamic(&mut doc.pages[0], id, 2, [a, b], [[0., 0.], [1., 1.]]).unwrap();
    set(&mut doc.pages[0].shapes.last_mut().unwrap().sheet, "ShapeRouteStyle", 0.);
    id
}
fn route(doc: &mut Document) -> misterdiagram_route::GlueHost {
    let before = doc.clone();
    let mut host = misterdiagram_route::GlueHost::prepare(&before, doc).unwrap();
    misterdiagram_route::reroute(&before, doc, &mut host, Some((0, None))).unwrap();
    host
}
fn points(doc: &Document, id: u32) -> Vec<Point> {
    let mut e = Evaluator::new(doc).unwrap();
    let scope = Scope::PageShape { page: 0, shape: id };
    let transform = e.shape_to_page(scope).unwrap();
    let rows = e
        .resolved_named_cells(scope)
        .into_iter()
        .filter(|n| n.section.is_some_and(|s| s.name == "Geometry" && s.index.unwrap_or(0) == 0))
        .filter_map(|n| n.row.filter(|r| !r.deleted).and_then(|r| r.index))
        .collect::<std::collections::BTreeSet<_>>();
    rows.into_iter()
        .map(|i| {
            transform
                * Point::new(
                    e.evaluate(scope, &format!("Geometry1.X{i}")).numeric().unwrap(),
                    e.evaluate(scope, &format!("Geometry1.Y{i}")).numeric().unwrap(),
                )
        })
        .collect()
}
fn cut(doc: &Document, ids: &[u32], vertical: bool, gap: f64) -> Vec<f64> {
    ids.iter()
        .map(|&id| {
            let p = points(doc, id);
            let lanes = p
                .windows(2)
                .enumerate()
                .filter(|(i, _)| *i > 0 && *i + 2 < p.len())
                .filter_map(|(_, s)| {
                    let (a, b) = if vertical { (Point::new(s[0].y, s[0].x), Point::new(s[1].y, s[1].x)) } else { (s[0], s[1]) };
                    ((a.y - b.y).abs() < 1e-9 && a.y > 1.5 && a.y < 1.5 + gap).then_some(a.y)
                })
                .collect::<Vec<_>>();
            assert_eq!(lanes.len(), 1, "connector{id} points{p:?}");
            lanes[0]
        })
        .collect()
}
fn weave(n: u32, gap: f64, spacing: f64, style: f64, vertical: bool) -> (Document, Vec<u32>) {
    let mut doc = setup(style, spacing);
    for row in 0..2 {
        for i in 0..2 * n {
            let x = 1. + 2. * f64::from(i);
            let y = if row == 0 { 1. } else { 1.5 + gap };
            let (x, y) = if vertical { (y, x) } else { (x, y) };
            box_(&mut doc, x, y);
        }
    }
    let ids = (0..n).map(|i| link(&mut doc, i + 1, 4 * n - i)).collect();
    (doc, ids)
}
#[test]
fn m540a_restricted_witnesses_and_three_holdouts() {
    // Literal observed values, independent of the implementation's recurrence.
    let observed = [1.6875, 1.625, 1.5625, 1.65625, 1.59375, 1.53125, 1.671875, 1.640625, 1.609375, 1.578125];
    let mut cases = Vec::new();
    for vertical in [false, true] {
        for gap in [3., 0.75] {
            for n in [2, 3, 5, 10] {
                cases.push((n, gap, 0.125, 1., vertical));
            }
        }
    }
    for gap in [3., 0.75] {
        cases.push((5, gap, 0.25, 1., false));
        cases.push((3, gap, 0.125, 0., false));
    }
    cases.extend([(4, 3., 0.125, 1., false), (7, 0.75, 0.125, 1., true), (5, 1.25, 0.125, 1., false)]);
    for (n, gap, spacing, style, vertical) in cases {
        let (mut doc, ids) = weave(n, gap, spacing, style, vertical);
        let host = route(&mut doc);
        assert!(host.outcomes().values().all(Option::is_none), "{:?}", host.outcomes());
        let lanes = cut(&doc, &ids, vertical, gap);
        let count = if vertical { n } else { n - 1 };
        for i in 0..count as usize {
            assert!(
                (lanes[i] - observed[i]).abs() <= 1e-6,
                "n{n} gap{gap} spacing{spacing} style{style} vertical{vertical} connector{}: {} != {}",
                ids[i],
                lanes[i],
                observed[i]
            );
        }
        println!(
            "M5.40a n={n} gap={gap} spacing={spacing} style={style} vertical={vertical} lanes={lanes:?} tie_break={:?}",
            (!vertical).then(|| *ids.last().unwrap())
        );
        let before = doc.clone();
        route(&mut doc);
        assert_eq!(doc, before, "lane reroute must be idempotent");
    }
}
fn shared(a: &[Point], b: &[Point]) -> bool {
    a.windows(2).any(|s| {
        b.windows(2).any(|t| {
            ((s[0].y - s[1].y).abs() < 1e-9
                && (t[0].y - t[1].y).abs() < 1e-9
                && (s[0].y - t[0].y).abs() < 1e-6
                && s[0].x.max(s[1].x).min(t[0].x.max(t[1].x)) - s[0].x.min(s[1].x).max(t[0].x.min(t[1].x)) > 1e-6)
                || ((s[0].x - s[1].x).abs() < 1e-9
                    && (t[0].x - t[1].x).abs() < 1e-9
                    && (s[0].x - t[0].x).abs() < 1e-6
                    && s[0].y.max(s[1].y).min(t[0].y.max(t[1].y)) - s[0].y.min(s[1].y).max(t[0].y.min(t[1].y)) > 1e-6)
        })
    })
}
#[test]
fn hundred_boxes_have_ten_distinct_heights_and_no_shared_trunk_or_box_hit() {
    let mut doc = setup(1., 0.125);
    let mut boxes = Vec::new();
    for row in 0..10 {
        for col in 0..10 {
            let x = 1. + 2. * f64::from(col);
            let y = 1. + 3. * f64::from(row);
            box_(&mut doc, x, y);
            boxes.push(Rect::new(x, y, x + 0.5, y + 0.5));
        }
    }
    let ids = (0..10).map(|i| link(&mut doc, i + 1, 100 - i)).collect::<Vec<_>>();
    let host = route(&mut doc);
    assert!(host.outcomes().values().all(Option::is_none), "{:?}", host.outcomes());
    let routes = ids.iter().map(|&id| points(&doc, id)).collect::<Vec<_>>();
    let mut heights = Vec::new();
    for (i, points) in routes.iter().enumerate() {
        for other in routes.iter().skip(i + 1) {
            assert!(!shared(points, other), "{points:?} shares {other:?}");
        }
        let horizontal = points
            .windows(2)
            .filter(|s| (s[0].y - s[1].y).abs() < 1e-9)
            .max_by(|a, b| (a[0].x - a[1].x).abs().total_cmp(&(b[0].x - b[1].x).abs()))
            .unwrap();
        heights.push(horizontal[0].y);
        for s in points.windows(2) {
            for r in &boxes {
                let hit = if (s[0].x - s[1].x).abs() < 1e-9 {
                    s[0].x > r.x0 && s[0].x < r.x1 && s[0].y.min(s[1].y) < r.y1 && s[0].y.max(s[1].y) > r.y0
                } else {
                    s[0].y > r.y0 && s[0].y < r.y1 && s[0].x.min(s[1].x) < r.x1 && s[0].x.max(s[1].x) > r.x0
                };
                assert!(!hit, "{} {s:?} in {r:?}", ids[i]);
            }
        }
    }
    println!("100-box heights={heights:?}");
    heights.sort_by(f64::total_cmp);
    heights.dedup_by(|a, b| (*a - *b).abs() < 1e-6);
    assert_eq!(heights.len(), 10);
}
#[test]
fn other_styles_and_unshared_routes_keep_serialized_geometry() {
    for style in [0., 1.] {
        let (mut doc, ids) = weave(1, 3., 0.125, style, false);
        route(&mut doc);
        assert_eq!(cut(&doc, &ids, false, 3.), vec![4.3125], "unshared route keeps its original clearance");
        let before = doc.clone();
        route(&mut doc);
        assert_eq!(doc, before);
    }
    for style in [2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16., 17., 18., 19., 20., 21., 22.] {
        let (mut doc, ids) = weave(3, 3., 0.125, style, false);
        route(&mut doc);
        if style == 17. {
            assert_eq!(cut(&doc, &ids, false, 3.), vec![3.125; 3], "tree bus must remain at its original midpoint");
        }
        let before = doc.clone();
        route(&mut doc);
        assert_eq!(doc, before, "style{style}");
    }
}

#[test]
fn targeted_reroute_sees_unchanged_fixed_and_other_style_connectors() {
    for fixed in [false, true] {
        let (mut doc, ids) = weave(3, 3., 0.125, 1., false);
        route(&mut doc);
        let other = doc.pages[0].shapes.iter_mut().find(|s| s.id == ids[2]).unwrap();
        set(&mut other.sheet, if fixed { "ConFixedCode" } else { "ShapeRouteStyle" }, if fixed { 2. } else { 17. });
        route(&mut doc);
        let original = doc.clone();
        let mut expected = doc.clone();
        route(&mut expected);
        let mut host = misterdiagram_route::GlueHost::prepare(&original, &mut doc).unwrap();
        misterdiagram_route::reroute(&original, &mut doc, &mut host, Some((0, Some(ids[0])))).unwrap();
        assert_eq!(doc, expected, "fixed={fixed}");
        assert!(host.outcomes().values().all(Option::is_none), "{:?}", host.outcomes());
    }
}
