//! Contributor-original routing scope and serialized preservation controls.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/confixedcode-cell-shape-layout-section
//! https://learn.microsoft.com/en-us/office/client-developer/visio/objtype-cell-miscellaneous-section
use misterdiagram_engine::Session;
use misterdiagram_model::{Cell, Document, Row, Section, Shape, Sheet, skeleton};
use serde_json::{Value, json};

fn set(sheet: &mut Sheet, name: &str, value: f64) {
    sheet.cells.retain(|c| c.name != name);
    sheet.cells.push(cell(name, value));
}
fn cell(name: &str, value: f64) -> Cell {
    Cell { name: name.into(), value: Some(value.to_string()), ..Default::default() }
}
fn line(id: u32, x: f64, y: f64, policy: f64, points: &[(f64, f64)]) -> Shape {
    let mut shape = skeleton::rectangle(id, x, y, 6., 2., "").unwrap();
    for (name, value) in [("ObjType", 2.), ("ConFixedCode", policy), ("BeginX", x), ("BeginY", y), ("EndX", x + 6.), ("EndY", y)] {
        set(&mut shape.sheet, name, value);
    }
    shape.sheet.sections = vec![Section {
        name: "Geometry".into(),
        index: Some(0),
        cells: vec![cell("NoFill", 1.)],
        rows: points
            .iter()
            .enumerate()
            .map(|(i, (x, y))| Row {
                index: Some((i + 1) as u32),
                row_type: Some(if i == 0 { "MoveTo" } else { "LineTo" }.into()),
                cells: vec![cell("X", *x), cell("Y", *y)],
                ..Default::default()
            })
            .collect(),
        ..Default::default()
    }];
    shape
}
fn drawing() -> Document {
    let mut d = skeleton::document(false);
    set(&mut d.pages[0].sheet, "PageWidth", 30.);
    set(&mut d.pages[0].sheet, "PageHeight", 30.);
    set(&mut d.pages[0].sheet, "RouteStyle", 1.);
    set(&mut d.pages[0].sheet, "LineJumpCode", 1.);
    for id in 1..=4 {
        d.pages[0].shapes.push(line(id, 1., f64::from(id) * 4., f64::from(id - 1), &[(0., 0.), (2., 0.), (2., 2.), (6., 2.), (6., 0.)]));
    }
    for (id, kind) in [(20, 1.), (21, 4.), (22, 0.)] {
        let mut s = skeleton::rectangle(id, 20., 20., 0.5, 0.5, "label").unwrap();
        set(&mut s.sheet, "ObjType", kind);
        d.pages[0].shapes.push(s);
    }
    let mut h = line(30, 10., 2., 0., &[(0., 0.), (1.9, 0.), (2.1, 0.), (6., 0.)]);
    h.sheet.sections[0].rows[2].row_type = Some("ArcTo".into());
    h.sheet.sections[0].rows[2].cells.push(cell("A", -0.1));
    d.pages[0].shapes.push(h);
    let mut v = line(31, 12., 0., 0., &[(0., 0.), (0., 4.)]);
    set(&mut v.sheet, "EndX", 12.);
    set(&mut v.sheet, "EndY", 4.);
    d.pages[0].shapes.push(v);
    for (id, x) in [(40, 1.), (41, 7.)] {
        d.pages[0].shapes.push(skeleton::rectangle(id, x, 24., 1., 1., "").unwrap());
    }
    skeleton::glue_dynamic(&mut d.pages[0], 42, 2, [40, 41], [[2., 24.5], [7., 24.5]]).unwrap();
    d
}
struct Opened {
    s: Session,
    path: std::path::PathBuf,
}
impl Opened {
    fn new(d: &Document, name: &str) -> Self {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../target/debug/routing-scope-test-{name}-{}.vsdx", std::process::id()));
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, misterdiagram_vsdx::write(d).unwrap().bytes).unwrap();
        let mut s = Session::new();
        run(&mut s, "file.open", json!({"path":path}));
        Self { s, path }
    }
}
impl Drop for Opened {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}
fn run(s: &mut Session, cmd: &str, p: Value) -> Value {
    s.execute(cmd, &p).unwrap_or_else(|e| panic!("{cmd} {p}: {e}"))
}
fn shape_xml(d: &Document, page: usize, id: u32) -> Vec<u8> {
    let saved = misterdiagram_vsdx::import(&misterdiagram_vsdx::write(d).unwrap().bytes).unwrap();
    let xml = std::str::from_utf8(&saved.parts[&saved.pages[page].part].bytes).unwrap();
    let marker = format!("ID=\"{id}\"");
    let start = xml.match_indices("<Shape ").find(|(i, _)| xml[*i..].split('>').next().unwrap().split_whitespace().any(|a| a == marker)).unwrap().0;
    let end = start + xml[start..].find("</Shape>").unwrap() + "</Shape>".len();
    xml.as_bytes()[start..end].to_vec()
}
fn move_to(s: &mut Session, id: u32, x: f64, y: f64) {
    run(s, "shape.move", json!({"id":id,"x":x,"y":y}));
}
#[test]
fn labels_far_placeables_and_moved_away_obstacles_keep_exact_saved_shapes() {
    let mut o = Opened::new(&drawing(), "labels");
    let ids = [1, 2, 3, 4, 30, 31, 42];
    let before = ids.map(|id| shape_xml(o.s.document(), 0, id));
    for (id, x, y) in [(21, 2., 4.), (22, 2., 4.), (20, 18., 18.)] {
        move_to(&mut o.s, id, x, y);
        for (id, bytes) in ids.iter().zip(&before) {
            assert_eq!(shape_xml(o.s.document(), 0, *id), *bytes, "shape {id} changed after moving {id}");
        }
    }
    move_to(&mut o.s, 20, 2., 4.);
    assert_ne!(shape_xml(o.s.document(), 0, 1), before[0]);
    for (id, bytes) in ids.iter().zip(&before).skip(1) {
        assert_eq!(shape_xml(o.s.document(), 0, *id), *bytes, "shape {id}");
    }
    let routed = shape_xml(o.s.document(), 0, 1);
    move_to(&mut o.s, 20, 18., 18.);
    assert_eq!(shape_xml(o.s.document(), 0, 1), routed);
}
#[test]
fn crossover_selects_only_free_policy_and_glued_targets_follow() {
    for (id, y) in [(1, 4.), (2, 8.), (3, 12.), (4, 16.)] {
        let mut o = Opened::new(&drawing(), &format!("policy{id}"));
        let before = shape_xml(o.s.document(), 0, id);
        let jump = shape_xml(o.s.document(), 0, 30);
        move_to(&mut o.s, 20, 2., y);
        assert_eq!(shape_xml(o.s.document(), 0, id) == before, id != 1, "policy {}", id - 1);
        assert_eq!(shape_xml(o.s.document(), 0, 30), jump);
    }
    let mut o = Opened::new(&drawing(), "glued");
    let before = shape_xml(o.s.document(), 0, 42);
    let other = shape_xml(o.s.document(), 0, 1);
    move_to(&mut o.s, 40, 2., 26.);
    assert_ne!(shape_xml(o.s.document(), 0, 42), before);
    assert_eq!(shape_xml(o.s.document(), 0, 1), other);
}
#[test]
fn endpoint_edits_update_only_changed_crossing_neighbors() {
    let mut o = Opened::new(&drawing(), "crossing");
    let unchanged = shape_xml(o.s.document(), 0, 1);
    let h = shape_xml(o.s.document(), 0, 30);
    run(
        &mut o.s,
        "engine.batch",
        json!({"commands":[
            {"command":"sheet.set","params":{"id":31,"name":"BeginX","value":18}},
            {"command":"sheet.set","params":{"id":31,"name":"EndX","value":18}}
        ]}),
    );
    let no_jump = shape_xml(o.s.document(), 0, 30);
    assert_ne!(no_jump, h);
    assert!(!String::from_utf8_lossy(&no_jump).contains("ArcTo"));
    assert_eq!(shape_xml(o.s.document(), 0, 1), unchanged);
    run(
        &mut o.s,
        "engine.batch",
        json!({"commands":[
            {"command":"sheet.set","params":{"id":31,"name":"BeginX","value":12}},
            {"command":"sheet.set","params":{"id":31,"name":"EndX","value":12}}
        ]}),
    );
    assert!(String::from_utf8_lossy(&shape_xml(o.s.document(), 0, 30)).contains("ArcTo"));
}
#[test]
fn undo_redo_failed_batch_and_second_edit_keep_scope_local() {
    let mut o = Opened::new(&drawing(), "history");
    let before = o.s.document().clone();
    move_to(&mut o.s, 20, 2., 4.);
    let moved = o.s.document().clone();
    run(&mut o.s, "edit.undo", json!({}));
    assert_eq!(o.s.document(), &before);
    run(&mut o.s, "edit.redo", json!({}));
    assert_eq!(o.s.document(), &moved);
    let error = o.s.execute(
        "engine.batch",
        &json!({"commands":[
            {"command":"shape.move","params":{"id":40,"x":2,"y":27}},
            {"command":"shape.move","params":{"id":99999,"x":2,"y":4}}
        ]}),
    );
    assert!(error.is_err());
    assert_eq!(o.s.document(), &moved);
    let untouched = shape_xml(o.s.document(), 0, 42);
    move_to(&mut o.s, 21, 4., 5.);
    assert_eq!(shape_xml(o.s.document(), 0, 42), untouched);
}
#[test]
fn page_policy_explicit_request_and_background_ids_are_isolated() {
    let mut d = drawing();
    let mut page = d.pages[0].clone();
    page.id = 1;
    page.name = "Background".into();
    page.name_u = "Background".into();
    page.part = String::new();
    page.attributes.insert("Background".into(), "1".into());
    d.pages.push(page);
    let mut o = Opened::new(&d, "pages");
    let second = shape_xml(o.s.document(), 1, 1);
    let before = shape_xml(o.s.document(), 0, 1);
    assert_eq!(run(&mut o.s, "route.reroute", json!({"page":1,"id":1}))["routed"], 1);
    assert_ne!(shape_xml(o.s.document(), 0, 1), before);
    assert_eq!(shape_xml(o.s.document(), 1, 1), second);
    run(&mut o.s, "sheet.set", json!({"scope":"page","page":1,"name":"RouteStyle","value":2}));
    assert_eq!(shape_xml(o.s.document(), 1, 1), second);
}

