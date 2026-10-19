//! Human-readable schemas are part of the public command contract.
use misterdiagram_engine::{Session, command_specs};
use serde_json::{Value, json};
fn check(schema: &Value, path: &str) {
    if let Some(props) = schema.get("properties").and_then(Value::as_object) {
        for (name, rule) in props {
            assert!(
                rule.get("description").and_then(Value::as_str).is_some_and(|s| !s.trim().is_empty()),
                "missing parameter description: {path}.{name}"
            );
            check(rule, &format!("{path}.{name}"));
        }
    }
    if let Some(items) = schema.get("items") {
        check(items, &format!("{path}[]"));
    }
}
#[test]
fn every_command_parameter_has_a_description_and_serializes_as_an_object() {
    for spec in command_specs() {
        check(&serde_json::from_str::<Value>(spec.params).unwrap(), spec.id);
    }
    let commands = Session::new().execute("engine.commands", &json!({})).unwrap();
    for command in commands.as_array().unwrap() {
        assert!(command["params"].is_object(), "{}", command["id"]);
        check(&command["params"], command["id"].as_str().unwrap());
    }
    let param = |id: &str, key: &str| -> String {
        commands.as_array().unwrap().iter().find(|c| c["id"] == id).unwrap()["params"]["properties"][key]["description"].as_str().unwrap().into()
    };
    assert!(param("shape.draw.rect", "x").contains("bottom-left"));
    assert!(param("shape.draw.rect", "y").contains("upward"));
    assert!(param("master.drop", "x").contains("pin"));
    assert!(param("format.line", "weight").contains("inches"));
    assert!(param("format.text", "size").contains("points"));
}
