//! Headless inspection is evaluated, bounded and shares strict command semantics.
use misterdiagram_engine::Session;
use serde_json::{Value, json};
fn run(s: &mut Session, command: &str, params: Value) -> Value {
    s.execute(command, &params).unwrap()
}
#[test]
fn control_registry_and_hostile_params_preserve_state() {
    let mut s = Session::new();
    for method in ["engine.execute", "engine.commands", "document.inspect", "shape.inspect"] {
        assert!(s.commands().iter().any(|c| c.id == method));
        let before = s.document().clone();
        for p in [Value::Null, json!([]), json!({"unexpected":1}), json!({"id":-1,"page":0}), json!({"cells":[false]})] {
            assert!(s.execute(method, &p).is_err(), "{method}: {p}");
            assert_eq!(s.document(), &before);
        }
    }
    assert!(s.execute("engine.execute", &json!({"command":"engine.execute","params":{"command":"file.new"}})).is_err());
    assert!(s.execute("engine.execute", &json!({"command":"file.new","params":[]})).is_err());
    assert!(s.execute("shape.inspect", &json!({"id":999})).is_err());
    assert!(s.execute("document.inspect", &json!({"page":2})).is_err());
    let commands = run(&mut s, "engine.commands", json!({}));
    assert_eq!(commands.as_array().unwrap().len(), s.commands().len());
}
#[test]
fn execute_alias_has_one_strict_undo_boundary() {
    let mut s = Session::new();
    run(&mut s, "engine.execute", json!({"command":"shape.draw.rect","params":{"x":1,"y":2,"width":3,"height":4}}));
    assert_eq!(s.document().pages[0].shapes.len(), 1);
    assert!(!s.document().parts.is_empty());
    assert_eq!(s.journal().len(), 1);
    run(&mut s, "edit.undo", json!({}));
    assert!(s.document().pages[0].shapes.is_empty());
    assert_eq!(run(&mut s, "edit.undo", json!({}))["changed"], false);
}
#[test]
fn inspection_exposes_evaluated_inherited_data_text_glue_and_page_bounds() {
    let mut s = Session::new();
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":2,"width":3,"height":4}));
    run(&mut s, "shape.text", json!({"id":1,"text":"Invented Hub"}));
    run(&mut s, "shape.data.define", json!({"id":1,"name":"Label","label":"Label"}));
    run(&mut s, "shape.data.set", json!({"id":1,"name":"Label","value":"009"}));
    run(&mut s, "master.new_from_shape", json!({"id":1,"name":"Invented"}));
    run(&mut s, "master.drop", json!({"master":"Invented","x":7,"y":7}));
    run(&mut s, "shape.connection.add", json!({"id":1,"x":"Width","y":"Height/2"}));
    run(&mut s, "master.drop", json!({"master":2,"x":0,"y":0}));
    run(&mut s, "connect.static", json!({"id":3,"end":"begin","target":1,"point":"Connections.X1"}));
    run(&mut s, "layer.add", json!({"name":"Devices"}));
    let before = s.document().clone();
    let history = s.journal().len();
    let shape = run(&mut s, "shape.inspect", json!({"id":1,"cells":["Width","Prop.Label"],"sections":["cells","connection_points","glue"]}));
    assert_eq!(shape["bounds"], json!({"x":1.0,"y":2.0,"width":3.0,"height":4.0}));
    assert_eq!(shape["cells"]["Width"]["value"], "3");
    assert_eq!(shape["cells"]["Prop.Label"]["value"], "009");
    assert_eq!(shape["connection_points"]["Connections.X1"]["value"], "3");
    assert_eq!(shape["connects"][0]["to_cell"], "Connections.X1");
    let dropped = run(&mut s, "shape.inspect", json!({"id":2}));
    assert!(dropped["text"].as_str().unwrap().contains("Invented Hub"));
    assert_eq!(dropped["data"]["Prop.Label"]["value"], "009");
    assert!(dropped["master"].is_number());
    let glue = run(&mut s, "shape.inspect", json!({"id":3}));
    assert!(glue["glue"]["BeginX"]["formula"].as_str().unwrap().contains("Connections.X1"));
    let doc = run(&mut s, "document.inspect", json!({"page":1}));
    assert_eq!(doc["pages"][0]["shapes"].as_array().unwrap().len(), 3);
    assert_eq!(doc["pages"][0]["connects"].as_array().unwrap().len(), 1);
    assert!(doc["pages"][0]["layers"].to_string().contains("Devices"));
    assert_eq!(s.document(), &before);
    assert_eq!(s.journal().len(), history);
    for p in [
        json!({"id":1,"cells":[false]}),
        json!({"id":1,"cells":["z".repeat(257)]}),
        json!({"id":1,"cells":vec!["Width";257]}),
        json!({"id":1,"cells":["NotACell"]}),
    ] {
        assert!(s.execute("shape.inspect", &p).is_err());
        assert_eq!(s.document(), &before);
    }
    run(&mut s, "shape.group", json!({"ids":[1,2]}));
    let child = run(&mut s, "shape.inspect", json!({"id":1}));
    assert_eq!(child["bounds"], shape["bounds"]);
}

