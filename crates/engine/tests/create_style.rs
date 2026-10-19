//! Convenience commands must serialize exactly like their fine-grained edits.
use misterdiagram_engine::Session;
use serde_json::{Value, json};
use std::{path::Path, process::Command};
fn run(s: &mut Session, command: &str, p: Value) -> Value {
    s.execute(command, &p).unwrap()
}
fn equal(a: &mut Session, b: &mut Session, name: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = root.join("target/create-style-tests").join(std::process::id().to_string());
    std::fs::create_dir_all(&out).unwrap();
    let left = out.join(format!("{name}-fine.vsdx"));
    let right = out.join(format!("{name}-compound.vsdx"));
    run(a, "file.save", json!({"path":left}));
    run(b, "file.save", json!({"path":right}));
    let result = Command::new(if cfg!(windows) { "python" } else { "python3" })
        .arg(root.join("tools/bench/compare.py"))
        .arg(left)
        .arg(right)
        .output()
        .unwrap();
    assert!(result.status.success(), "package equality: {}{}", String::from_utf8_lossy(&result.stdout), String::from_utf8_lossy(&result.stderr));
}
#[test]
fn create_style_matches_fine_grained_packages_and_is_one_undo_step() {
    for (name, command, base) in [
        ("rect", "shape.draw.rect", json!({"x":1,"y":2,"width":2,"height":1})),
        ("ellipse", "shape.draw.ellipse", json!({"x":1,"y":2,"width":2,"height":1})),
        ("line", "shape.draw.line", json!({"x1":1,"y1":2,"x2":3,"y2":4})),
        ("drop", "master.drop", json!({"master":"compute.server","x":3,"y":4})),
    ] {
        let mut a = Session::new();
        let mut b = Session::new();
        for s in [&mut a, &mut b] {
            run(s, "stencil.open", json!({"builtin":"network"}));
        }
        let mut styled = base.clone();
        styled["text"] = json!("$$literal label");
        styled["fill"] = json!({"color":"#A2B4C6"});
        styled["line"] = json!({"color":"#3C5164","weight":1.5,"units":"PT"});
        styled["font"] = json!({"font":"Arimo","size":12,"bold":true,"align":0});
        let id = run(&mut a, command, base)["id"].clone();
        for (key, cmd) in [("text", "shape.text"), ("fill", "format.fill"), ("line", "format.line"), ("font", "format.text")] {
            let mut p = if key == "text" { json!({"text":styled[key]}) } else { styled[key].clone() };
            p["id"] = id.clone();
            run(&mut a, cmd, p);
        }
        let history = b.journal().len();
        let result = run(&mut b, command, styled);
        assert_eq!(result["id"], id);
        assert_eq!(b.journal().len(), history + 1);
        equal(&mut a, &mut b, name);
        run(&mut b, "edit.undo", json!({}));
        assert!(b.document().pages[0].shapes.is_empty());
        run(&mut b, "edit.redo", json!({}));
        assert_eq!(b.document().pages[0].shapes.len(), 1);
    }
}
fn pair() -> Session {
    let mut s = Session::new();
    for x in [1, 5] {
        run(&mut s, "shape.draw.rect", json!({"x":x,"y":3,"width":2,"height":2}));
    }
    s
}
#[test]
fn connector_add_matches_both_fine_grained_glue_ends_and_line_style() {
    for mode in ["static", "dynamic", "points"] {
        let mut a = pair();
        let mut b = pair();
        if mode == "points" {
            for s in [&mut a, &mut b] {
                run(s, "shape.connection.add", json!({"id":1,"x":"Width","y":"Height/2"}));
                run(s, "shape.connection.add", json!({"id":2,"x":0,"y":"Height/2"}));
            }
        }
        let id = run(&mut a, "master.drop", json!({"master":"Dynamic connector","x":0,"y":0}))["id"].clone();
        let mut params = json!({"from":1,"to":2,"line":{"color":"#286FA3","weight":1.5,"units":"PT"}});
        if mode == "static" {
            run(&mut a, "connect.sides", json!({"id":id,"source":1,"target":2}));
        } else {
            let command = if mode == "dynamic" { "connect.dynamic" } else { "connect.static" };
            params["glue"] = json!(if mode == "dynamic" { "dynamic" } else { "static" });
            for (end, target) in [("begin", 1), ("end", 2)] {
                let mut p = json!({"id":id,"end":end,"target":target});
                if mode == "points" {
                    p["point"] = json!("Connections.X1");
                }
                run(&mut a, command, p);
            }
            if mode == "points" {
                params["from_point"] = json!("Connections.X1");
                params["to_point"] = json!("Connections.X1");
            }
        }
        run(&mut a, "format.line", json!({"id":id,"color":"#286FA3","weight":1.5,"units":"PT"}));
        assert_eq!(run(&mut b, "connector.add", params)["id"], id);
        assert_eq!(b.document().pages[0].connects.len(), 2);
        equal(&mut a, &mut b, mode);
        run(&mut b, "edit.undo", json!({}));
        assert_eq!(b.document().pages[0].shapes.len(), 2);
        assert!(b.document().pages[0].connects.is_empty());
    }
}
#[test]
fn automatic_parallel_links_and_fan_keep_three_distinct_points_at_both_ends() {
    for mode in ["static", "dynamic"] {
        let mut s = pair();
        let mut ids = vec![];
        for _ in 0..3 {
            ids.push(run(&mut s, "connector.add", json!({"from":1,"to":2,"glue":mode}))["id"].clone());
        }
        for fan in [false, true] {
            if fan {
                let result = run(&mut s, "layout.fan", json!({"hub":1,"spokes":[2]}));
                assert_eq!(result["assignments"].as_array().unwrap().len(), 3);
            }
            for (target, prefix) in [(1, "Begin"), (2, "End")] {
                let points = s.document().pages[0]
                    .connects
                    .iter()
                    .filter(|c| c.to_sheet == target)
                    .map(|c| c.to_cell.clone())
                    .collect::<std::collections::BTreeSet<_>>();
                assert_eq!(points.len(), 3, "distinct connection rows at target {target}");
                let mut positions = std::collections::BTreeSet::new();
                for id in &ids {
                    let x = run(&mut s, "sheet.get", json!({"id":id,"name":format!("{prefix}X")}))["value"].to_string();
                    let y = run(&mut s, "sheet.get", json!({"id":id,"name":format!("{prefix}Y")}))["value"].to_string();
                    positions.insert((x, y));
                    assert_eq!(run(&mut s, "sheet.get", json!({"id":id,"name":"ShapeRouteStyle"}))["value"], "2");
                }
                assert_eq!(positions.len(), 3, "distinct endpoint positions at target {target}");
            }
            let checked = run(&mut s, "layout.check", json!({}));
            assert!(!checked["violations"].as_array().unwrap().iter().any(|v| v["rule"] == "d"), "{checked}");
        }
    }
}
#[test]
fn invalid_compound_parameters_are_atomic_and_references_return_connector_id() {
    let mut s = pair();
    let before = s.document().clone();
    for (command, p) in [
        ("shape.draw.rect", json!({"x":0,"y":0,"width":1,"height":1,"line":{"weight":-1}})),
        ("master.drop", json!({"master":"Dynamic connector","font":{"id":1}})),
        ("connector.add", json!({"from":1,"to":999})),
        ("connector.add", json!({"from":1,"to":2,"from_point":"Connections.X1"})),
        ("connector.add", json!({"from":1,"to":2,"line":{"page":2}})),
    ] {
        assert!(s.execute(command, &p).is_err());
        assert_eq!(s.document(), &before);
    }
    let result = s
        .execute_batch(&json!({"commands":[{"command":"connector.add","as":"link","params":{"from":1,"to":2}},
        {"command":"shape.text","params":{"id":"$link","text":"a link"}}]}))
        .unwrap();
    assert_eq!(result["references"]["link"], result["results"][0]["id"]);
}

