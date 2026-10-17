//! Selected synthetic tests from craft-apps review 3a, user-authorized MIT OR Apache-2.0.
//! W3/W4 expectations and additional cases are contributor-original.
use misterdiagram_engine::Session;
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
use serde_json::{Value, json};

fn run(s: &mut Session, id: &str, p: Value) -> Value {
    s.execute(id, &p).unwrap_or_else(|e| panic!("{id} {p}: {e}"))
}
fn num(s: &Session, id: u32, name: &str) -> f64 {
    Evaluator::new(s.document()).unwrap().evaluate(Scope::PageShape { page: s.document().pages[0].id, shape: id }, name).numeric().unwrap()
}
fn rows(s: &Session, id: u32) -> Vec<(String, f64, f64, Option<f64>)> {
    let mut e = Evaluator::new(s.document()).unwrap();
    let scope = Scope::PageShape { page: s.document().pages[0].id, shape: id };
    let mut found = std::collections::BTreeMap::new();
    for n in e.resolved_named_cells(scope) {
        if n.section.is_some_and(|s| s.name == "Geometry" && s.index.unwrap_or(0) == 0)
            && n.cell.name == "X"
            && let Some(r) = n.row.filter(|r| !r.deleted)
        {
            found.insert(r.index.unwrap(), r.row_type.clone().unwrap_or_default());
        }
    }
    found
        .into_iter()
        .map(|(ix, kind)| {
            let x = e.evaluate(scope, &format!("Geometry1.X{ix}")).numeric().unwrap();
            let y = e.evaluate(scope, &format!("Geometry1.Y{ix}")).numeric().unwrap();
            let a = e.evaluate(scope, &format!("Geometry1.A{ix}")).numeric().ok();
            (kind, x, y, a)
        })
        .collect()
}
fn connect(s: &mut Session, conn: u32, a: u32, b: u32) {
    for (end, target) in [("begin", a), ("end", b)] {
        run(s, "connect.dynamic", json!({"id":conn,"end":end,"target":target}));
    }
}

fn new_page(route_style: i32) -> Session {
    let mut s = Session::new();
    run(&mut s, "file.new", json!({}));
    run(&mut s, "page.size", json!({"width":20,"height":20}));
    run(&mut s, "sheet.set", json!({"scope":"page","name":"RouteStyle","value":route_style}));
    s
}