#[test]
fn dense_master_inspection_filters_before_expansion_and_fails_over_budget() {
    let mut s = Session::new();
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":2,"width":3,"height":4}));
    // Original synthetic master: no vendor fixture or asset.
    for i in 0..200 {
        run(&mut s, "shape.connection.add", json!({"id":1,"x":format!("Width*{i}/200"),"y":"Height/2"}));
    }
    for i in 0..100 {
        run(&mut s, "shape.data.define", json!({"id":1,"name":format!("Field{i}"),"label":format!("Invented field {i}")}));
    }
    let master = run(&mut s, "master.new_from_shape", json!({"id":1,"name":"Dense original"}))["master"].clone();
    let id = run(&mut s, "master.drop", json!({"master":master,"x":7,"y":7}))["id"].clone();
    let before = s.document().clone();
    let minimal = run(&mut s, "shape.inspect", json!({"id":id,"cells":["Width","Height"]}));
    assert_eq!(minimal.as_object().unwrap().keys().map(String::as_str).collect::<Vec<_>>(), vec!["bounds", "cells", "id"]);
    assert_eq!(minimal["cells"].as_object().unwrap().len(), 2);
    assert_eq!(minimal["cells"]["Width"]["value"], "3");
    assert_eq!(minimal["cells"]["Height"]["value"], "4");
    assert!(minimal.to_string().len() < 1024);
    let error = s.execute("shape.inspect", &json!({"id":id})).unwrap_err().to_string();
    assert!(error.contains("budget") && error.contains("sections"), "{error}");
    let points = run(&mut s, "shape.inspect", json!({"id":id,"sections":["cells"],"cells":["Connections.X200"]}));
    assert!(points["cells"].get("Connections.X200").is_some());
    for section in ["xform", "text", "glue"] {
        let v = run(&mut s, "shape.inspect", json!({"id":id,"sections":[section]}));
        assert!(v.get(section).is_some());
        assert!(v.get("connection_points").is_none());
    }
    let data = run(&mut s, "shape.inspect", json!({"id":id,"cells":["Prop.Field99"]}));
    assert!(data["cells"].get("Prop.Field99").is_some());
    let doc = run(&mut s, "document.inspect", json!({"page":1,"sections":["cells"],"cells":["Width"]}));
    for shape in doc["pages"][0]["shapes"].as_array().unwrap() {
        assert_eq!(shape.as_object().unwrap().len(), 3);
        assert_eq!(shape["cells"].as_object().unwrap().len(), 1);
    }
    for params in [json!({"id":id,"sections":["unknown"]}), json!({"id":id,"sections":false}), json!({"id":id,"cells":[null]})] {
        assert!(s.execute("shape.inspect", &params).is_err());
    }
    assert_eq!(s.document(), &before);
}

#[test]
fn repeated_cell_names_do_not_charge_for_absent_output() {
    let mut s = Session::new();
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":2,"width":3,"height":4}));
    run(&mut s, "sheet.set", json!({"id":1,"name":"Width","formula":format!("3{}", " ".repeat(500))}));
    let single = run(&mut s, "shape.inspect", json!({"id":1,"cells":["Width"]}));
    let repeated = run(&mut s, "shape.inspect", json!({"id":1,"cells":vec!["Width";256]}));
    assert_eq!(single, repeated);
}

#[test]
fn oversized_escaped_batch_result_rolls_back_mutations() {
    let mut s = Session::new();
    let before = s.document().clone();
    let mut commands = vec![json!({"command":"shape.draw.rect","params":{"x":0,"y":0,"width":1,"height":1}})];
    // Schemas contain many escaped quotes: the raw results fit the older batch
    // limit while their MCP content envelope exceeds it.
    let registry = run(&mut s, "engine.commands", json!({}));
    let escaped_entry = serde_json::to_string(&registry.to_string()).unwrap().len();
    let count = 1024 * 1024 / escaped_entry + 1;
    let mut results = vec![json!({"id":1})];
    results.extend(std::iter::repeat_n(registry, count));
    let payload = json!({"results":results,"references":{}});
    let raw = serde_json::to_vec(&payload).unwrap().len();
    let envelope = json!({"content":[{"type":"text","text":payload.to_string()}],"isError":false});
    assert!(raw < 1024 * 1024, "raw result must fit: {raw}");
    assert!(serde_json::to_vec(&envelope).unwrap().len() > 1024 * 1024, "escaped envelope must overflow");
    commands.extend((0..count).map(|_| json!({"command":"engine.commands"})));
    let error = s.execute_batch(&json!({"commands":commands})).unwrap_err().to_string();
    assert!(error.contains("budget") && error.contains("fewer batch"), "{error}");
    assert_eq!(s.document(), &before);
    assert!(s.journal().is_empty());
}
