//! Original fixtures exercise each measurable topology rule through engine commands.
use misterdiagram_engine::Session;
use serde_json::{Value, json};
fn run(s: &mut Session, id: &str, p: Value) -> Value {
    s.execute(id, &p).unwrap_or_else(|e| panic!("{id}: {e}"))
}
fn has(value: &Value, rule: &str) -> bool {
    value["violations"].as_array().unwrap().iter().any(|v| v["rule"] == rule)
}
fn tree() -> Session {
    let mut s = Session::new();
    run(&mut s, "stencil.open", json!({"builtin":"network"}));
    for (i, master) in ["network.router", "compute.server", "compute.desktop"].iter().enumerate() {
        run(&mut s, "master.drop", json!({"master":master}));
        run(&mut s, "shape.text", json!({"id":i+1,"text":format!("Device {}",i+1)}));
    }
    run(&mut s, "layout.tree", json!({"root":1,"edges":[[1,2],[1,3]],"fit_page":true,"gutter_x":1,"gutter_y":1}));
    s
}
#[test]
fn checker_accepts_native_tree_trunks_tiers_labels_and_equal_margins() {
    let mut s = tree();
    let before = s.document().clone();
    let result = run(&mut s, "layout.check", json!({"root":1,"fit_page":true}));
    assert!(!has(&result, "a"), "library master glue must satisfy rule a: {result}");
    assert!(!has(&result, "e"), "tree parent and tier placement must satisfy rule e: {result}");
    assert_eq!(result["count"], 0, "{result}");
    assert_eq!(result["connectors"], 2);
    assert_eq!(s.document(), &before, "checker is read-only");
}
#[test]
fn checker_identifies_whole_shape_glue_tier_alignment_and_margins() {
    let mut s = tree();
    run(&mut s, "connect.dynamic", json!({"id":4,"target":1,"end":"begin"}));
    let result = run(&mut s, "layout.check", json!({}));
    assert!(has(&result, "a"), "{result}");
    assert!(result["violations"].as_array().unwrap().iter().any(|v| v["rule"] == "a" && v["connectors"] == json!([4]) && v["shapes"] == json!([1])));
    let mut s = tree();
    run(&mut s, "sheet.set", json!({"id":1,"name":"PinX","formula":"Sheet.2!PinX"}));
    let result = run(&mut s, "layout.check", json!({"root":1}));
    assert!(has(&result, "e"), "{result}");
    let mut s = tree();
    run(&mut s, "sheet.set", json!({"id":2,"name":"PinY","formula":"Sheet.3!PinY+0.25"}));
    let result = run(&mut s, "layout.check", json!({"root":1}));
    assert!(has(&result, "e"), "{result}");
    run(&mut s, "page.size", json!({"width":20,"height":20}));
    let result = run(&mut s, "layout.check", json!({"fit_page":true}));
    assert!(has(&result, "f"), "{result}");
}
#[test]
fn checker_reports_shape_crossing_border_hugging_label_crossing_and_overlap() {
    let mut s = Session::new();
    run(&mut s, "shape.draw.rect", json!({"x":2,"y":2,"width":2,"height":2}));
    run(&mut s, "shape.text", json!({"id":1,"text":"Label"}));
    run(&mut s, "shape.draw.line", json!({"x1":1,"y1":3,"x2":5,"y2":3}));
    let result = run(&mut s, "layout.check", json!({}));
    assert!(has(&result, "b") && has(&result, "g"), "{result}");
    run(&mut s, "shape.draw.line", json!({"x1":1,"y1":1.96875,"x2":5,"y2":1.96875}));
    let result = run(&mut s, "layout.check", json!({}));
    assert!(has(&result, "c"), "{result}");
    run(&mut s, "shape.draw.line", json!({"x1":1.5,"y1":1.96875,"x2":4.5,"y2":1.96875}));
    let result = run(&mut s, "layout.check", json!({}));
    assert!(has(&result, "d"), "{result}");
    assert!(result["violations"].as_array().unwrap().iter().any(|v| v["rule"] == "d" && v["connectors"] == json!([3, 4])));
}
#[test]
fn checker_uses_moved_text_transform_and_rejects_hostile_requests() {
    let mut s = Session::new();
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":1,"width":1,"height":1}));
    run(&mut s, "shape.text", json!({"id":1,"text":"Outside"}));
    run(&mut s, "sheet.set", json!({"id":1,"name":"TxtPinX","value":3}));
    run(&mut s, "shape.draw.line", json!({"x1":3,"y1":1.5,"x2":5,"y2":1.5}));
    let result = run(&mut s, "layout.check", json!({}));
    assert!(has(&result, "g") && !has(&result, "b"), "{result}");
    let before = s.document().clone();
    for params in [json!({"page":0}), json!({"page":999}), json!({"root":999}), json!({"root":-1}), json!({"fit_page":"true"})] {
        assert!(s.execute("layout.check", &params).is_err());
        assert_eq!(s.document(), &before);
    }
}

