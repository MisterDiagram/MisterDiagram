//! Contributor-original reproductions of #43 and the W1/W2 observations.
use misterdiagram_engine::Session;
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
use serde_json::{Value, json};
use std::collections::BTreeSet;
fn run(s: &mut Session, command: &str, params: Value) -> Value {
    s.execute(command, &params).unwrap_or_else(|e| panic!("{command} {params}: {e}"))
}
fn number(s: &Session, id: u32, name: &str) -> f64 {
    Evaluator::new(s.document()).unwrap().evaluate(Scope::PageShape { page: s.document().pages[0].id, shape: id }, name).numeric().unwrap()
}
fn references(formula: &str) -> Vec<u32> {
    // Fixtures contain no quoted Sheet references. Inspect every saved formula, not just endpoints.
    formula.split("Sheet.").skip(1).filter_map(|s| s.split_once('!').and_then(|(id, _)| id.parse().ok())).collect()
}
fn reachable(s: &Session) {
    let bytes = misterdiagram_vsdx::write(s.document()).unwrap().bytes;
    let doc = misterdiagram_vsdx::import(&bytes).unwrap();
    for (part, shapes, sheet) in
        doc.pages.iter().map(|p| (&p.part, &p.shapes, &p.sheet)).chain(doc.masters.iter().map(|m| (&m.part, &m.shapes, &m.sheet)))
    {
        let mut all = vec![];
        let mut pending = shapes.iter().collect::<Vec<_>>();
        while let Some(s) = pending.pop() {
            all.push(s);
            pending.extend(&s.shapes);
        }
        let ids = all.iter().map(|s| s.id).collect::<BTreeSet<_>>();
        let mut xml = doc.parts.get(part).and_then(|p| p.xml.as_ref()).into_iter().collect::<Vec<_>>();
        while let Some(element) = xml.pop() {
            if let Some(formula) = element.attr("F") {
                for id in references(formula) {
                    assert!(ids.contains(&id), "saved {part} has dangling Sheet.{id}: {formula}");
                }
            }
            xml.extend(element.elements());
        }
        for sheet in std::iter::once(sheet).chain(all.iter().map(|s| &s.sheet)) {
            for cell in sheet.cells.iter().chain(sheet.sections.iter().flat_map(|s| s.cells.iter().chain(s.rows.iter().flat_map(|r| &r.cells)))) {
                for id in cell.formula.as_deref().map(references).unwrap_or_default() {
                    assert!(ids.contains(&id), "{part} {} has dangling Sheet.{id}: {:?}", cell.name, cell.formula);
                }
            }
        }
    }
    for p in &doc.pages {
        let mut e = Evaluator::new(&doc).unwrap();
        for c in &p.connects {
            let scope = Scope::PageShape { page: p.id, shape: c.from_sheet };
            let name = c.from_cell.as_deref().unwrap();
            let endpoint = e.cell(scope, name).unwrap();
            assert!(endpoint.formula.as_deref().is_some_and(|f| f != "Inh"), "Connect row has literal {name}");
            let trigger = if name.starts_with("Begin") { "BegTrigger" } else { "EndTrigger" };
            let refs = endpoint
                .formula
                .as_deref()
                .map(references)
                .unwrap_or_default()
                .into_iter()
                .chain(e.cell(scope, trigger).and_then(|c| c.formula.as_deref()).map(references).unwrap_or_default())
                .collect::<Vec<_>>();
            assert!(refs.contains(&c.to_sheet), "Connect target {} disagrees with {name}/{trigger}", c.to_sheet);
            assert!(e.evaluate(scope, name).numeric().is_ok());
        }
    }
}
fn connected(dynamic: bool) -> Session {
    let mut s = Session::new();
    for x in [1, 6] {
        run(&mut s, "shape.draw.rect", json!({"x":x,"y":4,"width":1,"height":1}));
    }
    run(&mut s, "master.drop", json!({"master":2,"x":4,"y":1}));
    for (end, target) in [("begin", 1), ("end", 2)] {
        if dynamic {
            run(&mut s, "connect.dynamic", json!({"id":3,"end":end,"target":target}));
        } else {
            run(&mut s, "shape.connection.add", json!({"id":target,"x":"Width*0.5","y":"Height*0.5"}));
            run(&mut s, "connect.static", json!({"id":3,"end":end,"target":target,"point":"Connections.X1"}));
        }
    }
    s
}
#[test]
fn member_master_literals_do_not_follow_or_reference_the_former_group() {
    let mut s = Session::new();
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":1,"width":1,"height":1}));
    run(&mut s, "shape.draw.rect", json!({"x":3,"y":1,"width":2,"height":1}));
    run(&mut s, "shape.group", json!({"ids":[1,2]}));
    run(&mut s, "master.new_from_shape", json!({"id":2,"name":"Child"}));
    run(&mut s, "master.drop", json!({"master":"Child","x":4,"y":6}));
    run(&mut s, "shape.resize", json!({"id":3,"width":10,"height":1}));
    assert_eq!(number(&s, 4, "Width"), 2.);
    assert_eq!(number(&s, 4, "Height"), 1.);
    reachable(&s);
    run(&mut s, "shape.delete", json!({"id":3}));
    reachable(&s);
}
#[test]
fn connector_master_literals_are_independent_on_another_page() {
    for dynamic in [false, true] {
        let mut s = connected(dynamic);
        run(&mut s, "master.new_from_shape", json!({"id":3,"name":"Wire"}));
        let master = s.document().masters.iter().find(|m| m.name_u == "Wire").unwrap();
        let root = master.shapes.first().unwrap();
        for name in ["BeginX", "BeginY", "EndX", "EndY", "BegTrigger", "EndTrigger"] {
            assert!(root.sheet.cell(name).unwrap().formula.is_none(), "copied external glue retained {name}");
        }
        run(&mut s, "page.add", json!({"name":"Independent"}));
        run(&mut s, "master.drop", json!({"page":2,"master":"Wire","x":4,"y":2}));
        assert!(s.document().pages[1].connects.is_empty());
        reachable(&s);
    }
}
#[test]
fn ungroup_detaches_static_and_dynamic_glue_and_other_group_references() {
    for dynamic in [false, true] {
        let mut s = Session::new();
        for x in [1, 3, 7] {
            run(&mut s, "shape.draw.rect", json!({"x":x,"y":4,"width":1,"height":1}));
        }
        run(&mut s, "shape.group", json!({"ids":[1,2]}));
        run(&mut s, "master.drop", json!({"master":2,"x":5,"y":1}));
        run(&mut s, "sheet.set", json!({"id":3,"name":"Width","formula":"Sheet.4!Width"}));
        for (end, target) in [("begin", 4), ("end", 3)] {
            if dynamic {
                run(&mut s, "connect.dynamic", json!({"id":5,"end":end,"target":target}));
            } else {
                run(&mut s, "shape.connection.add", json!({"id":target,"x":"Width*0.5","y":"Height*0.5"}));
                run(&mut s, "connect.static", json!({"id":5,"end":end,"target":target,"point":"Connections.X1"}));
            }
        }
        let before = [number(&s, 5, "BeginX"), number(&s, 5, "BeginY")];
        run(&mut s, "shape.ungroup", json!({"id":4}));
        assert_eq!([number(&s, 5, "BeginX"), number(&s, 5, "BeginY")], before);
        assert_eq!(number(&s, 5, "BegTrigger"), 0.);
        assert!(s.document().pages[0].connects.iter().all(|c| c.to_sheet != 4));
        reachable(&s);
    }
}
#[test]
fn duplicated_connector_has_no_external_glue_before_or_after_target_deletion() {
    for dynamic in [false, true] {
        let mut s = connected(dynamic);
        run(&mut s, "shape.duplicate", json!({"id":3,"dx":0,"dy":-2}));
        assert_eq!(number(&s, 4, "BegTrigger"), 0.);
        assert_eq!(number(&s, 4, "EndTrigger"), 0.);
        assert!(s.document().pages[0].connects.iter().all(|c| c.from_sheet != 4));
        reachable(&s);
        run(&mut s, "shape.delete", json!({"id":1}));
        reachable(&s);
    }
}
#[test]
fn move_and_arrangements_remove_glue_rows_and_triggers_for_changed_ends() {
    for dynamic in [false, true] {
        for (command, params) in [
            ("shape.move", json!({"id":3,"x":4,"y":6})),
            ("arrange.align", json!({"ids":[1,3],"mode":"bottom"})),
            ("arrange.distribute", json!({"ids":[1,3,4],"axis":"y"})),
            ("arrange.same_size", json!({"ids":[1,3],"axis":"width"})),
        ] {
            let mut s = connected(dynamic);
            // Detachment exposes #44 when the old target surrounds the free end.
            // Keep this bookkeeping check independent; the exact orthogonal repro
            // is covered with the endpoint-obstacle fix in A4.
            run(&mut s, "sheet.set", json!({"scope":"page","name":"RouteStyle","value":2}));
            // Give distribution distinct endpoints without moving the connector itself.
            if command == "arrange.distribute" {
                run(&mut s, "shape.draw.rect", json!({"x":9,"y":12,"width":1,"height":1}));
            }
            run(&mut s, command, params);
            assert!(s.document().pages[0].connects.iter().all(|c| c.from_sheet != 3), "{command} retained glue rows");
            assert_eq!(number(&s, 3, "BegTrigger"), 0., "{command}");
            assert_eq!(number(&s, 3, "EndTrigger"), 0., "{command}");
            let y = number(&s, 3, "BeginY");
            run(&mut s, "shape.move", json!({"id":1,"x":1.5,"y":8}));
            assert_eq!(number(&s, 3, "BeginY"), y, "{command} still follows target");
            reachable(&s);
        }
    }
}