// ---------------------------------------------------------------------------------------
// 3a-1: a route elsewhere on the page rewrites an untouched glued 1-D shape's Geometry1.
// Oracle: invariant. Gluing connector 6 between shapes 4 and 5 must not change shape 3.
// Shape 3 is a static-glued line whose path is MoveTo(0,0) LineTo(1,0) ArcTo(4,0,A=0.5) LineTo(5,0).
fn curved_line_session() -> Session {
    let mut s = new_page(1);
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":4,"width":1,"height":1}));
    run(&mut s, "shape.draw.rect", json!({"x":6,"y":4,"width":1,"height":1}));
    for id in [1, 2] {
        run(&mut s, "shape.connection.add", json!({"id":id,"x":"Width*0.5","y":"Height*0.5"}));
    }
    run(&mut s, "shape.draw.line", json!({"x1":1.5,"y1":4.5,"x2":6.5,"y2":4.5}));
    run(&mut s, "connect.static", json!({"id":3,"end":"begin","target":1,"point":"Connections.X1"}));
    run(&mut s, "connect.static", json!({"id":3,"end":"end","target":2,"point":"Connections.X1"}));
    run(&mut s, "sheet.set", json!({"id":3,"name":"Geometry1.X2","formula":"Width*0.2"}));
    run(&mut s, "sheet.row.add", json!({"id":3,"section":"Geometry","section_index":0,"type":"ArcTo"}));
    run(&mut s, "sheet.set", json!({"id":3,"name":"Geometry1.X3","formula":"Width*0.8"}));
    run(&mut s, "sheet.set", json!({"id":3,"name":"Geometry1.Y3","formula":"0 in"}));
    run(&mut s, "sheet.set", json!({"id":3,"name":"Geometry1.A3","formula":"0.5 in"}));
    run(&mut s, "sheet.row.add", json!({"id":3,"section":"Geometry","section_index":0,"type":"LineTo"}));
    run(&mut s, "sheet.set", json!({"id":3,"name":"Geometry1.X4","formula":"Width*1"}));
    run(&mut s, "sheet.set", json!({"id":3,"name":"Geometry1.Y4","formula":"0 in"}));
    s
}
#[test]
fn r3a_1_unrelated_route_keeps_glued_arc_geometry() {
    let mut s = curved_line_session();
    let before = rows(&s, 3);
    assert_eq!(before.len(), 4, "setup: {before:?}");
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":8,"width":1,"height":1}));
    run(&mut s, "shape.draw.rect", json!({"x":6,"y":8,"width":1,"height":1}));
    run(&mut s, "master.drop", json!({"master":2,"x":4,"y":10}));
    connect(&mut s, 6, 4, 5);
    assert_eq!(rows(&s, 3), before, "shape 3 Geometry1 changed by an unrelated connector");
}
/// MS LineJumpCode/ConLineJumpCode pages: jumps are added to connectors. Shape 3 is a plain
/// line (ObjType 0, not routable). W3 observes no jump on either crossing shape.
#[test]
fn r3a_1b_plain_glued_line_gets_no_line_jump() {
    let mut s = new_page(1);
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":4,"width":1,"height":1}));
    run(&mut s, "shape.draw.rect", json!({"x":6,"y":4,"width":1,"height":1}));
    for id in [1, 2] {
        run(&mut s, "shape.connection.add", json!({"id":id,"x":"Width*0.5","y":"Height*0.5"}));
    }
    run(&mut s, "shape.draw.line", json!({"x1":1.5,"y1":4.5,"x2":6.5,"y2":4.5}));
    run(&mut s, "connect.static", json!({"id":3,"end":"begin","target":1,"point":"Connections.X1"}));
    run(&mut s, "connect.static", json!({"id":3,"end":"end","target":2,"point":"Connections.X1"}));
    assert_eq!(num(&s, 3, "ObjType"), 0.);
    run(&mut s, "shape.draw.rect", json!({"x":3.5,"y":0.5,"width":1,"height":1}));
    run(&mut s, "shape.draw.rect", json!({"x":3.5,"y":8,"width":1,"height":1}));
    run(&mut s, "master.drop", json!({"master":2,"x":4,"y":10}));
    connect(&mut s, 6, 4, 5);
    let kinds: Vec<_> = rows(&s, 3).into_iter().map(|r| r.0).collect();
    assert_eq!(kinds, ["MoveTo", "LineTo"], "plain line was given a jump");
    run(&mut s, "sheet.set", json!({"scope":"page","name":"LineJumpCode","value":2}));
    for id in [3, 6] {
        assert_eq!(rows(&s, id).iter().map(|r| r.0.as_str()).collect::<Vec<_>>(), ["MoveTo", "LineTo"], "W3 shape {id} received a jump");
    }
}

// 3a-3: glue and reroute overwrite user literals in ConFixedCode and ObjType.
// Oracle: sheet.set must stick (or the command must fail); MS ConFixedCode page (2 = Never
// reroute); MS ObjType page (Visio resets 0 to 1 on glue; the &H4 bit takes precedence).
#[test]
fn r3a_3_confixedcode_never_reroute_is_kept() {
    let mut s = new_page(1);
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":4,"width":1,"height":1}));
    run(&mut s, "shape.draw.rect", json!({"x":6,"y":4,"width":1,"height":1}));
    run(&mut s, "master.drop", json!({"master":2,"x":4,"y":8}));
    connect(&mut s, 3, 1, 2);
    run(&mut s, "sheet.set", json!({"id":3,"name":"ConFixedCode","value":2}));
    assert_eq!(num(&s, 3, "ConFixedCode"), 2., "sheet.set ConFixedCode=2 did not stick");
}
#[test]
fn r3a_3b_glue_keeps_objtype_not_placeable() {
    let mut s = new_page(1);
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":4,"width":1,"height":1}));
    run(&mut s, "shape.draw.rect", json!({"x":6,"y":4,"width":1,"height":1}));
    run(&mut s, "sheet.set", json!({"id":1,"name":"ObjType","value":4}));
    run(&mut s, "master.drop", json!({"master":2,"x":4,"y":8}));
    connect(&mut s, 3, 1, 2);
    assert_eq!(num(&s, 1, "ObjType"), 4.);
}

