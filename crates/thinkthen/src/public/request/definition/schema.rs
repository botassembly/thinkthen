//! Reuse the authored structural grammar; native admission keeps semantic rules.
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "test-only schema projections use the committed authored schema checked by decoding fixtures"
)]
use schemars::{JsonSchema, Schema, SchemaGenerator};
use serde_json::{Value, json};

impl JsonSchema for super::RequestDefinition {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "RequestDefinition".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let mut authored: Value = serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../specification/question-file.schema.json"
        )))
        .unwrap();
        rewrite_refs(&mut authored);
        let mut definitions = authored["$defs"].as_object().unwrap().clone();
        definitions.remove("doorRequest");
        let recognize = recognize(
            &definitions,
            generator
                .subschema_for::<crate::RecognitionStageContext>()
                .to_value(),
            generator
                .subschema_for::<crate::RecognitionMode>()
                .to_value(),
        );
        let set = set(&definitions);
        for (name, definition) in definitions {
            generator
                .definitions_mut()
                .insert(format!("Authored_{name}"), definition);
        }
        let mut choices = authored["anyOf"].as_array().unwrap().clone();
        choices.extend([recognize, set]);
        Schema::try_from(json!({"anyOf":choices,
            "description":"Existing authored structural grammar. Native admission checks wording, thresholds, references and function applicability."})).unwrap()
    }
}
fn rewrite_refs(value: &mut Value) {
    match value {
        Value::Object(fields) => {
            for (name, held) in fields {
                if name == "$ref"
                    && let Some(reference) = held.as_str()
                {
                    *held = reference.replace("#/$defs/", "#/$defs/Authored_").into();
                } else {
                    rewrite_refs(held);
                }
            }
        }
        Value::Array(items) => items.iter_mut().for_each(rewrite_refs),
        _ => {}
    }
}
fn reference(name: &str) -> Value {
    json!({"$ref":format!("#/$defs/Authored_{name}")})
}
fn recognize(
    definitions: &serde_json::Map<String, Value>,
    stage_context: Value,
    mode: Value,
) -> Value {
    // Existing find declarations supply the shared metadata, route and pointers.
    let mut properties = definitions["find"]["properties"]
        .as_object()
        .unwrap()
        .clone();
    properties.remove("find");
    properties.insert("version".into(), json!({"const":1}));
    for name in ["threshold", "relation_threshold"] {
        properties.insert(name.into(), reference("cut"));
    }
    properties.insert(
        "recognize".into(),
        json!({"type":"object","additionalProperties":false,
        "properties":{
            "kinds":{"type":"object","additionalProperties":reference("description")},
            "relations":{"type":"array","items":reference("relation")},
            "instructions":reference("questionText"),
            "entity_definition":reference("questionText"),
            "snippet_pieces": schemars::schema_for!(crate::RequestOptions).to_value()["properties"]["snippet_pieces"].clone(),
            "stage_context":stage_context,
            "mode": mode
        }}),
    );
    json!({"type":"object","required":["version","recognize"],
        "additionalProperties":false,"properties":properties})
}
fn set(definitions: &serde_json::Map<String, Value>) -> Value {
    // Set members are the existing four atomic shapes without per-call controls.
    let members = ["decide", "choose", "tag", "score"].map(|name| {
        let mut member = definitions[name].clone();
        let properties = member["properties"].as_object_mut().unwrap();
        for name in ["batch", "model", "profile"] {
            properties.remove(name);
        }
        member
    });
    json!({"type":"object","required":["version","questions"],"additionalProperties":false,
    "properties":{
        "version":{"const":1},"threshold":reference("threshold"),"profile":reference("profile"),
        "batch":{},
        "questions":{"type":"object","minProperties":1,
            "additionalProperties":{"anyOf":members}}
    }})
}
