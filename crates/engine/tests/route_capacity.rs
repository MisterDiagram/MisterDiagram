//! Contributor-original pages for the bounded obstacle routing contract (#39).
use misterdiagram_engine::{EngineError, Session};
use misterdiagram_model::{Cell, skeleton};
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
use serde_json::{Value, json};

fn run(s: &mut Session, id: &str, params: Value) -> Value {
    let result = s.execute(id, &params);
    assert!(result.is_ok(), "capacity page {id} {params} refused: {result:?}");
    result.unwrap()
}

fn page(count: u32, style: u32) -> Session {
    let mut s = Session::new();
    run(&mut s, "sheet.set", json!({"scope":"page","name":"RouteStyle","value":style}));
    // Construct the fixture in one ordinary model edit, then exercise the shipped
    // per-command route/glue/publication path. All boxes are real placeable shapes.
    s.edit_document(|doc, _| {
        for id in 1..=count {
            let (x, y, w, h) = match id {
                1 => (0., 0., 1., 1.),
                2 => (9., 0., 1., 1.),
                3 => (4., -1., 1., 3.),
                _ => (100. + f64::from(id) * 2., 100. + f64::from(id) * 3., 0.5, 0.5),
            };
            let mut shape = skeleton::rectangle(id, x, y, w, h, "Invented obstacle").map_err(EngineError::Operation)?;
            shape.sheet.cells.push(Cell { name: "ObjType".into(), value: Some("1".into()), ..Default::default() });
            doc.pages[0].shapes.push(shape);
        }
        Ok(())
    })
    .unwrap();
    s
}

fn save(s: &mut Session, name: &str, count: usize) {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/route-capacity").join(std::process::id().to_string());
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(name);
    run(s, "file.save", json!({"path":path}));
    let reread = misterdiagram_vsdx::read(&path).unwrap();
    assert_eq!(reread.pages[0].shapes.len(), count);
    println!("capacity page saved: {} ({} shapes, {} Connects)", path.display(), count, reread.pages[0].connects.len());
}

#[test]
fn five_hundred_placeable_shapes_with_straight_connectors_save() {
    let mut s = page(500, 2);
    for target in [1, 2] {
        run(&mut s, "shape.connection.add", json!({"id":target,"x":"Width*0.5","y":"Height*0.5"}));
    }
    for (id, style, glue) in [(501, 2, "connect.static"), (502, 16, "connect.dynamic")] {
        run(&mut s, "master.drop", json!({"master":2,"x":1,"y":0.5}));
        run(&mut s, "sheet.set", json!({"id":id,"name":"ShapeRouteStyle","value":style}));
        for (end, target) in [("begin", 1), ("end", 2)] {
            let mut params = json!({"id":id,"end":end,"target":target});
            if glue == "connect.static" {
                params["point"] = json!("Connections.X1");
            }
            run(&mut s, glue, params);
        }
    }
    assert_eq!(s.document().pages[0].connects.len(), 4);
    save(&mut s, "straight-500.vsdx", 502);
}

#[test]
fn two_hundred_obstacles_route_an_orthogonal_connector() {
    // Keep the requested 200-shape page, and also exercise 200 gathered
    // obstacles after excluding the two glued endpoint shapes.
    for count in [200, 202] {
        let mut s = page(count, 1);
        let connector = count + 1;
        run(&mut s, "master.drop", json!({"master":2,"x":1,"y":0.5}));
        for (end, target) in [("begin", 1), ("end", 2)] {
            run(&mut s, "connect.dynamic", json!({"id":connector,"end":end,"target":target}));
        }
        let doc = s.document();
        let mut e = Evaluator::new(doc).unwrap();
        let scope = Scope::PageShape { page: doc.pages[0].id, shape: connector };
        let rows = e.resolved_named_cells(scope).iter().filter(|c| c.name.starts_with("Geometry1.X")).count();
        assert!(rows >= 4, "orthogonal connector must bend around the blocking shape, got {rows} X cells");
        assert_eq!(e.evaluate(scope, "BeginX").numeric().unwrap(), 1.);
        assert_eq!(s.document().pages[0].connects.len(), 2);
        save(&mut s, &format!("orthogonal-{count}.vsdx"), connector as usize);
    }
}

#[test]
fn capacity_degrades_with_glue_warnings_one_undo_step_and_retry() {
    let mut s = page(260, 1);
    let id = 261;
    run(&mut s, "master.drop", json!({"master":2,"x":1,"y":0.5}));
    for (end, target) in [("begin", 1), ("end", 2)] {
        let result = run(&mut s, "connect.dynamic", json!({"id":id,"end":end,"target":target}));
        assert_eq!(result["route_warnings"][0]["connector"], id);
        assert_eq!(result["route_warnings"][0]["limit"], "route obstacle count limit");
    }
    let before = s.document().clone();
    let result = run(&mut s, "shape.move", json!({"id":2,"x":12,"y":0}));
    assert_eq!(result["route_warnings"].as_array().unwrap().len(), 1);
    let mut e = Evaluator::new(s.document()).unwrap();
    let scope = Scope::PageShape { page: 0, shape: id };
    assert_eq!(e.evaluate(scope, "EndX").numeric().unwrap(), 11.5);
    drop(e);
    run(&mut s, "edit.undo", json!({}));
    assert_eq!(s.document(), &before, "degraded edit must undo in one step");
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plan/brief32");
    std::fs::create_dir_all(&dir).unwrap();
    let result = run(&mut s, "file.save", json!({"path":dir.join("degraded.vsdx")}));
    assert!(result["warnings"].as_array().unwrap().iter().any(|v| v.as_str().unwrap().contains("connector 261")));
    let result = run(&mut s, "route.reroute", json!({"id":id}));
    assert_eq!(result["route_warnings"][0]["connector"], id);
    s.edit_document(|doc, _| {
        doc.pages[0].shapes.retain(|shape| shape.id <= 2 || shape.id == id);
        Ok(())
    })
    .unwrap();
    let result = run(&mut s, "route.reroute", json!({"id":id}));
    assert!(result.get("route_warnings").is_none());
}

#[test]
fn grid_capacity_publishes_but_bad_geometry_still_rolls_back() {
    let mut s = Session::new();
    s.edit_document(|doc, _| {
        let page = &mut doc.pages[0];
        for id in 1..=202 {
            let (x, y, w, h) = match id {
                1 => (0., 0., 1., 1.),
                2 => (1000., 0., 1., 1.),
                _ => (f64::from(id) * 4., -1. - f64::from(id) * 0.001, 1., 3. + f64::from(id) * 0.002),
            };
            let mut shape = skeleton::rectangle(id, x, y, w, h, "").map_err(EngineError::Operation)?;
            shape.sheet.cells.push(Cell { name: "ObjType".into(), value: Some("1".into()), ..Default::default() });
            page.shapes.push(shape);
        }
        skeleton::glue_dynamic(page, 203, 2, [1, 2], [[1., 0.5], [1000., 0.5]]).map_err(EngineError::Operation)?;
        Ok(())
    })
    .unwrap();
    let result = run(&mut s, "route.reroute", json!({"id":203}));
    assert_eq!(result["route_warnings"][0]["limit"], "route grid limit");
    let before = s.document().clone();
    assert!(s.execute("sheet.set", &json!({"id":1,"name":"PinX","value":"NaN"})).is_err());
    assert_eq!(s.document(), &before);
}