// Append to create_style.rs. Uses that module's run() and pair() helpers.
fn review_assert_distinct_ports(s: &mut Session, target: u32, expected: usize) {
    let connections = s.document().pages[0].connects.iter().filter(|c| c.to_sheet == target).cloned().collect::<Vec<_>>();
    assert_eq!(connections.len(), expected);
    let rows = connections.iter().map(|c| c.to_cell.clone()).collect::<std::collections::BTreeSet<_>>();
    assert_eq!(rows.len(), expected, "shared connection rows at shape {target}");
    let mut positions = std::collections::BTreeSet::new();
    for c in connections {
        let x_name = c.from_cell.as_deref().unwrap();
        let y_name = x_name.replace('X', "Y");
        let x = run(s, "sheet.get", json!({"id":c.from_sheet,"name":x_name}))["value"].to_string();
        let y = run(s, "sheet.get", json!({"id":c.from_sheet,"name":y_name}))["value"].to_string();
        positions.insert((x, y));
    }
    assert_eq!(positions.len(), expected, "shared endpoint positions at shape {target}");
}

#[test]
fn parallel_links_at_touching_tiers_do_not_overlap_before_or_after_fan() {
    for mode in ["static", "dynamic"] {
        let mut s = Session::new();
        for (x, y) in [(1, 3), (5, 5)] {
            run(&mut s, "shape.draw.rect", json!({"x":x,"y":y,"width":2,"height":2}));
        }
        for _ in 0..3 {
            run(&mut s, "connector.add", json!({"from":1,"to":2,"glue":mode}));
        }
        for apply_fan in [false, true] {
            if apply_fan {
                run(&mut s, "layout.fan", json!({"hub":1,"spokes":[2]}));
            }
            review_assert_distinct_ports(&mut s, 1, 3);
            review_assert_distinct_ports(&mut s, 2, 3);
            let checked = run(&mut s, "layout.check", json!({}));
            assert!(!checked["violations"].as_array().unwrap().iter().any(|v| v["rule"] == "d"), "{checked}");
        }
    }
}

