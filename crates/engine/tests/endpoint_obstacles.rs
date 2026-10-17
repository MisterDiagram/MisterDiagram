//! Selected synthetic #44 regressions from craft-apps review 3a, user-authorized MIT OR Apache-2.0.
use misterdiagram_engine::Session;
use serde_json::{Value, json};
fn run(s: &mut Session, id: &str, p: Value) -> Value {
    s.execute(id, &p).unwrap_or_else(|e| panic!("{id} {p}: {e}"))
}

fn new_page(route_style: i32) -> Session {
    let mut s = Session::new();
    run(&mut s, "file.new", json!({}));
    run(&mut s, "page.size", json!({"width":20,"height":20}));
    run(&mut s, "sheet.set", json!({"scope":"page","name":"RouteStyle","value":route_style}));
    s
}

fn connect(s: &mut Session, conn: u32, a: u32, b: u32) {
    for (end, target) in [("begin", a), ("end", b)] {
        run(s, "connect.dynamic", json!({"id":conn,"end":end,"target":target}));
    }
}

#[test]
fn r3a_6_move_placeable_shape_over_glued_endpoint() {
    let mut s = new_page(1);
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":4,"width":1,"height":1}));
    run(&mut s, "shape.draw.rect", json!({"x":6,"y":5,"width":1,"height":1}));
    run(&mut s, "shape.draw.rect", json!({"x":3.5,"y":3,"width":1,"height":4}));
    run(&mut s, "sheet.set", json!({"id":3,"name":"ObjType","value":1}));
    run(&mut s, "master.drop", json!({"master":2,"x":12,"y":15.5}));
    connect(&mut s, 4, 1, 2);
    let r = s.execute("shape.move", &json!({"id":3,"x":2.0,"y":4.5}));
    assert!(r.is_ok(), "shape.move refused: {}", r.err().map(|e| e.to_string()).unwrap_or_default());
}

#[test]
fn r3a_6b_glue_shapes_standing_on_placeable_background() {
    let mut s = new_page(1);
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":4,"width":1,"height":1}));
    run(&mut s, "shape.draw.rect", json!({"x":6,"y":5,"width":1,"height":1}));
    run(&mut s, "shape.draw.rect", json!({"x":0,"y":0,"width":10,"height":9}));
    run(&mut s, "sheet.set", json!({"id":3,"name":"ObjType","value":1}));
    run(&mut s, "master.drop", json!({"master":2,"x":12,"y":15.5}));
    for (end, target) in [("begin", 1), ("end", 2)] {
        let r = s.execute("connect.dynamic", &json!({"id":4,"end":end,"target":target}));
        assert!(r.is_ok(), "connect.dynamic {end} refused: {}", r.err().map(|e| e.to_string()).unwrap_or_default());
    }
}

#[test]
fn dropping_connector_onto_glued_shape_succeeds() {
    let mut s = new_page(1);
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":4,"width":2,"height":2}));
    run(&mut s, "shape.draw.rect", json!({"x":6,"y":4,"width":1,"height":1}));
    run(&mut s, "master.drop", json!({"master":2,"x":4.5,"y":4.5}));
    connect(&mut s, 3, 1, 2);
    run(&mut s, "master.drop", json!({"master":2,"x":2,"y":5}));
    assert_eq!(s.document().pages[0].shapes.len(), 4);
    assert_eq!(s.document().pages[0].connects.len(), 2);
    assert!(misterdiagram_vsdx::write(s.document()).is_ok());
}
#[test]
fn duplicated_shape_covering_static_glue_point_succeeds() {
    let mut s = new_page(1);
    for x in [1, 5] {
        run(&mut s, "shape.draw.rect", json!({"x":x,"y":4,"width":1,"height":1}));
    }
    run(&mut s, "shape.connection.add", json!({"id":1,"x":"Width","y":"Height*0.5"}));
    run(&mut s, "shape.connection.add", json!({"id":2,"x":0,"y":"Height*0.5"}));
    run(&mut s, "master.drop", json!({"master":2,"x":3.5,"y":4.5}));
    for (end, target) in [("begin", 1), ("end", 2)] {
        run(&mut s, "connect.static", json!({"id":3,"end":end,"target":target,"point":"Connections.X1"}));
    }
    run(&mut s, "shape.duplicate", json!({"id":1}));
    assert_eq!(s.document().pages[0].shapes.len(), 4);
    assert_eq!(s.document().pages[0].connects.len(), 2);
    assert!(misterdiagram_vsdx::write(s.document()).is_ok());
}

#[test]
fn aligned_connector_can_detach_inside_its_former_glue_targets() {
    let mut s = Session::new();
    for x in [1, 6] {
        run(&mut s, "shape.draw.rect", json!({"x":x,"y":4,"width":1,"height":1}));
    }
    run(&mut s, "master.drop", json!({"master":2,"x":4,"y":1}));
    connect(&mut s, 3, 1, 2);
    run(&mut s, "arrange.align", json!({"ids":[1,3],"mode":"bottom"}));
    assert!(s.document().pages[0].connects.is_empty());
    run(&mut s, "shape.move", json!({"id":1,"x":1.5,"y":8}));
    assert!(misterdiagram_vsdx::write(s.document()).is_ok());
}
