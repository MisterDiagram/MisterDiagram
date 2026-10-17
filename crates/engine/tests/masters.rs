//! Master-drop contracts against original synthetic and local Visio witnesses.
use misterdiagram_engine::Session;
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};
fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus")
}
fn run(s: &mut Session, id: &str, p: Value) -> Value {
    s.execute(id, &p).unwrap_or_else(|e| panic!("{id} {p}: {e}"))
}
fn cells(d: &misterdiagram_model::Document, id: u32) -> BTreeMap<String, (Option<String>, Option<String>, Option<String>)> {
    Evaluator::new(d)
        .unwrap()
        .source_named_cells(Scope::PageShape { page: 0, shape: id })
        .unwrap()
        .into_iter()
        .map(|n| (n.name, (n.cell.value.clone(), n.cell.formula.clone(), n.cell.units.clone())))
        .collect()
}
#[test]
fn basic_rectangle_drop_has_exact_local_and_inherited_cell_set() {
    let stencil = root().join("ms-shipped/1033/BASIC_U.vssx");
    let witness = root().join("generated/master/basic-rect.vsdx");
    if !stencil.exists() || !witness.exists() {
        eprintln!("skip: private BASIC_U witness absent");
        return;
    }
    let expected = misterdiagram_vsdx::read(witness).unwrap();
    let mut s = Session::new();
    run(&mut s, "stencil.open", json!({"path":stencil}));
    let search = run(&mut s, "stencil.search", json!({"stencil":1,"query":"rEcTaNgLe"}));
    assert!(search["matched"].as_u64().unwrap() > 0);
    let list = run(&mut s, "stencil.masters", json!({"stencil":1}));
    assert!(list["masters"].as_array().unwrap().iter().any(|m| m["name_u"] == "Rectangle" && m["width"] == 1.5 && m["height"] == 1.));
    run(&mut s, "master.drop", json!({"stencil":1,"master":"Rectangle","x":2,"y":6}));
    run(&mut s, "shape.resize", json!({"id":1,"width":3,"height":2}));
    let expected = cells(&expected, 1);
    let actual = cells(s.document(), 1);
    assert_eq!(actual.keys().collect::<Vec<_>>(), expected.keys().collect::<Vec<_>>());
    for (name, (v, f, u)) in &expected {
        let (av, af, au) = &actual[name];
        assert_eq!(af, f, "{name} formula");
        assert_eq!(au, u, "{name} units");
        match (v.as_deref().and_then(|v| v.parse::<f64>().ok()), av.as_deref().and_then(|v| v.parse::<f64>().ok())) {
            (Some(a), Some(b)) => assert!((a - b).abs() <= 1e-6, "{name}: expected {a}, actual {b}"),
            _ => assert_eq!(av, v, "{name}"),
        }
    }
    let count = s.document().masters.len();
    run(&mut s, "master.drop", json!({"stencil":1,"master":"Rectangle","x":5,"y":6}));
    assert_eq!(s.document().masters.len(), count, "same master copied twice");
}
#[test]
fn original_shape_becomes_a_master_and_instance_keeps_only_overrides() {
    let mut s = Session::new();
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":2,"width":2,"height":1}));
    run(&mut s, "shape.text", json!({"id":1,"text":"Original master"}));
    let master = run(&mut s, "master.new_from_shape", json!({"name":"Original block"}))["master"].as_u64().unwrap();
    run(&mut s, "master.drop", json!({"master":master,"x":5,"y":4}));
    let shape = &s.document().pages[0].shapes[1];
    assert_eq!(shape.master, Some(master as u32));
    assert!(shape.sheet.cells.iter().filter(|c| c.formula.as_deref() != Some("Inh")).all(|c| matches!(c.name.as_str(), "PinX" | "PinY")));
    assert!(shape.text.is_none());
    let master = s.document().masters.iter().find(|m| u64::from(m.id) == master).unwrap();
    assert!(master.shapes[0].id >= 5);
    assert!(master.unique_id.as_ref().unwrap().ends_with("-8E40-00608CF305B2}"));
    run(&mut s, "edit.undo", json!({}));
    assert_eq!(s.document().pages[0].shapes.len(), 1);
    run(&mut s, "edit.undo", json!({}));
    assert_eq!(s.document().masters.len(), 1);
}
#[test]
fn ten_masters_from_flowchart_and_network_stencils_roundtrip() {
    let flow = root().join("ms-shipped/1033/BASFLO_U.vssx");
    let network = root().join("ms-shipped/1033/NETSME_U.vssx");
    if !flow.exists() || !network.exists() {
        eprintln!("skip: private shipped stencils absent");
        return;
    }
    let mut s = Session::new();
    run(&mut s, "stencil.open", json!({"path":flow}));
    run(&mut s, "stencil.open", json!({"path":network}));
    for (stencil, ids) in [(1, [2, 4, 5, 6, 7]), (2, [2, 5, 7, 10, 12])] {
        let list = run(&mut s, "stencil.masters", json!({"stencil":stencil}));
        let available = list["masters"].as_array().unwrap();
        for (i, id) in ids.into_iter().enumerate() {
            assert!(available.iter().any(|m| m["id"] == id), "missing network master {id}");
            run(&mut s, "master.drop", json!({"stencil":stencil,"master":id,"x":1.5+i as f64*1.5,"y":if stencil==1{8.}else{3.}}));
        }
    }
    assert_eq!(s.document().pages[0].shapes.len(), 10);
    let written = misterdiagram_vsdx::write(s.document()).unwrap();
    let reopened = misterdiagram_vsdx::import(&written.bytes).unwrap();
    assert_eq!(reopened.pages[0].shapes.len(), 10);
    let out = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/engine-tests/ten-masters.vsdx");
    std::fs::create_dir_all(out.parent().unwrap()).unwrap();
    std::fs::write(out, written.bytes).unwrap();
}