// ---------------------------------------------------------------------------------------
// 3a-4: reroute drops the connector's local Geometry1 section cells.
// Oracle: MS NoShow/NoLine/NoSnap/NoFill pages (section-level cells); a reroute changes rows only.
#[test]
fn r3a_4_reroute_keeps_geometry_section_cells() {
    let mut failures = vec![];
    for (cell, value) in
        [("Geometry1.NoQuickDrag", 1.), ("Geometry1.NoShow", 1.), ("Geometry1.NoLine", 1.), ("Geometry1.NoSnap", 1.), ("Geometry1.NoFill", 0.)]
    {
        let mut s = new_page(1);
        run(&mut s, "shape.draw.rect", json!({"x":1,"y":4,"width":1,"height":1}));
        run(&mut s, "shape.draw.rect", json!({"x":6,"y":4,"width":1,"height":1}));
        run(&mut s, "master.drop", json!({"master":2,"x":4,"y":8}));
        connect(&mut s, 3, 1, 2);
        run(&mut s, "sheet.set", json!({"id":3,"name":cell,"value":value}));
        assert_eq!(num(&s, 3, cell), value);
        run(&mut s, "shape.move", json!({"id":2,"x":6.5,"y":6.0}));
        let after = num(&s, 3, cell);
        if after != value {
            failures.push(format!("{cell}: {value} -> {after}"));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn never_reroute_follows_glued_ends_without_replacing_the_path() {
    let mut s = new_page(1);
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":6,"width":1.5,"height":1.5}));
    run(&mut s, "shape.draw.rect", json!({"x":5,"y":6,"width":1.5,"height":1.5}));
    run(&mut s, "master.drop", json!({"master":2,"x":4,"y":8}));
    connect(&mut s, 3, 1, 2);
    run(&mut s, "sheet.set", json!({"id":3,"name":"ConFixedCode","value":2}));
    let before = rows(&s, 3);
    assert_eq!(before.iter().map(|r| r.0.as_str()).collect::<Vec<_>>(), ["MoveTo", "LineTo"]);
    assert_eq!(num(&s, 3, "Geometry1.X2"), 2.5);
    run(&mut s, "shape.move", json!({"id":2,"x":6.75,"y":6.75}));
    assert_eq!(num(&s, 3, "ConFixedCode"), 2.);
    assert_eq!(num(&s, 3, "BeginX"), 2.5);
    assert_eq!(num(&s, 3, "EndX"), 6.);
    assert_eq!(num(&s, 3, "Geometry1.X2"), 3.5);
    assert_eq!(num(&s, 3, "Geometry1.Y2"), 0.125);
    let before = rows(&s, 3);
    assert_eq!(run(&mut s, "route.reroute", json!({"id":3}))["routed"], 0);
    assert_eq!(rows(&s, 3), before);
    run(&mut s, "shape.draw.rect", json!({"x":3,"y":6,"width":1,"height":1.5}));
    run(&mut s, "sheet.set", json!({"id":4,"name":"ObjType","value":1}));
    assert_eq!(rows(&s, 3), before, "fixed connector routed around a new obstacle");
}

#[test]
fn explicit_routing_policies_and_target_flags_survive_both_glue_calls() {
    for policy in [0, 1, 2, 3, 6] {
        for static_glue in [false, true] {
            let mut s = new_page(1);
            for x in [1, 6] {
                run(&mut s, "shape.draw.rect", json!({"x":x,"y":4,"width":1,"height":1}));
            }
            for id in [1, 2] {
                run(&mut s, "shape.connection.add", json!({"id":id,"x":"Width*0.5","y":"Height*0.5"}));
                run(&mut s, "sheet.set", json!({"id":id,"name":"ObjType","value":5}));
            }
            run(&mut s, "master.drop", json!({"master":2,"x":4,"y":8}));
            run(&mut s, "sheet.set", json!({"id":3,"name":"ConFixedCode","value":policy}));
            for (end, target) in [("begin", 1), ("end", 2)] {
                if static_glue {
                    run(&mut s, "connect.static", json!({"id":3,"end":end,"target":target,"point":"Connections.X1"}));
                } else {
                    run(&mut s, "connect.dynamic", json!({"id":3,"end":end,"target":target}));
                }
                assert_eq!(num(&s, 3, "ConFixedCode"), f64::from(policy), "policy {policy}, static={static_glue}");
                assert_eq!(num(&s, target, "ObjType"), 5.);
            }
        }
    }
}

#[test]
fn unrelated_routes_preserve_polyline_and_nurbs_rows() {
    for kind in ["PolylineTo", "NURBSTo"] {
        let mut s = curved_line_session();
        run(&mut s, "sheet.row.add", json!({"id":3,"section":"Geometry","section_index":0,"type":kind}));
        run(&mut s, "sheet.set", json!({"id":3,"name":"Geometry1.X5","value":5}));
        let before = s.document().pages[0].shapes.iter().find(|s| s.id == 3).unwrap().sheet.sections.clone();
        run(&mut s, "shape.draw.rect", json!({"x":1,"y":8,"width":1,"height":1}));
        run(&mut s, "shape.draw.rect", json!({"x":6,"y":8,"width":1,"height":1}));
        run(&mut s, "master.drop", json!({"master":2,"x":4,"y":10}));
        connect(&mut s, 6, 4, 5);
        assert_eq!(s.document().pages[0].shapes.iter().find(|s| s.id == 3).unwrap().sheet.sections, before, "{kind} row changed");
    }
}

#[test]
fn never_reroute_relative_terminal_rows_keep_fractional_coordinates() {
    let mut s = new_page(1);
    for x in [1., 5.] {
        run(&mut s, "shape.draw.rect", json!({"x":x,"y":6,"width":1.5,"height":1.5}));
    }
    run(&mut s, "master.drop", json!({"master":2,"x":4,"y":8}));
    connect(&mut s, 3, 1, 2);
    run(&mut s, "sheet.set", json!({"id":3,"name":"ConFixedCode","value":2}));
    s.edit_document(|doc, _| {
        let shape = doc.pages[0].shapes.iter_mut().find(|s| s.id == 3).unwrap();
        let section = shape.sheet.sections.iter_mut().find(|s| s.name == "Geometry").unwrap();
        for (ix, kind, x) in [(1, "RelMoveTo", "0"), (2, "RelLineTo", "1")] {
            let row = section.rows.iter_mut().find(|r| r.index == Some(ix)).unwrap();
            row.row_type = Some(kind.into());
            row.cells = [("X", x), ("Y", "0.5")]
                .into_iter()
                .map(|(axis, value)| misterdiagram_model::Cell { name: axis.into(), value: Some(value.into()), ..Default::default() })
                .collect();
        }
        Ok(())
    })
    .unwrap();
    run(&mut s, "shape.move", json!({"id":2,"x":6.75,"y":6.75}));
    assert_eq!(num(&s, 3, "Geometry1.X2"), 1., "relative terminal X must remain a Width fraction");
    assert_eq!(num(&s, 3, "Geometry1.Y2"), 0.5, "relative terminal Y must remain a Height fraction");
    assert_eq!(num(&s, 3, "EndX"), 6.);
    assert_eq!(rows(&s, 3).iter().map(|r| r.0.as_str()).collect::<Vec<_>>(), ["RelMoveTo", "RelLineTo"]);
}