#[test]
fn group_placeables_and_glued_descendants_use_page_space_bounds() {
    let mut d = drawing();
    let mover = d.pages[0].shapes.iter_mut().find(|s| s.id == 20).unwrap();
    mover.shape_type = Some("Group".into());
    set(&mut mover.sheet, "ObjType", 8.);
    let mut child = skeleton::rectangle(23, 0., 0., 0.5, 0.5, "").unwrap();
    set(&mut child.sheet, "ObjType", 1.);
    mover.shapes.push(child);
    let mut o = Opened::new(&d, "group-placement");
    let before = shape_xml(o.s.document(), 0, 1);
    let jump = shape_xml(o.s.document(), 0, 30);
    move_to(&mut o.s, 20, 2., 4.);
    assert_ne!(shape_xml(o.s.document(), 0, 1), before);
    assert_eq!(shape_xml(o.s.document(), 0, 30), jump);
    let target_index = d.pages[0].shapes.iter().position(|s| s.id == 40).unwrap();
    let child = d.pages[0].shapes.remove(target_index);
    let mut group = skeleton::rectangle(80, 0., 0., 30., 30., "").unwrap();
    group.shape_type = Some("Group".into());
    set(&mut group.sheet, "ObjType", 8.);
    group.shapes.push(child);
    d.pages[0].shapes.push(group);
    let mut o = Opened::new(&d, "group-glue");
    let before = shape_xml(o.s.document(), 0, 42);
    let master = o.s.document().masters.clone();
    move_to(&mut o.s, 80, 16., 16.);
    assert_ne!(shape_xml(o.s.document(), 0, 42), before);
    assert_eq!(o.s.document().masters, master);
}
#[test]
fn unchanged_crossing_and_unrequested_shared_trunk_are_immutable_context() {
    let mut d = drawing();
    let first = d.pages[0].shapes.iter().find(|s| s.id == 1).unwrap().clone();
    let second = d.pages[0].shapes.iter_mut().find(|s| s.id == 2).unwrap();
    *second = first;
    second.id = 2;
    let mut o = Opened::new(&d, "lanes");
    let neighbor = shape_xml(o.s.document(), 0, 2);
    let crossing = shape_xml(o.s.document(), 0, 30);
    run(&mut o.s, "sheet.set", json!({"id":1,"name":"EndX","value":8}));
    assert_eq!(shape_xml(o.s.document(), 0, 2), neighbor);
    run(&mut o.s, "route.reroute", json!({"id":31}));
    assert_eq!(shape_xml(o.s.document(), 0, 30), crossing);
}

