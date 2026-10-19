//! Original rooted topology fixture, with B25 COM midpoint routing evidence.
use misterdiagram_engine::Session;
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
use serde_json::{Value, json};
fn run(s: &mut Session, id: &str, p: Value) -> Value {
    s.execute(id, &p).unwrap_or_else(|e| panic!("{id}: {e}"))
}
fn office() -> Session {
    let mut s = Session::new();
    run(&mut s, "stencil.open", json!({"builtin":"network"}));
    for master in [
        "network.cloud",
        "network.firewall",
        "network.router",
        "network.switch.core",
        "network.switch.access",
        "network.switch.access",
        "compute.server",
        "network.wireless_ap",
        "compute.desktop",
        "compute.desktop",
    ] {
        run(&mut s, "master.drop", json!({"master":master}));
    }
    s
}
fn edges() -> Value {
    json!([[1, 2], [2, 3], [3, 4], [4, 7], [4, 5], [4, 6], [5, 9], [6, 10], [6, 8]])
}
#[test]
fn tree_centres_tiers_fits_equal_margins_and_glues_midpoint_routes() {
    let mut s = office();
    let result = run(&mut s, "layout.tree", json!({"root":1,"edges":edges(),"gutter_x":2,"gutter_y":0.5,"fit_page":true}));
    assert_eq!(s.document().pages[0].connects.len(), 18);
    let doc = s.document();
    let page = doc.pages[0].id;
    let mut e = Evaluator::new(doc).unwrap();
    let mut number = |id, name: &str| e.evaluate(Scope::PageShape { page, shape: id }, name).numeric().unwrap();
    let mut bounds = [f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY];
    for row in result["placed"].as_array().unwrap() {
        let id = row["id"].as_u64().unwrap() as u32;
        let x = number(id, "PinX");
        let y = number(id, "PinY");
        let w = number(id, "Width");
        let h = number(id, "Height");
        assert_eq!(x, row["x"].as_f64().unwrap());
        assert_eq!(y, row["y"].as_f64().unwrap());
        for v in [x, y, w, h] {
            assert_eq!((v * 64.).fract(), 0.);
        }
        bounds = [bounds[0].min(x - w / 2.), bounds[1].min(y - h / 2.), bounds[2].max(x + w / 2.), bounds[3].max(y + h / 2.)];
    }
    for ids in [vec![7, 5, 6], vec![9, 10, 8]] {
        let y = number(ids[0], "PinY");
        for id in ids {
            assert_eq!(number(id, "PinY"), y);
        }
    }
    for (parent, kids) in [(4, vec![7, 5, 6]), (6, vec![10, 8])] {
        let min = kids.iter().map(|id| number(*id, "PinX") - 0.75).fold(f64::INFINITY, f64::min);
        let max = kids.iter().map(|id| number(*id, "PinX") + 0.75).fold(f64::NEG_INFINITY, f64::max);
        assert!((number(parent, "PinX") - (min + max) / 2.).abs() <= 1. / 64.);
    }
    for id in 11..=19 {
        let by = number(id, "BeginY");
        let ey = number(id, "EndY");
        let bx = number(id, "BeginX");
        let ex = number(id, "EndX");
        if bx != ex {
            assert!((number(id, "Geometry1.Y2") - (ey - by) / 2.).abs() < 1e-6);
        }
    }
    let width = e.evaluate(Scope::Page(page), "PageWidth").numeric().unwrap();
    let height = e.evaluate(Scope::Page(page), "PageHeight").numeric().unwrap();
    assert!(width > height);
    for m in [bounds[0], bounds[1], width - bounds[2], height - bounds[3]] {
        assert_eq!(m, 0.5);
    }
    assert!(doc.pages[0].connects.iter().all(|c| c.to_cell.as_deref().is_some_and(|n| n.starts_with("Connections.X"))));
    let count = doc.pages[0].shapes.len();
    run(&mut s, "layout.tree", json!({"root":1,"ids":[1,2,3,4,5,6,7,8,9,10],"gutter_x":2,"gutter_y":0.5,"fit_page":true}));
    assert_eq!(s.document().pages[0].shapes.len(), count);
}
#[test]
fn hostile_trees_reject_without_partial_placement_or_connectors() {
    let mut s = office();
    for p in [
        json!({}),
        json!({"root":1}),
        json!({"root":1,"edges":[[1,2],[2,1]]}),
        json!({"root":1,"edges":[[1,2],[1,3],[3,2]]}),
        json!({"root":1,"edges":[[1,2],[3,4]]}),
        json!({"root":1,"edges":[[1,999]]}),
        json!({"root":1,"edges":[[1,2]],"region":{"x":0,"y":0,"width":0.1,"height":0.1}}),
        json!({"root":1,"edges":[[1,2]],"gutter_y":-1}),
        json!({"root":1,"edges":[[1,2]],"margin":1e20}),
        json!({"root":1,"edges":vec![json!([1,2]);10000]}),
    ] {
        let before = s.document().clone();
        assert!(s.execute("layout.tree", &p).is_err(), "{p}");
        assert_eq!(s.document(), &before);
    }
}

