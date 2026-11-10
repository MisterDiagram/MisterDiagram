//! Compact registry queries preserve the literal schema's property order.
use crate::{Result, Session, operation};
use serde::{
    Deserialize, Deserializer,
    de::{IgnoredAny, MapAccess, Visitor},
};
use serde_json::{Value, json};

#[derive(Default)]
struct Names(Vec<String>);
impl<'de> Deserialize<'de> for Names {
    fn deserialize<D: Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        struct PropertyVisitor;
        impl<'de> Visitor<'de> for PropertyVisitor {
            type Value = Names;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("schema properties")
            }
            fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> std::result::Result<Names, M::Error> {
                let mut names = Vec::new();
                while let Some(name) = map.next_key::<String>()? {
                    map.next_value::<IgnoredAny>()?;
                    names.push(name);
                }
                Ok(Names(names))
            }
        }
        d.deserialize_map(PropertyVisitor)
    }
}
#[derive(Deserialize)]
struct Schema {
    #[serde(default)]
    properties: Names,
    #[serde(default)]
    required: Vec<String>,
}
pub(crate) fn commands(s: &mut Session, p: &Value) -> Result<Value> {
    let summary = p.get("detail").and_then(Value::as_str) == Some("summary");
    let registry = s.commands();
    let selected = match p.get("ids") {
        None => registry.iter().collect::<Vec<_>>(),
        Some(Value::Array(ids)) if !ids.is_empty() && ids.len() <= 128 => ids
            .iter()
            .map(|id| {
                let id = id.as_str().ok_or_else(|| operation("command ids must be strings"))?;
                registry.iter().find(|c| c.id == id).ok_or_else(|| operation(format!("unknown command: {id}")))
            })
            .collect::<Result<Vec<_>>>()?,
        _ => return Err(operation("ids must contain 1 through 128 command ids")),
    };
    selected
        .into_iter()
        .map(|c| {
            if !summary {
                return serde_json::to_value(c).map_err(operation);
            }
            let schema: Schema = serde_json::from_str(c.params).map_err(operation)?;
            let mut value = json!({"id":c.id,"label":c.label,"params":schema.properties.0,"required":schema.required});
            if !c.enabled {
                value["enabled"] = json!(false);
                value["disabled_reason"] = json!(c.disabled_reason);
            }
            Ok(value)
        })
        .collect::<Result<Vec<_>>>()
        .map(Value::Array)
}