#[test]
fn copying_a_connected_group_preserves_internal_glue() {
    for dynamic in [false, true] {
        let mut s = connected(dynamic);
        run(&mut s, "shape.group", json!({"ids":[1,2,3]}));
        run(&mut s, "master.new_from_shape", json!({"id":4,"name":"ConnectedGroup"}));
        let master = s.document().masters.iter().find(|m| m.name_u == "ConnectedGroup").unwrap();
        assert_eq!(master.connects.len(), 2);
        run(&mut s, "shape.duplicate", json!({"id":4,"dx":0,"dy":-4}));
        assert_eq!(s.document().pages[0].connects.len(), 4);
        reachable(&s);
        run(&mut s, "shape.delete", json!({"id":4}));
        assert_eq!(s.document().pages[0].connects.len(), 2);
        reachable(&s);
    }
}

#[test]
fn unchanged_endpoint_edits_keep_glue() {
    for dynamic in [false, true] {
        let mut s = connected(dynamic);
        let x = number(&s, 3, "PinX");
        let y = number(&s, 3, "PinY");
        for (command, p) in [
            ("shape.move", json!({"id":3,"x":x,"y":y})),
            ("arrange.align", json!({"ids":[3],"mode":"bottom"})),
            ("arrange.same_size", json!({"ids":[3]})),
        ] {
            run(&mut s, command, p);
            assert_eq!(s.document().pages[0].connects.len(), 2, "{command} detached an unmoved end");
            reachable(&s);
        }
    }
}

