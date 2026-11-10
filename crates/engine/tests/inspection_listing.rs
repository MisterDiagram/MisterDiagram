//! The listing escape hatch retains the legacy document inspection contract.
use misterdiagram_engine::Session;
use serde_json::{Value, json};

fn drawing() -> Session {
    let mut s = Session::new();
    for x in [1, 5] {
        s.execute("shape.draw.rect", &json!({"x":x,"y":1,"width":2,"height":1})).unwrap();
    }
    s.execute("shape.text", &json!({"id":1,"text":"Cedar"})).unwrap();
    s.execute("sheet.set", &json!({"id":1,"name":"TxtWidth","formula":"Width/2"})).unwrap();
    s.execute("shape.data.define", &json!({"id":1,"name":"Code","label":"Code"})).unwrap();
    s.execute("shape.data.set", &json!({"id":1,"name":"Code","value":"A7"})).unwrap();
    s.execute("layer.add", &json!({"name":"Nodes"})).unwrap();
    s.execute("connector.add", &json!({"from":1,"to":2})).unwrap();
    s.execute("page.add", &json!({"name":"Second"})).unwrap();
    s
}
fn reply(s: &mut Session, p: &Value) -> Result<String, String> {
    s.execute("document.inspect", p).map(|v| v.to_string()).map_err(|e| e.to_string())
}
fn listing(mut p: Value) -> Value {
    p["sections"] = json!(["listing"]);
    p
}
#[test]
fn listing_matches_legacy_bytes_for_all_selectors() {
    let mut s = drawing();
    for p in [json!({}), json!({"page":1}), json!({"find":"cedar"}), json!({"page":1,"ids":[3,1,2,1]}), json!({"find":"absent"})] {
        let old = reply(&mut s, &p).unwrap();
        assert_eq!(reply(&mut s, &listing(p)).unwrap(), old);
    }
    let schema: Value = serde_json::from_str(misterdiagram_engine::find_command("document.inspect").unwrap().params).unwrap();
    assert_eq!(schema["properties"]["sections"]["maxItems"], 8);
    assert_eq!(schema["properties"]["sections"]["items"]["enum"].as_array().unwrap().len(), 9);
}
#[test]
fn listing_rejects_combinations_and_shape_use() {
    let mut s = drawing();
    for sections in [json!(["listing", "text"]), json!(["text", "listing"]), json!(["listing", "listing"]), json!(["listing", "unknown"])] {
        assert_eq!(reply(&mut s, &json!({"sections":sections})).unwrap_err(), "listing cannot be combined with other sections");
    }
    for cells in [json!([]), json!([false]), json!(["Width"])] {
        assert_eq!(reply(&mut s, &json!({"sections":["listing"],"cells":cells})).unwrap_err(), "listing cannot be combined with cells");
    }
    assert_eq!(s.execute("shape.inspect", &json!({"page":1,"id":1,"sections":["listing"]})).unwrap_err().to_string(), "invalid inspection section");
}
#[test]
fn listing_preserves_budget_and_filter_errors() {
    let mut s = drawing();
    for p in [json!({"page":999}), json!({"ids":[1]}), json!({"page":1,"ids":[999]}), json!({"page":1,"ids":[1],"find":"Cedar"})] {
        let old = reply(&mut s, &p);
        assert!(old.is_err());
        assert_eq!(reply(&mut s, &listing(p)), old);
    }
    s.execute("shape.text", &json!({"page":1,"id":1,"text":"x".repeat(65530)})).unwrap();
    let old = reply(&mut s, &json!({"page":1}));
    assert!(old.as_ref().unwrap_err().contains("65536 byte budget"));
    assert_eq!(reply(&mut s, &listing(json!({"page":1}))), old);
}
