//! Batch aliases are ephemeral command inputs, with no saved marker cells.
use misterdiagram_engine::Session;
use serde_json::{Value, json};
fn run(s: &mut Session, command: &str, params: Value) -> Value {
    s.execute(command, &params).unwrap()
}
#[test]
fn names_resolve_in_order_in_scalar_array_and_nested_edge_ids() {
    let mut s = Session::new();
    run(&mut s, "shape.draw.rect", json!({"x":9,"y":9,"width":1,"height":1}));
    let result = run(
        &mut s,
        "engine.batch",
        json!({"commands":[
            {"command":"shape.draw.rect","as":"first","params":{"x":1,"y":1,"width":1,"height":1}},
            {"command":"shape.draw.ellipse","as":"second","params":{"x":4,"y":1,"width":1,"height":1}},
            {"command":"shape.text","params":{"id":"$first","text":"$$cost"}},
            {"command":"shape.text","params":{"id":"$second","text":"$ordinary text"}},
            {"command":"layout.tree","params":{"root":"$first","edges":[["$first","$second"]],"fit_page":false}},
            {"command":"shape.group","as":"group","params":{"ids":["$first","$second"]}}
        ]}),
    );
    let names = &result["references"];
    assert_eq!(names["first"], result["results"][0]["id"]);
    assert_eq!(names["second"], result["results"][1]["id"]);
    assert_eq!(names["group"], result["results"][5]["id"]);
    assert_ne!(names["first"], names["second"]);
    assert_eq!(
        run(&mut s, "shape.inspect", json!({"id":names["first"],"sections":["text"]}))["text"].as_str().unwrap().trim_end_matches('\n'),
        "$cost"
    );
    assert_eq!(
        run(&mut s, "shape.inspect", json!({"id":names["second"],"sections":["text"]}))["text"].as_str().unwrap().trim_end_matches('\n'),
        "$ordinary text"
    );
    run(&mut s, "edit.undo", json!({}));
    assert_eq!(s.document().pages[0].shapes.len(), 1);
}
#[test]
fn master_group_returns_its_actual_id_and_unknown_duplicate_names_roll_back() {
    let mut s = Session::new();
    for x in [1, 3] {
        run(&mut s, "shape.draw.rect", json!({"x":x,"y":1,"width":1,"height":1}));
    }
    let group = run(&mut s, "shape.group", json!({"ids":[1,2]}))["id"].clone();
    let master = run(&mut s, "master.new_from_shape", json!({"id":group,"name":"Original group"}))["master"].clone();
    let result = run(
        &mut s,
        "engine.batch",
        json!({"commands":[
            {"command":"master.drop","as":"drop","params":{"master":master,"x":6,"y":6}},
            {"command":"shape.text","params":{"id":"$drop","text":"Group label"}}
        ]}),
    );
    let id = result["references"]["drop"].clone();
    assert_eq!(id, result["results"][0]["id"]);
    assert_eq!(run(&mut s, "shape.inspect", json!({"id":id,"sections":["text"]}))["text"].as_str().unwrap().trim_end_matches('\n'), "Group label");
    let before = s.document().clone();
    let journal = s.journal().len();
    for item in [
        json!({"command":"shape.move","params":{"id":"$unknown","x":0,"y":0}}),
        json!({"command":"shape.draw.rect","as":"taken","params":{"x":0,"y":0,"width":1,"height":1}}),
        json!({"command":"shape.text","as":"no-id","params":{"id":"$taken","text":"bad"}}),
        json!({"command":"shape.text","as":false,"params":{"id":"$taken","text":"bad"}}),
    ] {
        let error = s
            .execute_batch(&json!({"commands":[{"command":"shape.draw.line","as":"taken","params":{"x1":0,"y1":0,"x2":1,"y2":1}},item]}))
            .unwrap_err()
            .to_string();
        assert!(error.contains("index 1 (command 2)"), "{error}");
        assert_eq!(s.document(), &before);
        assert_eq!(s.journal().len(), journal);
    }
}

#[test]
fn nested_batches_are_rejected_before_alias_resolution_or_text_unescaping() {
    let mut s = Session::new();
    let before = s.document().clone();
    let inner = json!({"commands":[
        {"command":"shape.draw.rect","as":"same","params":{"x":0,"y":0,"width":1,"height":1}},
        {"command":"shape.text","params":{"id":"$same","text":"$$$cost"}}
    ]});
    for item in
        [json!({"command":"engine.batch","params":inner}), json!({"command":"engine.execute","params":{"command":"engine.batch","params":inner}})]
    {
        let error = s
            .execute_batch(&json!({"commands":[
                {"command":"shape.draw.rect","as":"same","params":{"x":3,"y":0,"width":1,"height":1}},item
            ]}))
            .unwrap_err()
            .to_string();
        assert!(error.contains("index 1") && error.contains("nested batches"), "{error}");
        assert_eq!(s.document(), &before);
        assert!(s.journal().is_empty());
    }
    let result = s
        .execute_batch(&json!({"commands":[
            {"command":"shape.draw.rect","as":"first","params":{"x":0,"y":0,"width":1,"height":1}},
            {"command":"engine.execute","params":{"command":"shape.text","params":{"id":"$first","text":"$$$cost"}}}
        ]}))
        .unwrap();
    assert_eq!(
        run(&mut s, "shape.inspect", json!({"id":result["references"]["first"],"sections":["text"]}))["text"]
            .as_str()
            .unwrap()
            .trim_end_matches('\n'),
        "$$cost"
    );
}