#[test]
fn checker_sees_grouped_connectors_and_transformed_child_labels() {
    let mut s = Session::new();
    for x in [1, 3] {
        run(&mut s, "shape.draw.line", json!({"x1":x,"y1":1,"x2":x,"y2":2}));
    }
    run(&mut s, "shape.group", json!({"ids":[1,2]}));
    let result = run(&mut s, "layout.check", json!({}));
    assert_eq!(result["connectors"], 2);
    assert!(has(&result, "a"), "{result}");
    let mut s = Session::new();
    for (x, y) in [(1, 1), (5, 1), (10, 10), (12, 10)] {
        run(&mut s, "shape.draw.rect", json!({"x":x,"y":y,"width":1,"height":1}));
    }
    run(&mut s, "shape.text", json!({"id":3,"text":"Nested label"}));
    for (name, value) in [("TxtPinX", -6.5), ("TxtPinY", -8.5), ("TxtAngle", 0.25)] {
        run(&mut s, "sheet.set", json!({"id":3,"name":name,"value":value}));
    }
    run(&mut s, "master.drop", json!({"master":2}));
    run(&mut s, "connect.sides", json!({"id":5,"source":1,"target":2}));
    run(&mut s, "shape.group", json!({"ids":[3,4]}));
    let result = run(&mut s, "layout.check", json!({}));
    assert!(
        result["violations"].as_array().unwrap().iter().any(|v| v["rule"] == "g" && v["shapes"] == json!([3]) && v["connectors"] == json!([5])),
        "{result}"
    );
}

#[test]
fn checker_includes_connector_labels_and_rotated_text_in_crossings_and_fit() {
    let mut s = Session::new();
    for (x, y) in [(1, 1), (5, 1), (3, 2), (3, 5)] {
        run(&mut s, "shape.draw.rect", json!({"x":x,"y":y,"width":1,"height":1}));
    }
    for (id, source, target) in [(5, 1, 2), (6, 3, 4)] {
        run(&mut s, "master.drop", json!({"master":2}));
        run(&mut s, "connect.sides", json!({"id":id,"source":source,"target":target}));
    }
    run(&mut s, "shape.text", json!({"id":5,"text":"Wire label"}));
    for (name, value) in [("TxtWidth", 1.), ("TxtHeight", 0.5), ("TxtPinX", 1.5), ("TxtPinY", 2.5), ("TxtLocPinX", 0.5), ("TxtLocPinY", 0.25)] {
        run(&mut s, "sheet.set", json!({"id":5,"name":name,"value":value}));
    }
    for angle in [0., 0.5] {
        run(&mut s, "sheet.set", json!({"id":5,"name":"TxtAngle","value":angle}));
        let result = run(&mut s, "layout.check", json!({}));
        assert!(
            result["violations"].as_array().unwrap().iter().any(|v| v["rule"] == "g" && v["shapes"] == json!([5]) && v["connectors"] == json!([6])),
            "{result}"
        );
    }
    let mut s = tree();
    run(&mut s, "shape.text", json!({"id":4,"text":"Outside frame"}));
    run(&mut s, "sheet.set", json!({"id":4,"name":"TxtPinX","value":30}));
    let result = run(&mut s, "layout.check", json!({"fit_page":true}));
    assert!(has(&result, "f"), "{result}");
}