#[test]
fn copied_external_boolean_formulas_become_numeric_literals() {
    let mut s = Session::new();
    for x in [1, 4] {
        run(&mut s, "shape.draw.rect", json!({"x":x,"y":1,"width":1,"height":1}));
    }
    run(&mut s, "sheet.set", json!({"id":2,"name":"FlipX","formula":"Sheet.1!Width>0"}));
    run(&mut s, "shape.duplicate", json!({"id":2,"dx":0,"dy":2}));
    assert_eq!(number(&s, 3, "FlipX"), 1.);
    run(&mut s, "master.new_from_shape", json!({"id":2,"name":"Boolean"}));
    run(&mut s, "master.drop", json!({"master":"Boolean","x":7,"y":2}));
    assert_eq!(number(&s, 4, "FlipX"), 1.);
    run(&mut s, "shape.delete", json!({"id":2}));
    run(&mut s, "shape.delete", json!({"id":1}));
    reachable(&s);
}
#[test]
fn ungroup_preserves_references_to_the_same_id_on_another_page() {
    let mut s = Session::new();
    for x in [1, 3] {
        run(&mut s, "shape.draw.rect", json!({"x":x,"y":1,"width":1,"height":1}));
    }
    run(&mut s, "shape.group", json!({"ids":[1,2]}));
    run(&mut s, "page.add", json!({"name":"Elsewhere"}));
    for y in [1, 3, 5] {
        run(&mut s, "shape.draw.rect", json!({"page":2,"x":1,"y":y,"width":7,"height":1}));
    }
    run(&mut s, "shape.draw.rect", json!({"page":1,"x":5,"y":1,"width":1,"height":1}));
    let formula = "Pages[Elsewhere]!Sheet.3!Width";
    run(&mut s, "sheet.set", json!({"page":1,"id":4,"name":"Width","formula":formula}));
    run(&mut s, "shape.ungroup", json!({"page":1,"id":3}));
    assert_eq!(s.document().pages[0].shapes.iter().find(|s| s.id == 4).unwrap().sheet.cell("Width").unwrap().formula.as_deref(), Some(formula));
    run(&mut s, "shape.resize", json!({"page":2,"id":3,"width":9,"height":1}));
    assert_eq!(number(&s, 4, "Width"), 9.);
    let round = misterdiagram_vsdx::import(&misterdiagram_vsdx::write(s.document()).unwrap().bytes).unwrap();
    assert_eq!(Evaluator::new(&round).unwrap().evaluate(Scope::PageShape { page: round.pages[0].id, shape: 4 }, "Width").numeric().unwrap(), 9.);
}