#[test]
fn automatic_ports_remain_distinct_across_existing_neighbor_pairs() {
    for mode in ["static", "dynamic"] {
        let mut s = Session::new();
        for (x, y) in [(1, 3), (5, 2), (8, 4)] {
            run(&mut s, "shape.draw.rect", json!({"x":x,"y":y,"width":2,"height":2}));
        }
        for (from, to) in [(1, 2), (1, 2), (1, 3), (1, 3)] {
            run(&mut s, "connector.add", json!({"from":from,"to":to,"glue":mode}));
        }
        review_assert_distinct_ports(&mut s, 1, 4);
        review_assert_distinct_ports(&mut s, 2, 2);
        review_assert_distinct_ports(&mut s, 3, 2);
    }
}

#[test]
fn automatic_parallel_add_preserves_numbered_and_named_explicit_ports() {
    for name in [None, Some("Port"), Some("XPort")] {
        for mode in ["static", "dynamic"] {
            let mut s = pair();
            let mut points = Vec::new();
            for (id, x) in [(1, json!("Width")), (2, json!(0))] {
                let mut params = json!({"id":id,"x":x,"y":0});
                if let Some(name) = name {
                    params["name"] = json!(name);
                }
                points.push(run(&mut s, "shape.connection.add", params)["cell"].clone());
            }
            let explicit = run(&mut s, "connector.add", json!({"from":1,"to":2,"from_point":points[0],"to_point":points[1]}))["id"].as_u64().unwrap();
            let before = s.document().pages[0].connects.iter().filter(|c| u64::from(c.from_sheet) == explicit).cloned().collect::<Vec<_>>();
            run(&mut s, "connector.add", json!({"from":1,"to":2,"glue":mode}));
            let after = s.document().pages[0].connects.iter().filter(|c| u64::from(c.from_sheet) == explicit).cloned().collect::<Vec<_>>();
            assert_eq!(before, after, "explicit port changed for {name:?}, new connector mode {mode}");
            review_assert_distinct_ports(&mut s, 1, 2);
            review_assert_distinct_ports(&mut s, 2, 2);
        }
    }
}

#[test]
fn parallel_connector_packages_match_fine_grained_point_edits() {
    let mut fine = pair();
    let mut compound = pair();
    let mut previous = Vec::new();
    for fraction in [None, Some(0.25), Some(0.75)] {
        let id = run(&mut fine, "master.drop", json!({"master":"Dynamic connector","x":0,"y":0}))["id"].clone();
        if fraction.is_some() {
            for link in previous.iter().chain(std::iter::once(&id)) {
                run(&mut fine, "sheet.set", json!({"id":link,"name":"ShapeRouteStyle","value":2}));
            }
        }
        run(&mut fine, "connect.sides", json!({"id":id,"source":1,"target":2}));
        if let Some(fraction) = fraction {
            for (target, end, x) in [(1, "begin", "Width"), (2, "end", "0")] {
                let point = run(&mut fine, "shape.connection.add", json!({"id":target,"x":x,"y":format!("Height*{fraction}")}))["cell"].clone();
                run(&mut fine, "connect.static", json!({"id":id,"end":end,"target":target,"point":point}));
            }
            for link in previous.iter().chain(std::iter::once(&id)) {
                run(&mut fine, "sheet.set", json!({"id":link,"name":"ShapeRouteStyle","value":2}));
            }
        }
        previous.push(id);
        run(&mut compound, "connector.add", json!({"from":1,"to":2}));
    }
    equal(&mut fine, &mut compound, "parallel-fine-points");
}

/// Visio witness: corpus/_state/brief27/first-acceptance/b27-fabric/saved.vsdx.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/lineweight-cell-line-format-section
#[test]
fn authored_point_weights_use_visio_decimal_serialization() {
    let mut s = pair();
    for (weight, expected) in [(1., "0.01388888888888889"), (1.5, "0.02083333333333333")] {
        run(&mut s, "format.line", json!({"id":1,"weight":weight,"units":"PT"}));
        let cell = s.document().pages[0].shapes[0].sheet.cell("LineWeight").unwrap();
        assert_eq!(cell.value.as_deref(), Some(expected));
        assert_eq!(cell.units.as_deref(), Some("PT"));
        assert!(cell.formula.is_none());
    }
}

/// Visio removes literal-zero F from authored connection coordinates.
/// Witness: corpus/_state/brief27/first-acceptance/b27-san/saved.vsdx.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/connection-points-section
#[test]
fn literal_zero_connection_coordinates_have_no_formula() {
    let mut s = pair();
    run(&mut s, "shape.connection.add", json!({"id":1,"x":"0","y":"0"}));
    let section = s.document().pages[0].shapes[0].sheet.sections.iter().find(|v| v.name == "Connection").unwrap();
    let row = section.rows.last().unwrap();
    for name in ["X", "Y"] {
        let cell = row.cells.iter().find(|c| c.name == name).unwrap();
        assert_eq!(cell.value.as_deref(), Some("0"));
        assert!(cell.formula.is_none(), "literal zero {name} must not retain F");
    }
}
