//! Keep the legacy schema graph intact while closing the complete readers.
use serde_json::{Map, Value, json};
use std::collections::BTreeSet;

fn name(original: &str) -> String {
    if original == "CompleteFacts" {
        return "completeFacts".into();
    }
    if original == "CompleteError" {
        return "completeCallError".into();
    }
    if original == "ErrorSnapshot" {
        return "completeError".into();
    }
    if original.starts_with("complete") {
        original.to_owned()
    } else {
        format!("complete{original}")
    }
}
pub(super) fn graph(definitions: &mut Map<String, Value>, root: &str) {
    let original = definitions.clone();
    let mut pending = vec![root.to_owned()];
    let mut visited = BTreeSet::new();
    while let Some(key) = pending.pop() {
        if !visited.insert(key.clone()) {
            continue;
        }
        let mut schema = original.get(&key).expect("registered reference").clone();
        references(&mut schema, &mut pending);
        close(&mut schema, false);
        definitions.insert(name(&key), schema);
    }
}
fn references(schema: &mut Value, pending: &mut Vec<String>) {
    match schema {
        Value::Object(object) => {
            if let Some(reference) = object.get_mut("$ref") {
                let key = reference
                    .as_str()
                    .expect("reference")
                    .strip_prefix("#/$defs/")
                    .expect("local reference");
                pending.push(key.to_owned());
                *reference = json!(format!("#/$defs/{}", name(key)));
            }
            for value in object.values_mut() {
                references(value, pending);
            }
        }
        Value::Array(array) => {
            for value in array {
                references(value, pending);
            }
        }
        _ => {}
    }
}
fn nonnull(schema: &mut Value) {
    if let Some(types) = schema.get_mut("type").and_then(Value::as_array_mut) {
        types.retain(|kind| kind != "null");
    }
    if let Some(choices) = schema.get_mut("anyOf").and_then(Value::as_array_mut) {
        choices.retain(|choice| choice.get("type") != Some(&json!("null")));
    }
}
fn close(schema: &mut Value, flattened: bool) {
    let Some(object) = schema.as_object_mut() else {
        return;
    };
    // JSON Schema does not enforce the Rust integer format names.
    let maximum = match object.get("format").and_then(Value::as_str) {
        Some("uint16") => Some(u64::from(u16::MAX)),
        Some("uint32") => Some(u64::from(u32::MAX)),
        Some("uint64" | "uint") => Some(u64::MAX),
        _ => None,
    };
    if let Some(maximum) = maximum {
        object.insert("maximum".into(), json!(maximum));
    }
    let required = object
        .get("required")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let has_properties = object.contains_key("properties");
    if let Some(properties) = object.get_mut("properties").and_then(Value::as_object_mut) {
        for (key, property) in properties {
            if !required.iter().any(|item| item == key) {
                nonnull(property);
            }
            close(property, false);
        }
    }
    for union in ["oneOf", "anyOf", "allOf"] {
        if let Some(choices) = object.get_mut(union).and_then(Value::as_array_mut) {
            for choice in choices {
                close(choice, has_properties || flattened);
            }
        }
    }
    for key in ["items", "additionalProperties"] {
        if let Some(child) = object.get_mut(key) {
            close(child, false);
        }
    }
    if !flattened && object.get("type") == Some(&json!("object")) && has_properties {
        object.insert("unevaluatedProperties".into(), json!(false));
    }
}