#[test]
fn crossover_uses_the_selected_router_clearance() {
    for style in [1., 17.] {
        let mut d = drawing();
        set(&mut d.pages[0].shapes[0].sheet, "ShapeRouteStyle", style);
        let mut o = Opened::new(&d, &format!("clearance-{style}"));
        let before = shape_xml(o.s.document(), 0, 1);
        // New bounds stop 0.15 inch above the route. Only the inflated ordinary
        // orthogonal clearance reaches it; SimpleNS uses zero inflation.
        move_to(&mut o.s, 20, 2., 4.4);
        assert_eq!(shape_xml(o.s.document(), 0, 1) == before, style == 17.);
        if style == 17. {
            move_to(&mut o.s, 20, 2., 4.2);
            assert_ne!(shape_xml(o.s.document(), 0, 1), before);
        }
    }
}

#[test]
fn indirect_endpoint_references_see_new_glue_points_and_triggers() {
    for name in ["BeginX", "BegTrigger"] {
        let mut d = drawing();
        let endpoint = d.pages[0].shapes[0].sheet.cells.iter_mut().find(|c| c.name == "EndX").unwrap();
        endpoint.formula = Some(format!("Sheet.42!{name}"));
        let mut o = Opened::new(&d, &format!("indirect-{name}"));
        let before = shape_xml(o.s.document(), 0, 1);
        let untouched = shape_xml(o.s.document(), 0, 2);
        move_to(&mut o.s, 40, 4., 26.);
        assert_ne!(shape_xml(o.s.document(), 0, 1), before, "indirect {name}");
        assert_eq!(shape_xml(o.s.document(), 0, 2), untouched);
    }
}