#[test]
fn border_clearance_includes_exact_sixteenth_and_physical_flips_route_midway() {
    for (clearance, violation) in [(1. / 32., true), (1. / 16., true), (5. / 64., false)] {
        let mut s = Session::new();
        run(&mut s, "shape.draw.rect", json!({"x":2,"y":2,"width":2,"height":2}));
        run(&mut s, "shape.draw.line", json!({"x1":1,"y1":2.-clearance,"x2":5,"y2":2.-clearance}));
        let result = run(&mut s, "layout.check", json!({}));
        assert_eq!(has(&result, "c"), violation, "{result}");
    }
    for (flipped, source, target) in [(1, 1, 2), (2, 1, 2), (1, 2, 1), (2, 2, 1)] {
        let mut s = Session::new();
        for (x, y) in [(1, 6), (5, 1)] {
            run(&mut s, "shape.draw.rect", json!({"x":x,"y":y,"width":1,"height":1}));
        }
        run(&mut s, "sheet.set", json!({"id":flipped,"name":"FlipY","value":1}));
        run(&mut s, "master.drop", json!({"master":2}));
        run(&mut s, "connect.sides", json!({"id":3,"source":source,"target":target}));
        let result = run(&mut s, "layout.check", json!({}));
        assert!(!has(&result, "c"), "{result}");
        assert_eq!(result["count"], 0, "{result}");
        assert_eq!(run(&mut s, "sheet.get", json!({"id":3,"name":"ShapeRouteStyle"}))["value"], "17");
        let mut e = misterdiagram_vsdx::evaluate::Evaluator::new(s.document()).unwrap();
        let scope = misterdiagram_vsdx::evaluate::Scope::PageShape { page: s.document().pages[0].id, shape: 3 };
        let n = |e: &mut misterdiagram_vsdx::evaluate::Evaluator<'_>, name: &str| e.evaluate(scope, name).numeric().unwrap();
        assert!((n(&mut e, "Geometry1.Y2") - (n(&mut e, "EndY") - n(&mut e, "BeginY")) / 2.).abs() < 1e-9);
    }
}

/// Original synthetic Geometry rows retain endpoint cells while breaking route continuity.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/geometry-section
#[test]
fn checker_rejects_detached_shared_fragments_and_drawn_endpoint_mismatches() {
    use misterdiagram_model::{Cell, Row};
    for detached_end in [false, true] {
        let mut s = tree();
        let mut doc = s.document().clone();
        for shape in doc.pages[0].shapes.iter_mut().filter(|s| [4, 5].contains(&s.id)) {
            let rows = shape.sheet.sections.iter_mut().find(|section| section.name == "Geometry" && section.index.unwrap_or(0) == 0).unwrap();
            let mut points = vec![("MoveTo", "0", "0"), ("LineTo", "0", "-0.25"), ("MoveTo", "0", "-0.5"), ("LineTo", "0", "-0.75")];
            if !detached_end {
                points.extend([("MoveTo", "0", "-0.9"), ("LineTo", "Width", "-1")]);
            }
            rows.rows = points
                .into_iter()
                .enumerate()
                .map(|(i, (kind, x, y))| Row {
                    index: Some(i as u32 + 1),
                    row_type: Some(kind.into()),
                    cells: vec![
                        Cell {
                            name: "X".into(),
                            value: Some(if x == "Width" { "0" } else { x }.into()),
                            formula: (x == "Width").then(|| "Width".into()),
                            ..Default::default()
                        },
                        Cell { name: "Y".into(), value: Some(y.into()), ..Default::default() },
                    ],
                    ..Default::default()
                })
                .collect();
        }
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/layout-check-tests");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join(format!("disconnected-{detached_end}.vsdx"));
        let themes = misterdiagram_theme::Themes::from_document(&doc).unwrap();
        std::fs::write(&path, misterdiagram_vsdx::write_with_hosts(&doc, Some(&themes), Some(&misterdiagram_text::Host)).unwrap().bytes).unwrap();
        run(&mut s, "file.open", json!({"path":path}));
        let result = run(&mut s, "layout.check", json!({}));
        assert!(has(&result, "d"), "detached segment must fail d: {result}");
        assert_eq!(has(&result, "a"), detached_end, "drawn endpoints: {result}");
    }
}

#[test]
fn fan_keeps_incoming_isl_tracks_distinct_at_both_ends() {
    let mut s = Session::new();
    for (x, y, w, h) in [(1., 1., 1.5, 1.), (3.125, 4., 0.5, 0.5), (6.125, 7., 0.5, 0.5)] {
        run(&mut s, "shape.draw.rect", json!({"x":x,"y":y,"width":w,"height":h}));
    }
    for (connector, spoke) in [(4, 2), (5, 3)] {
        run(&mut s, "master.drop", json!({"master":2}));
        run(&mut s, "connect.sides", json!({"id":connector,"source":spoke,"target":1}));
    }
    run(&mut s, "layout.fan", json!({"hub":1,"spokes":[2,3]}));
    let result = run(&mut s, "layout.check", json!({}));
    assert!(!has(&result, "d"), "ISLs must not share a segment: {result}");
    assert_eq!(result["count"], 0, "{result}");
}