#[test]
fn copied_font_formulas_keep_serialized_face_identity() {
    let mut s = Session::new();
    for x in [1, 4] {
        run(&mut s, "shape.draw.rect", json!({"x":x,"y":1,"width":1,"height":1}));
    }
    let face = s.document().face_names.values().next().unwrap().clone();
    run(&mut s, "sheet.set", json!({"id":2,"name":"Char.Font","formula":format!("IF(Sheet.1!Width>0,FONT(\"{face}\"),0)")}));
    let before = number(&s, 2, "Char.Font");
    run(&mut s, "shape.duplicate", json!({"id":2,"dx":0,"dy":2}));
    assert_eq!(number(&s, 3, "Char.Font"), before);
    let bytes = misterdiagram_vsdx::write(s.document()).unwrap().bytes;
    let doc = misterdiagram_vsdx::import(&bytes).unwrap();
    let mut e = Evaluator::new(&doc).unwrap();
    let scope = Scope::PageShape { page: doc.pages[0].id, shape: 3 };
    assert_eq!(e.cell(scope, "Char.Font").unwrap().value.as_deref(), Some(face.as_str()));
    assert_eq!(e.evaluate(scope, "Char.Font").numeric().unwrap(), before);
}
#[test]
fn copied_named_internal_references_follow_the_copied_sibling() {
    for master in [false, true] {
        let mut s = Session::new();
        for x in [1, 4] {
            run(&mut s, "shape.draw.rect", json!({"x":x,"y":1,"width":1,"height":1}));
        }
        s.edit_document(|doc, _| {
            doc.pages[0].shapes[0].name_u = "NamedA".into();
            Ok(())
        })
        .unwrap();
        run(&mut s, "shape.group", json!({"ids":[1,2]}));
        run(&mut s, "sheet.set", json!({"id":2,"name":"Width","formula":"NamedA!Width+LEN(\"NamedA!Width\")*0"}));
        if master {
            run(&mut s, "master.new_from_shape", json!({"id":3,"name":"NamedGroup"}));
            run(&mut s, "master.drop", json!({"master":"NamedGroup","x":5,"y":5}));
        } else {
            run(&mut s, "shape.duplicate", json!({"id":3,"dx":0,"dy":3}));
        }
        let copy = s.document().pages[0].shapes.iter().find(|s| s.id != 3).unwrap();
        let dependent = copy
            .shapes
            .iter()
            .find(|shape| {
                Evaluator::new(s.document())
                    .unwrap()
                    .cell(Scope::PageShape { page: s.document().pages[0].id, shape: shape.id }, "Width")
                    .and_then(|c| c.formula.as_deref())
                    .is_some_and(|f| f.contains("LEN"))
            })
            .unwrap()
            .id;
        let target = copy.shapes.iter().find(|s| s.id != dependent).unwrap().id;
        run(&mut s, "shape.resize", json!({"id":1,"width":2,"height":1}));
        assert_eq!(number(&s, dependent, "Width"), 1., "copy followed original sibling");
        run(&mut s, "shape.resize", json!({"id":target,"width":3,"height":1}));
        assert_eq!(number(&s, dependent, "Width"), 3.);
        run(&mut s, "shape.delete", json!({"id":3}));
        reachable(&s);
    }
}
#[test]
fn inactive_rows_lose_external_references_without_being_resurrected() {
    for ungroup in [false, true] {
        let mut s = Session::new();
        for x in [1, 4] {
            run(&mut s, "shape.draw.rect", json!({"x":x,"y":1,"width":1,"height":1}));
        }
        run(&mut s, "shape.group", json!({"ids":[1,2]}));
        s.edit_document(|doc, _| {
            let shape = doc.pages[0].shapes[0].shapes.iter_mut().find(|s| s.id == 2).unwrap();
            let geometry = shape.sheet.sections.iter_mut().find(|s| s.name == "Geometry").unwrap();
            geometry.rows.push(misterdiagram_model::Row {
                index: Some(99),
                row_type: Some("LineTo".into()),
                deleted: true,
                cells: vec![misterdiagram_model::Cell {
                    name: "X".into(),
                    value: Some("7".into()),
                    formula: Some("Sheet.3!Width".into()),
                    ..Default::default()
                }],
                ..Default::default()
            });
            Ok(())
        })
        .unwrap();
        if ungroup {
            run(&mut s, "shape.ungroup", json!({"id":3}));
        } else {
            run(&mut s, "master.new_from_shape", json!({"id":2,"name":"Inactive"}));
        }
        let shape = if ungroup {
            s.document().pages[0].shapes.iter().find(|s| s.id == 2).unwrap()
        } else {
            &s.document().masters.iter().find(|m| m.name_u == "Inactive").unwrap().shapes[0]
        };
        let row = shape.sheet.sections.iter().find(|s| s.name == "Geometry").unwrap().rows.iter().find(|r| r.index == Some(99)).unwrap();
        assert!(row.deleted);
        assert_eq!(row.cells[0].value.as_deref(), Some("7"));
        assert!(row.cells[0].formula.is_none());
        reachable(&s);
    }
}
