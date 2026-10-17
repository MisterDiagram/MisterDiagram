//! Contributor-original layout cases and private Visio side-glue witness.
use misterdiagram_engine::Session;
use serde_json::{Value, json};
fn run(s: &mut Session, id: &str, p: Value) -> Value {
    s.execute(id, &p).unwrap_or_else(|e| panic!("{id} {p}: {e}"))
}
fn pair() -> Session {
    let mut s = Session::new();
    run(&mut s, "file.new", json!({}));
    for (x, text) in [(1, "A"), (5, "B")] {
        let id = run(&mut s, "shape.draw.rect", json!({"x":x,"y":6,"width":1.5,"height":1.5}))["id"].as_u64().unwrap();
        run(&mut s, "shape.text", json!({"id":id,"text":text}));
    }
    run(&mut s, "master.drop", json!({"master":2,"x":0,"y":0}));
    s
}
#[test]
fn facing_sides_use_static_points_reuse_rows_and_reject_hostile_requests() {
    let mut s = pair();
    for p in [json!({}), json!({"id":3,"source":1,"target":1}), json!({"id":3,"source":1,"target":999}), json!({"id":3,"source":-1,"target":2})] {
        let before = s.document().clone();
        assert!(s.execute("connect.sides", &p).is_err());
        assert_eq!(s.document(), &before);
    }
    run(&mut s, "connect.sides", json!({"id":3,"source":1,"target":2}));
    let before = s.document().pages[0].shapes[0].sheet.sections.iter().find(|s| s.name == "Connection").unwrap().rows.len();
    run(&mut s, "connect.sides", json!({"id":3,"source":1,"target":2}));
    assert_eq!(s.document().pages[0].shapes[0].sheet.sections.iter().find(|s| s.name == "Connection").unwrap().rows.len(), before);
    assert_eq!(s.document().pages[0].connects.len(), 2);
    for c in &s.document().pages[0].connects {
        assert_eq!(c.to_cell.as_deref(), Some("Connections.X1"));
    }
    assert_eq!(run(&mut s, "sheet.get", json!({"id":3,"name":"BeginX"}))["value"], "2.5");
    assert_eq!(run(&mut s, "sheet.get", json!({"id":3,"name":"EndX"}))["value"], "5");
}
#[test]
fn side_glue_rebuild_matches_original_dump_and_route() {
    use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let witness = root.join("corpus/p5a/original/route/side-glue.vsdx");
    if !witness.exists() {
        eprintln!("private P5a side-glue witness absent; skipped");
        return;
    }
    let mut s = pair();
    run(&mut s, "connect.sides", json!({"id":3,"source":1,"target":2}));
    let dir = root.join("target/side-glue-test");
    std::fs::create_dir_all(&dir).unwrap();
    run(&mut s, "file.save", json!({"path":dir.join("side-glue.vsdx")}));
    let original = misterdiagram_vsdx::read(witness).unwrap();
    let mut a = Evaluator::new(s.document()).unwrap();
    let mut b = Evaluator::new(&original).unwrap();
    let scope = Scope::PageShape { page: original.pages[0].id, shape: 3 };
    let names = b
        .resolved_named_cells(scope)
        .into_iter()
        .chain(a.resolved_named_cells(scope))
        .map(|c| c.name)
        .filter(|n| {
            n.starts_with("Geometry")
                || ["BeginX", "BeginY", "EndX", "EndY", "PinX", "PinY", "Width", "Height", "LocPinX", "LocPinY", "Angle"].contains(&n.as_str())
        })
        .collect::<std::collections::BTreeSet<_>>();
    for name in names {
        let x = a.evaluate(scope, &name);
        let y = b.evaluate(scope, &name);
        match (x.numeric(), y.numeric()) {
            (Ok(x), Ok(y)) => assert!((x - y).abs() <= 1e-6, "route {name}: {x} != {y}"),
            _ => assert_eq!(x, y, "route {name}"),
        };
    }
    for id in [1, 2] {
        let our = &s.document().pages[0].shapes.iter().find(|s| s.id == id).unwrap().sheet;
        let their = &original.pages[0].shapes.iter().find(|s| s.id == id).unwrap().sheet;
        let rows = |sheet: &misterdiagram_model::Sheet| {
            sheet
                .sections
                .iter()
                .filter(|s| s.name == "Connection")
                .flat_map(|s| &s.rows)
                .map(|r| r.cells.iter().map(|c| (c.name.clone(), c.value.clone(), c.formula.clone(), c.units.clone())).collect::<Vec<_>>())
                .collect::<Vec<_>>()
        };
        assert_eq!(rows(our), rows(their), "canonical Connection rows");
    }
    let mut our = s.document().pages[0].connects.clone();
    let mut their = original.pages[0].connects.clone();
    our.sort_by_key(|c| c.from_cell.clone());
    their.sort_by_key(|c| c.from_cell.clone());
    assert_eq!(
        our.iter().map(|c| (&c.from_cell, &c.to_cell, c.to_sheet, c.to_part)).collect::<Vec<_>>(),
        their.iter().map(|c| (&c.from_cell, &c.to_cell, c.to_sheet, c.to_part)).collect::<Vec<_>>(),
        "static side-glue Connects must match Visio"
    );
    let theme = misterdiagram_theme::Themes::from_document(s.document()).unwrap();
    let mut dump = Vec::new();
    misterdiagram_vsdx::dump::dump_with_hosts(
        s.document(),
        misterdiagram_vsdx::dump::Selection::All,
        Some(&theme),
        Some(&misterdiagram_text::Host),
        &mut dump,
    )
    .unwrap();
    std::fs::write(dir.join("side-glue.sheet.tsv"), dump).unwrap();
    let result = std::process::Command::new(if cfg!(windows) { "python" } else { "python3" })
        .arg(root.join("tools/oracle/diff_dump.py"))
        .arg(dir.join("side-glue.sheet.tsv"))
        .arg(root.join("corpus/p5a/original/route/side-glue.sheet.tsv"))
        .arg("--ignore-hooks")
        .arg(root.join("crates/sheet/tests/hooks.txt"))
        .arg("--ignore-volatile")
        .arg(root.join("crates/sheet/tests/volatile.txt"))
        .output()
        .unwrap();
    println!("{}", String::from_utf8_lossy(&result.stdout));
    assert!(
        result.status.success(),
        "side-glue rebuild diff: {}{}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
}

#[test]
fn rows_are_centered_ordered_and_return_exact_bounds() {
    let mut s = Session::new();
    for _ in 0..3 {
        run(&mut s, "shape.draw.rect", json!({"x":0,"y":0,"width":2,"height":1}));
    }
    let value =
        run(&mut s, "layout.rows", json!({"ids":[1,2,3],"counts":[2,1],"region":{"x":0,"y":0,"width":10,"height":10},"gutter_x":1,"gutter_y":1}));
    for (i, (x, y)) in [(2.5, 5.5), (5.5, 5.5), (4., 3.5)].into_iter().enumerate() {
        assert_eq!(value["placed"][i]["bounds"], json!({"x":x,"y":y,"width":2.,"height":1.}));
        let actual = run(&mut s, "shape.inspect", json!({"id":i+1}));
        assert_eq!(actual["bounds"], value["placed"][i]["bounds"]);
    }
    let default = run(&mut s, "layout.rows", json!({"ids":[1,2,3],"counts":[2,1]}));
    let get = |s: &mut Session, name| run(s, "sheet.get", json!({"scope":"page","name":name}))["value"].as_str().unwrap().parse::<f64>().unwrap();
    assert_eq!(default["gutter_x"].as_f64().unwrap(), get(&mut s, "BlockSizeX").max(2. * get(&mut s, "LineToNodeX")));
    let before = s.document().clone();
    for params in [
        json!({"ids":[1,2],"counts":[1]}),
        json!({"ids":[1,1],"counts":[2]}),
        json!({"ids":[1],"counts":[0]}),
        json!({"ids":[1],"counts":[1],"gutter_x":-1}),
        json!({"ids":[1],"counts":[1],"region":{"x":0,"y":0,"width":0.1,"height":0.1}}),
        json!({"ids":[1],"counts":[true]}),
    ] {
        assert!(s.execute("layout.rows", &params).is_err());
        assert_eq!(s.document(), &before);
    }
}
#[test]
fn fan_assigns_distinct_hub_points_in_spoke_order_and_routes_clear() {
    let mut s = Session::new();
    run(&mut s, "shape.draw.rect", json!({"x":4,"y":6,"width":4,"height":1}));
    for x in [2, 5, 8] {
        run(&mut s, "shape.draw.rect", json!({"x":x,"y":1,"width":1,"height":1}));
    }
    for spoke in 2..=4 {
        let id = run(&mut s, "master.drop", json!({"master":2,"x":0,"y":0}))["id"].clone();
        run(&mut s, "connect.sides", json!({"id":id,"source":1,"target":spoke}));
    }
    let result = run(&mut s, "layout.fan", json!({"hub":1,"spokes":[4,2,3]}));
    let mut points = std::collections::BTreeSet::new();
    for (i, spoke) in [4, 2, 3].into_iter().enumerate() {
        let assignment = &result["assignments"][i];
        assert_eq!(assignment["spoke"], spoke);
        let name = assignment["point"].as_str().unwrap();
        assert!(points.insert(name.to_string()), "fan must assign each spoke a distinct hub point");
        let x = run(&mut s, "sheet.get", json!({"id":1,"name":name}))["value"].as_str().unwrap().parse::<f64>().unwrap();
        assert!((x - (i + 1) as f64).abs() < 1e-9, "hub point order: {x}");
    }
    assert_eq!(result["crossings"], json!([]));
    let before = s.document().clone();
    for p in [json!({"hub":1,"spokes":[2,2]}), json!({"hub":1,"spokes":[1]}), json!({"hub":1,"spokes":[999]}), json!({"hub":1,"spokes":[false]})] {
        assert!(s.execute("layout.fan", &p).is_err());
        assert_eq!(s.document(), &before);
    }
}

#[test]
fn rows_reject_position_dependent_bounds_and_restore_the_session() {
    let mut s = Session::new();
    run(&mut s, "shape.draw.rect", json!({"x":0.5,"y":1,"width":1,"height":1}));
    run(&mut s, "sheet.set", json!({"id":1,"name":"Width","formula":"PinX"}));
    let before = s.document().clone();
    let error = s.execute("layout.rows", &json!({"ids":[1],"counts":[1],"region":{"x":0,"y":0,"width":10,"height":10}})).unwrap_err();
    assert!(error.to_string().contains("position-dependent shape bounds"), "{error}");
    assert_eq!(s.document(), &before);
}
#[test]
fn tiny_hub_fan_points_remain_distinct_and_at_the_requested_fractions() {
    let mut s = Session::new();
    run(&mut s, "shape.draw.rect", json!({"x":0,"y":6,"width":1e-10,"height":1}));
    for y in [0, 2] {
        run(&mut s, "shape.draw.rect", json!({"x":-0.5+5e-11,"y":y,"width":1,"height":1}));
    }
    for spoke in 2..=3 {
        let id = run(&mut s, "master.drop", json!({"master":2,"x":0,"y":0}))["id"].clone();
        run(&mut s, "connect.sides", json!({"id":id,"source":1,"target":spoke}));
    }
    let result = run(&mut s, "layout.fan", json!({"hub":1,"spokes":[2,3]}));
    assert_ne!(result["assignments"][0]["point"], result["assignments"][1]["point"], "tiny hub points must remain distinct");
    for (index, assignment) in result["assignments"].as_array().unwrap().iter().enumerate() {
        let x = run(&mut s, "sheet.get", json!({"id":1,"name":assignment["point"]}))["value"].as_str().unwrap().parse::<f64>().unwrap();
        assert!((x / 1e-10 - (index + 1) as f64 / 3.).abs() < 1e-9, "tiny hub fraction: {x}");
    }
}

/// Text-metric geometry uses the same hosts as normal command evaluation.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/textwidth-function
#[test]
fn rows_and_side_points_support_text_sized_shapes() {
    let mut s = Session::new();
    for text in ["Cedar Grove", "Maple"] {
        let id = run(&mut s, "shape.draw.rect", json!({"x":0,"y":0,"width":2,"height":1}))["id"].clone();
        run(&mut s, "shape.text", json!({"id":id,"text":text}));
        run(&mut s, "sheet.set", json!({"id":id,"name":"Width","formula":"TEXTWIDTH(TheText)"}));
    }
    let placed = run(&mut s, "layout.rows", json!({"ids":[1,2],"counts":[2]}));
    assert!(placed["placed"][0]["bounds"]["width"].as_f64().unwrap() > 0.);
    run(&mut s, "master.drop", json!({"master":2,"x":0,"y":0}));
    run(&mut s, "connect.sides", json!({"id":3,"source":1,"target":2}));
    let fan = run(&mut s, "layout.fan", json!({"hub":1,"spokes":[2]}));
    assert_eq!(fan["crossings"], json!([]));
}
