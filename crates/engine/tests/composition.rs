//! Regressions from the P4a independent command-composition discovery pass.
use diagramcraft_engine::Session;
use diagramcraft_vsdx::evaluate::{Evaluator, Scope};
use serde_json::{Value, json};
fn run(s: &mut Session, id: &str, p: Value) -> Value {
    s.execute(id, &p).unwrap_or_else(|e| panic!("{id} {p}: {e}"))
}
fn number(s: &Session, id: u32, name: &str) -> f64 {
    Evaluator::new(s.document()).unwrap().evaluate(Scope::PageShape { page: s.document().pages[0].id, shape: id }, name).numeric().unwrap()
}
fn near(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-6, "{a} != {b}");
}
#[test]
fn deleting_a_static_target_detaches_endpoints_and_keeps_export_valid() {
    let mut s = Session::new();
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":1,"width":2,"height":2}));
    run(&mut s, "shape.connection.add", json!({"id":1,"x":1,"y":1}));
    run(&mut s, "shape.draw.line", json!({"x1":2,"y1":2,"x2":6,"y2":2}));
    run(&mut s, "connect.static", json!({"id":2,"end":"begin","target":1,"point":"Connections.X1"}));
    run(&mut s, "shape.delete", json!({"id":1}));
    near(number(&s, 2, "BeginX"), 2.);
    near(number(&s, 2, "BeginY"), 2.);
    assert!(s.document().pages[0].connects.is_empty());
    let out = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/engine-tests/deleted-target.svg");
    std::fs::create_dir_all(out.parent().unwrap()).unwrap();
    run(&mut s, "file.export", json!({"path":out}));
    assert!(std::fs::read_to_string(out).unwrap().contains("<svg"));
    run(&mut s, "edit.undo", json!({}));
    assert_eq!(s.document().pages[0].connects.len(), 1);
}
#[test]
fn boolean_transform_cells_work_with_duplicate_group_and_flip() {
    let mut s = Session::new();
    run(&mut s, "master.drop", json!({"master":2,"x":5,"y":2}));
    run(&mut s, "shape.duplicate", json!({"id":1}));
    run(&mut s, "shape.flip", json!({"id":1,"axis":"x"}));
    near(number(&s, 1, "FlipX"), 1.);
    run(&mut s, "arrange.align", json!({"ids":[1,2],"mode":"left"}));
    run(&mut s, "shape.group", json!({"ids":[1,2]}));
    assert_eq!(s.document().pages[0].shapes.len(), 1);
}
#[test]
fn moving_free_end_rewalks_dynamic_begin() {
    let mut s = Session::new();
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":1,"width":2,"height":2}));
    run(&mut s, "master.drop", json!({"master":2,"x":5,"y":2}));
    run(&mut s, "connect.dynamic", json!({"id":2,"end":"begin","target":1}));
    run(&mut s, "sheet.set", json!({"id":2,"name":"EndX","value":2,"force":true}));
    run(&mut s, "sheet.set", json!({"id":2,"name":"EndY","value":8,"force":true}));
    near(number(&s, 2, "BeginX"), 2.);
    near(number(&s, 2, "BeginY"), 3.);
}
#[test]
fn grouped_glue_retains_formulas_and_uses_parent_coordinates() {
    let mut s = Session::new();
    for x in [1, 6] {
        run(&mut s, "shape.draw.rect", json!({"x":x,"y":1,"width":2,"height":2}));
    }
    run(&mut s, "shape.draw.line", json!({"x1":3,"y1":2,"x2":6,"y2":2}));
    for (end, target) in [("begin", 1), ("end", 2)] {
        run(&mut s, "connect.dynamic", json!({"id":3,"end":end,"target":target}));
    }
    run(&mut s, "shape.group", json!({"ids":[1,2,3]}));
    near(number(&s, 3, "EndX"), 5.);
    run(&mut s, "shape.move", json!({"id":2,"x":8,"y":1}));
    near(number(&s, 3, "EndX"), 7.);
    run(&mut s, "connect.dynamic", json!({"id":3,"end":"end","target":2}));
    near(number(&s, 3, "EndX"), 7.);
    run(&mut s, "shape.ungroup", json!({"id":4}));
    near(number(&s, 3, "EndX"), 8.);
    run(&mut s, "shape.move", json!({"id":2,"x":10,"y":2}));
    near(number(&s, 3, "EndX"), 9.);
}
#[test]
fn same_size_keeps_line_endpoints_consistent() {
    let mut s = Session::new();
    run(&mut s, "shape.draw.line", json!({"x1":0,"y1":0,"x2":4,"y2":0}));
    run(&mut s, "shape.draw.line", json!({"x1":0,"y1":2,"x2":2,"y2":2}));
    run(&mut s, "arrange.same_size", json!({"ids":[1,2],"axis":"width"}));
    near(number(&s, 2, "Width"), 4.);
    near(number(&s, 2, "EndX") - number(&s, 2, "BeginX"), 4.);
    run(&mut s, "shape.move", json!({"id":2,"x":5,"y":5}));
    near(number(&s, 2, "EndX") - number(&s, 2, "BeginX"), 4.);
}
#[test]
fn deleting_earlier_page_preserves_current_page_identity() {
    let mut s = Session::new();
    run(&mut s, "page.add", json!({"name":"B"}));
    run(&mut s, "page.add", json!({"name":"C"}));
    run(&mut s, "shape.draw.rect", json!({"page":2,"x":1,"y":1,"width":2,"height":2}));
    run(&mut s, "page.delete", json!({"page":1}));
    assert_eq!(run(&mut s, "shape.draw.rect", json!({"x":1,"y":1,"width":2,"height":2}))["id"], 2);
    assert_eq!(s.document().pages[0].name, "B");
    assert_eq!(s.document().pages[0].shapes.len(), 2);
    assert!(s.document().pages[1].shapes.is_empty());
}

/// Replacing an inherited theme sentinel must produce a persistent explicit color.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.cell.formulau
#[test]
fn explicit_color_replaces_theme_sentinel_and_registers_palette() {
    let mut s = Session::new();
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":1,"width":2,"height":1}));
    run(&mut s, "sheet.set", json!({"id":1,"name":"FillForegnd","formula":"RGB(204,0,0)"}));
    let color = s.document().pages[0].shapes[0].sheet.cell("FillForegnd").unwrap();
    assert_eq!(color.value.as_deref(), Some("#cc0000"));
    assert_eq!(color.formula, None);
    assert_eq!(s.document().colors.get(&26).map(String::as_str), Some("#CC0000"));
    run(&mut s, "edit.undo", json!({}));
    assert!(!s.document().colors.values().any(|v| v.eq_ignore_ascii_case("#cc0000")));
}

/// The imported app.xml part must track later page and master commands too.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx
#[test]
fn incremental_authoring_refreshes_summary_names() {
    let mut s = Session::new();
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":1,"width":2,"height":1}));
    run(&mut s, "master.new_from_shape", json!({"name":"Original block"}));
    run(&mut s, "page.add", json!({"name":"Second page"}));
    let xml = s.document().parts["docProps/app.xml"].xml.as_ref().unwrap();
    let titles = xml.elements().find(|e| e.name == "TitlesOfParts").unwrap();
    let vector = titles.elements().next().unwrap();
    assert_eq!(vector.attributes.get("size").map(String::as_str), Some("4"));
    let names = vector.elements().map(|e| e.text()).collect::<Vec<_>>();
    assert_eq!(names, vec!["Page-1", "Second page", "Dynamic connector", "Original block"]);
}