#[test]
fn tree_region_defaults_and_position_dependent_bounds_are_checked() {
    let mut s = Session::new();
    for _ in 0..3 {
        run(&mut s, "shape.draw.rect", json!({"x":0,"y":0,"width":1,"height":0.5}));
    }
    let result = run(&mut s, "layout.tree", json!({"root":1,"edges":[[1,2],[1,3]],"region":{"x":1,"y":2,"width":8,"height":6}}));
    assert_eq!(result["placed"][0]["x"], 5.);
    let page = s.document().pages[0].id;
    let mut e = Evaluator::new(s.document()).unwrap();
    for (key, block, line) in [("gutter_x", "BlockSizeX", "LineToNodeX"), ("gutter_y", "BlockSizeY", "LineToNodeY")] {
        let desired = e.evaluate(Scope::Page(page), block).numeric().unwrap().max(2. * e.evaluate(Scope::Page(page), line).numeric().unwrap());
        assert_eq!(result[key].as_f64().unwrap(), (desired * 32.).ceil() / 32.);
    }
    drop(e);
    run(&mut s, "sheet.set", json!({"id":2,"name":"Width","formula":"PinX+1"}));
    let before = s.document().clone();
    assert!(s.execute("layout.tree", &json!({"root":1,"edges":[[1,2],[1,3]],"fit_page":true})).is_err());
    assert_eq!(s.document(), &before);
}

#[test]
fn tree_rejects_grid_overflow_and_glues_physical_sides_of_flipped_shapes() {
    let mut s = office();
    let before = s.document().clone();
    assert!(s.execute("layout.tree", &json!({"root":1,"edges":[],"region":{"x":0.003,"y":0,"width":1.5,"height":1.25}})).is_err());
    assert_eq!(s.document(), &before);
    for id in 3..=10 {
        run(&mut s, "shape.delete", json!({"id":id}));
    }
    for id in [1, 2] {
        run(&mut s, "sheet.set", json!({"id":id,"name":"FlipY","formula":"1"}));
    }
    run(&mut s, "layout.tree", json!({"root":1,"edges":[[1,2]],"fit_page":true}));
    let mut e = Evaluator::new(s.document()).unwrap();
    let page = s.document().pages[0].id;
    let n = |e: &mut Evaluator<'_>, id, name| e.evaluate(Scope::PageShape { page, shape: id }, name).numeric().unwrap();
    assert_eq!(n(&mut e, 3, "BeginY"), n(&mut e, 1, "PinY") - 0.625);
    assert_eq!(n(&mut e, 3, "EndY"), n(&mut e, 2, "PinY") + 0.625);
}

#[test]
fn tree_rejects_obstacle_detours_and_unsafe_page_fits() {
    let mut s = Session::new();
    for _ in 0..2 {
        run(&mut s, "shape.draw.rect", json!({"x":0,"y":0,"width":1,"height":0.5}));
    }
    run(&mut s, "sheet.set", json!({"id":1,"name":"FlipY","formula":"PinY>1"}));
    let before = s.document().clone();
    assert!(s.execute("layout.tree", &json!({"root":1,"edges":[[1,2]],"gutter_y":0.5,"fit_page":true})).is_err());
    assert_eq!(s.document(), &before);
    run(&mut s, "sheet.set", json!({"id":1,"name":"FlipY","formula":"0"}));
    run(&mut s, "shape.draw.rect", json!({"x":0.7,"y":1.3,"width":0.4,"height":0.4}));
    run(&mut s, "sheet.set", json!({"id":3,"name":"ObjType","formula":"1"}));
    let before = s.document().clone();
    assert!(s.execute("layout.tree", &json!({"root":1,"edges":[[1,2]],"gutter_y":2,"fit_page":true})).is_err());
    assert_eq!(s.document(), &before);
    run(&mut s, "shape.move", json!({"id":3,"x":10,"y":10}));
    let before = s.document().clone();
    assert!(s.execute("layout.tree", &json!({"root":1,"edges":[[1,2]],"fit_page":true})).is_err());
    assert_eq!(s.document(), &before);
}

#[test]
fn fit_page_never_crops_unrelated_one_dimensional_content() {
    let mut s = Session::new();
    run(&mut s, "shape.draw.rect", json!({"x":0,"y":0,"width":1,"height":0.5}));
    run(&mut s, "shape.draw.line", json!({"x1":10,"y1":10,"x2":11,"y2":11}));
    let before = s.document().clone();
    assert!(s.execute("layout.tree", &json!({"root":1,"edges":[],"fit_page":true})).is_err());
    assert_eq!(s.document(), &before);
}
