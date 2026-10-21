//! Existing automatic ports retain their master formulas and prompts (#67).
//! https://learn.microsoft.com/en-us/office/client-developer/visio/connection-points-section
use misterdiagram_engine::Session;
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
use serde_json::{Value, json};
fn run(s: &mut Session, cmd: &str, p: Value) -> Value {
    s.execute(cmd, &p).unwrap_or_else(|e| panic!("{cmd} {p}: {e}"))
}
fn points(s: &Session, id: u32) -> Vec<(String, f64, f64)> {
    let themes = misterdiagram_theme::Themes::from_document(s.document()).unwrap();
    let mut e = Evaluator::new(s.document()).unwrap().with_theme_host(&themes).with_text_host(&misterdiagram_text::Host);
    let scope = Scope::PageShape { page: s.document().pages[0].id, shape: id };
    let all = e.resolved_named_cells(scope);
    let mut out = vec![];
    for x in all.iter().filter(|n| n.cell.name == "X" && n.section.is_some_and(|s| s.name == "Connection")) {
        let y = all
            .iter()
            .find(|n| {
                n.cell.name == "Y"
                    && n.section.is_some_and(|s| s.name == "Connection")
                    && n.row.zip(x.row).is_some_and(|(a, b)| a.index == b.index && a.name == b.name)
            })
            .unwrap();
        out.push((x.name.clone(), e.evaluate(scope, &x.name).numeric().unwrap(), e.evaluate(scope, &y.name).numeric().unwrap()));
    }
    out
}
fn unique(s: &Session) {
    for shape in &s.document().pages[0].shapes {
        let p = points(s, shape.id);
        for (i, a) in p.iter().enumerate() {
            for b in p.iter().skip(i + 1) {
                assert!((a.1 - b.1).abs() > 1e-6 || (a.2 - b.2).abs() > 1e-6, "shape {} duplicate {:?} {:?}", shape.id, a, b);
            }
        }
    }
}
#[test]
fn automatic_connectors_reuse_existing_master_rows_and_layout_ports() {
    let mut s = Session::new();
    run(&mut s, "stencil.open", json!({"builtin":"network"}));
    let a = run(&mut s, "master.drop", json!({"master":"compute.server","x":4,"y":6}))["id"].as_u64().unwrap() as u32;
    let before = points(&s, a);
    assert_eq!(before.len(), 16);
    let mut ids = vec![];
    for (x, y) in [(1., 2.), (5., 2.), (3., 0.2)] {
        let b = run(&mut s, "shape.draw.rect", json!({"x":x,"y":y,"width":2,"height":1}))["id"].as_u64().unwrap() as u32;
        ids.push(b);
        run(&mut s, "connector.add", json!({"from":a,"to":b}));
        unique(&s);
    }
    assert_eq!(points(&s, a), before, "master ports must be reused without overrides");
    let cells = s.document().pages[0]
        .shapes
        .iter()
        .find(|v| v.id == a)
        .unwrap()
        .sheet
        .sections
        .iter()
        .filter(|v| v.name == "Connection")
        .cloned()
        .collect::<Vec<_>>();
    assert!(cells.is_empty(), "existing master connection formulas and prompts stay inherited");
    for _ in 0..2 {
        run(&mut s, "layout.fan", json!({"hub":a,"spokes":ids}));
        unique(&s);
    }
}
#[test]
fn repeated_automatic_links_never_duplicate_positions() {
    let mut s = Session::new();
    for (x, y) in [(1, 5), (1, 1), (5, 1)] {
        run(&mut s, "shape.draw.rect", json!({"x":x,"y":y,"width":2,"height":1}));
    }
    for _ in 0..3 {
        for to in [2, 3] {
            run(&mut s, "connector.add", json!({"from":1,"to":to}));
            unique(&s);
        }
    }
    for _ in 0..2 {
        run(&mut s, "layout.fan", json!({"hub":1,"spokes":[2,3]}));
        unique(&s);
    }
}

#[test]
fn coincident_outward_row_does_not_hide_an_inward_row() {
    for names in [["AOut", "ZIn"], ["ZOut", "AIn"]] {
        let mut s = Session::new();
        for x in [0, 4] {
            run(&mut s, "shape.draw.rect", json!({"x":x,"y":1,"width":1,"height":1}));
        }
        for (name, kind) in names.into_iter().zip([1, 0]) {
            run(&mut s, "shape.connection.add", json!({"id":1,"name":name,"x":1,"y":0.5,"type":kind}));
        }
        run(&mut s, "connector.add", json!({"from":1,"to":2}));
        let c = s.document().pages[0].connects.iter().find(|c| c.to_sheet == 1).unwrap();
        assert!(c.to_cell.as_deref().unwrap().contains(names[1]), "{:?}", c.to_cell);
        assert_eq!(points(&s, 1).len(), 2);
    }
}
