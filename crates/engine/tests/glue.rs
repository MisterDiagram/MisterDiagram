//! Contributor-original glue scripts, compared with private Visio observations.
use diagramcraft_engine::Session;
use diagramcraft_vsdx::evaluate::{Evaluator, Scope};
use serde_json::{Value, json};
fn run(s: &mut Session, id: &str, p: Value) -> Value {
    s.execute(id, &p).unwrap_or_else(|e| panic!("{id} {p}: {e}"))
}
fn number(s: &Session, id: u32, name: &str) -> f64 {
    Evaluator::new(s.document()).unwrap().evaluate(Scope::PageShape { page: s.document().pages[0].id, shape: id }, name).numeric().unwrap()
}
#[test]
fn static_glue_moves_with_target_and_records_connection_parts() {
    let mut s = Session::new();
    for x in [1., 5.] {
        run(&mut s, "shape.draw.rect", json!({"x":x,"y":6,"width":1.5,"height":1.5}));
    }
    for id in [1, 2] {
        assert_eq!(run(&mut s, "shape.connection.add", json!({"id":id,"x":"Width*0.5","y":"Height*0.5"}))["cell"], "Connections.X1");
    }
    run(&mut s, "master.drop", json!({"master":2,"x":4,"y":6.75}));
    for (end, target) in [("begin", 1), ("end", 2)] {
        run(&mut s, "connect.static", json!({"id":3,"end":end,"target":target,"point":"Connections.X1"}));
    }
    assert_eq!(number(&s, 3, "BeginX"), 1.75);
    assert_eq!(number(&s, 3, "BeginY"), 6.75);
    assert_eq!(number(&s, 3, "EndX"), 5.75);
    assert_eq!(number(&s, 3, "EndY"), 6.75);
    assert_eq!(s.document().pages[0].connects.len(), 2);
    let begin = &s.document().pages[0].connects[0];
    assert_eq!(begin.from_part, Some(9));
    assert_eq!(begin.to_part, Some(100));
    run(&mut s, "shape.move", json!({"id":1,"x":2.75,"y":4.75}));
    assert_eq!(number(&s, 3, "BeginX"), 2.75);
    assert_eq!(number(&s, 3, "BeginY"), 4.75);
    run(&mut s, "edit.undo", json!({}));
    assert_eq!(number(&s, 3, "BeginX"), 1.75);
    assert!(s.execute("connect.static", &json!({"id":1,"target":2,"end":"begin","point":"Connections.X1"})).is_err());
    let before = s.document().clone();
    assert!(s.execute("connect.static", &json!({"id":3,"target":2,"end":"begin","point":"Connections.X99"})).is_err());
    assert_eq!(s.document(), &before);
}
#[test]
fn named_points_and_hostile_connection_params_are_bounded() {
    let mut s = Session::new();
    run(&mut s, "shape.draw.rect", json!({"x":0,"y":0,"width":2,"height":2}));
    run(&mut s, "shape.connection.add", json!({"id":1,"name":"Port","x":"Width","y":"Height/2","dir_x":1,"type":2,"prompt":"port"}));
    run(&mut s, "shape.draw.line", json!({"x1":3,"y1":1,"x2":5,"y2":1}));
    run(&mut s, "connect.static", json!({"id":2,"end":"begin","target":1,"point":"Connections.Port.X"}));
    assert_eq!(number(&s, 2, "BeginX"), 2.);
    assert_eq!(number(&s, 2, "BeginY"), 1.);
    for p in [
        json!({"id":1,"x":"(","y":0}),
        json!({"id":1,"x":0,"y":0,"name":"bad!name"}),
        json!({"id":1,"x":1e99,"y":0}),
        json!({"id":1,"x":0,"y":0,"type":3}),
        json!({"id":1,"name":"Port","x":0,"y":0}),
    ] {
        let before = s.document().clone();
        assert!(s.execute("shape.connection.add", &p).is_err());
        assert_eq!(s.document(), &before);
    }
}
#[test]
fn dynamic_glue_recomputes_move_after_endpoints_through_host() {
    if !std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus").exists() {
        eprintln!("skip: private corpus absent");
        return;
    }
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus/generated/connector");
    let mut s = Session::new();
    run(&mut s, "file.open", json!({"path":root.join("move-before.vsdx")}));
    let before = number(&s, 3, "EndTrigger");
    run(&mut s, "shape.move", json!({"id":2,"x":7.75,"y":5.75}));
    let expected = diagramcraft_vsdx::read(root.join("move-after.vsdx")).unwrap();
    let mut oracle = Evaluator::new(&expected).unwrap();
    let scope = Scope::PageShape { page: expected.pages[0].id, shape: 3 };
    let mut differences = vec![];
    for name in ["BeginX", "BeginY", "EndX", "EndY", "PinX", "PinY", "Width", "Height", "Controls.TextPosition", "Controls.TextPosition.Y"] {
        let expected = oracle.evaluate(scope, name).numeric().unwrap();
        let actual = number(&s, 3, name);
        if (actual - expected).abs() > 1e-6 {
            differences.push(format!("{name}: {actual} != {expected}"));
        }
    }
    assert!(differences.is_empty(), "{}", differences.join("\n"));
    assert_eq!(number(&s, 3, "EndTrigger"), before + 1.);
    run(&mut s, "edit.undo", json!({}));
    assert_eq!(number(&s, 3, "EndX"), 5.);
    run(&mut s, "edit.redo", json!({}));
    assert_eq!(number(&s, 3, "EndX"), 7.);
}
#[test]
fn dynamic_glue_new_drawing_recomputes_both_ends() {
    let mut s = Session::new();
    for x in [1., 5.] {
        run(&mut s, "shape.draw.rect", json!({"x":x,"y":6,"width":1.5,"height":1.5}));
    }
    run(&mut s, "master.drop", json!({"master":2,"x":4,"y":6.75}));
    for (end, target) in [("begin", 1), ("end", 2)] {
        run(&mut s, "connect.dynamic", json!({"id":3,"end":end,"target":target}));
    }
    assert_eq!(number(&s, 3, "BeginX"), 2.5);
    assert_eq!(number(&s, 3, "EndX"), 5.);
    run(&mut s, "shape.move", json!({"id":2,"x":7.75,"y":5.75}));
    assert_eq!(number(&s, 3, "BeginY"), 6.25);
    assert_eq!(number(&s, 3, "EndX"), 7.);
}
#[test]
fn reroute_preserves_four_straight_witnesses_geometry_and_xform() {
    if !std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus").exists() {
        eprintln!("skip: private corpus absent");
        return;
    }
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus/generated/connector");
    for name in ["dynamic-glue", "move-before", "move-after", "static-glue"] {
        let path = root.join(format!("{name}.vsdx"));
        let original = diagramcraft_vsdx::read(&path).unwrap();
        let mut s = Session::new();
        run(&mut s, "file.open", json!({"path":path}));
        assert_eq!(run(&mut s, "route.reroute", json!({"id":3}))["routed"], 1);
        let scope = Scope::PageShape { page: original.pages[0].id, shape: 3 };
        let mut expected = Evaluator::new(&original).unwrap();
        let mut actual = Evaluator::new(s.document()).unwrap();
        let mut cells = expected
            .resolved_named_cells(scope)
            .into_iter()
            .filter(|n| {
                n.name.starts_with("Geometry")
                    || matches!(
                        n.name.as_str(),
                        "PinX"
                            | "PinY"
                            | "Width"
                            | "Height"
                            | "LocPinX"
                            | "LocPinY"
                            | "Angle"
                            | "FlipX"
                            | "FlipY"
                            | "BeginX"
                            | "BeginY"
                            | "EndX"
                            | "EndY"
                    )
            })
            .map(|n| n.name)
            .collect::<Vec<_>>();
        cells.sort();
        for cell in cells {
            let a = actual.evaluate(scope, &cell);
            let b = expected.evaluate(scope, &cell);
            match (a.numeric(), b.numeric()) {
                (Ok(a), Ok(b)) => assert!((a - b).abs() < 1e-6, "{name} {cell}: {a} != {b}"),
                _ => assert_eq!(a, b, "{name} {cell}"),
            }
        }
        let original =
            expected.source_named_cells(scope).unwrap().into_iter().map(|n| (n.name, n.cell.clone())).collect::<std::collections::BTreeMap<_, _>>();
        let result =
            actual.source_named_cells(scope).unwrap().into_iter().map(|n| (n.name, n.cell.clone())).collect::<std::collections::BTreeMap<_, _>>();
        assert_eq!(original.keys().collect::<Vec<_>>(), result.keys().collect::<Vec<_>>(), "{name} local cell set");
        let shape = &s.document().pages[0].shapes[2];
        assert!(shape.sheet.sections.iter().find(|s| s.name == "Geometry").unwrap().rows.iter().any(|r| r.index == Some(3) && r.deleted));
    }
}
#[test]
fn straight_route_tracks_a_diagonal_endpoint_in_page_space() {
    let mut s = Session::new();
    // The straight style remains a two-row route after right-angle routing is implemented.
    // https://learn.microsoft.com/en-us/office/client-developer/visio/routestyle-cell-page-layout-section
    run(&mut s, "sheet.set", json!({"scope":"page","name":"RouteStyle","value":2}));
    for (x, y) in [(1., 1.), (5., 5.)] {
        run(&mut s, "shape.draw.rect", json!({"x":x,"y":y,"width":1,"height":1}));
    }
    run(&mut s, "master.drop", json!({"master":2,"x":3,"y":3}));
    for (end, target) in [("begin", 1), ("end", 2)] {
        run(&mut s, "connect.dynamic", json!({"id":3,"end":end,"target":target}));
    }
    let mut e = Evaluator::new(s.document()).unwrap();
    let scope = Scope::PageShape { page: s.document().pages[0].id, shape: 3 };
    let transform = e.shape_to_page(scope).unwrap();
    for (row, prefix) in [(1, "Begin"), (2, "End")] {
        let x = e.evaluate(scope, &format!("Geometry1.X{row}")).numeric().unwrap();
        let y = e.evaluate(scope, &format!("Geometry1.Y{row}")).numeric().unwrap();
        let point = transform * diagramcraft_geom::Point::new(x, y);
        assert!((point.x - number(&s, 3, &format!("{prefix}X"))).abs() < 1e-6);
        assert!((point.y - number(&s, 3, &format!("{prefix}Y"))).abs() < 1e-6);
    }
}

/// Local endpoint literals must survive rerouting before both ends are glued.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
#[test]
fn parallel_row_connectors_keep_horizontal_routes_in_either_order() {
    for (first, second) in [(1.2, 2.8), (1.5, 3.), (2.8, 1.2), (4.4, 6.), (6., 1.2), (6., 4.4), (2.8, 1.2), (3., 1.5)] {
        let mut s = Session::new();
        run(&mut s, "file.new", json!({}));
        run(&mut s, "page.size", json!({"width":11,"height":8.5}));
        run(&mut s, "sheet.set", json!({"scope":"page","name":"RouteStyle","value":1}));
        run(&mut s, "shape.draw.rect", json!({"x":0,"y":0,"width":2.3,"height":1}));
        run(&mut s, "master.new_from_shape", json!({"id":1,"name":"Box"}));
        run(&mut s, "shape.delete", json!({"id":1}));
        for y in [first, second] {
            for x in [1.6, 7.1] {
                run(&mut s, "master.drop", json!({"master":"Box","x":x,"y":y}));
            }
        }
        for (id, target, y) in [(5, 1, first), (6, 3, second)] {
            run(&mut s, "master.drop", json!({"master":2,"x":4.35,"y":y}));
            for (end, target) in [("begin", target), ("end", target + 1)] {
                run(&mut s, "connect.dynamic", json!({"id":id,"end":end,"target":target,"preference":"horizontal"}));
            }
        }
        let mut e = Evaluator::new(s.document()).unwrap();
        for (id, y) in [(5, first), (6, second)] {
            let scope = Scope::PageShape { page: s.document().pages[0].id, shape: id };
            let paths = e.geometry_paths(scope, diagramcraft_geom::Rect::new(0., 0., 11., 8.5)).unwrap();
            assert_eq!(paths.len(), 1);
            assert_eq!(paths[0].path.elements().len(), 2, "rows {first}/{second}, connector {id}");
            let transform = e.shape_to_page(scope).unwrap();
            for (row, x) in [(1, 2.75), (2, 5.95)] {
                let px = e.evaluate(scope, &format!("Geometry1.X{row}")).numeric().unwrap();
                let py = e.evaluate(scope, &format!("Geometry1.Y{row}")).numeric().unwrap();
                let point = transform * diagramcraft_geom::Point::new(px, py);
                assert!(
                    (point.x - x).abs() <= 1e-9 && (point.y - y).abs() <= 1e-9,
                    "rows {first}/{second}, connector {id}, point {row}: {point:?}, expected ({x}, {y})"
                );
            }
        }
    }
}
