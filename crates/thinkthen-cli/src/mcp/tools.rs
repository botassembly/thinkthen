//! Exactly the ten judging tools; no administrative or acting capabilities.

use serde_json::{Value, json};

pub(super) type Tool = crate::RequestFunction;

fn description(tool: Tool) -> &'static str {
    match tool {
        Tool::Decide => "Answer yes, no or unsure.",
        Tool::Choose => {
            "Pick one declared option, or null. question.options is an ordered list of at least two names or a name-to-description map."
        }
        Tool::Tag => {
            "Return every applicable declared label. question.labels is an ordered list of names or a name-to-description map."
        }
        Tool::Score => {
            "Place evidence on declared levels. question.levels is an ordered list of at least two names, lowest first, or a name-to-meaning map."
        }
        Tool::Filter => "Keep records passing a yes/no question.",
        Tool::Rank => "Order the complete records by the declared question or set.",
        Tool::Find => "Select one unit from a complete set, or none.",
        Tool::Annotate => {
            "Ask an ordered ordinary question set for each record. question is {version:1,questions:{name:{decide:question_text}}}; entries also accept choose, tag or score."
        }
        Tool::Recognize => {
            "Recognize caller-defined literal entities and physical text spans. question is {version:1,recognize:{kinds:{kind_name:description}}}."
        }
        Tool::Relate => {
            "Judge relationships over the complete entity set. question is {version:1,relate:{relations:[{name:relation_name}]}}; records hold entities with name and kind."
        }
    }
}

/// Compact discovery guidance; shared native admission owns the full grammar.
pub(super) fn list() -> Value {
    let tools: Vec<_> = Tool::ALL
        .into_iter()
        .map(|tool| {
            json!({
                "name":tool,"description":format!("{} Full argument grammar: thinkthen mcp --help.", description(tool)),
                "inputSchema":input_schema(tool),
                "execution":{"taskSupport":"forbidden"}
            })
        })
        .collect();
    json!({"tools":tools})
}

fn input_schema(tool: Tool) -> Value {
    let canonical: Value =
        serde_json::from_str(crate::request_schema()).unwrap_or_else(|_| json!({}));
    let mut options = json!({"type":"object","additionalProperties":false,
        "properties":canonical.pointer("/$defs/RequestOptions/properties").cloned().unwrap_or_else(|| json!({}))});
    if let Some(map) = options.get_mut("properties").and_then(Value::as_object_mut) {
        map.retain(|name, _| tool.allows_option(name));
        for schema in map.values_mut() {
            *schema = summary(schema, &canonical);
        }
        let field = map.get("field").cloned().unwrap_or_else(|| json!({}));
        map.insert("field".into(), json!({"oneOf":[{"type":"string"},field]}));
        map.insert(
            "cancelled".into(),
            json!({"type":"boolean","default":false}),
        );
        map.insert(
            "proxy".into(),
            json!({"description":"Reserved activation; native admission always refuses it."}),
        );
    }
    let mut properties = json!({
        "question":{"type":["string","object"]},
        "question_file":{"type":"string","minLength":1},
        "question_name":{"type":"string","minLength":1},
        "question_reference":{"type":"string","minLength":1},
        "evidence":{"type":"string"},"records":{"type":"array"},
        "source":source_schema(false, tool.images(), &canonical),"options":options
    });
    if let Some(question) = properties.get_mut("question") {
        *question = question_schema(tool, &canonical);
    }
    if tool.images()
        && let Some(map) = properties.as_object_mut()
    {
        map.insert("images".into(), json!({"type":"array","minItems":1,"maxItems":crate::MAX_IMAGES,
            "items":{"type":"string","minLength":1},"description":"Explicit ordered attachments; duplicates survive."}));
    }
    if let Some(map) = properties.as_object_mut() {
        map.insert(
            "inputs".into(),
            json!({"type":"array","items":descriptor_schema(tool, &canonical)}),
        );
    }
    json!({"type":"object","additionalProperties":false,"properties":properties})
}

fn question_schema(tool: Tool, canonical: &Value) -> Value {
    let function = match tool {
        Tool::Filter | Tool::Rank => Tool::Decide,
        _ => tool,
    };
    let name = function.name();
    let reference = format!("#/$defs/Authored_{name}");
    let definition = canonical
        .pointer("/$defs/RequestDefinition/anyOf")
        .and_then(Value::as_array)
        .and_then(|choices| {
            choices.iter().find(|choice| {
                choice.get("$ref").and_then(Value::as_str) == Some(reference.as_str())
                    || choice.pointer(&format!("/properties/{name}")).is_some()
                    || (tool == Tool::Annotate && choice.pointer("/properties/questions").is_some())
            })
        })
        .unwrap_or(&Value::Null);
    let mut object = question_fields(definition, canonical, 2);
    if tool == Tool::Rank {
        object = json!({"type":"object","description":"A decide or score question, or an ordered question set; see server help."});
    }
    if matches!(tool, Tool::Decide | Tool::Filter | Tool::Rank | Tool::Find) {
        json!({"anyOf":[{"type":"string"},object]})
    } else {
        object
    }
}

fn question_fields(schema: &Value, canonical: &Value, depth: u8) -> Value {
    if let Some(reference) = schema.get("$ref").and_then(Value::as_str)
        && let Some(target) = canonical.pointer(reference.trim_start_matches('#'))
    {
        return question_fields(target, canonical, depth);
    }
    let mut compact = summary(schema, canonical);
    if depth > 0
        && let Some(properties) = schema.get("properties").and_then(Value::as_object)
        && let Some(compact) = compact.as_object_mut()
    {
        let fields: serde_json::Map<_, _> = properties
            .iter()
            .map(|(name, schema)| (name.clone(), question_fields(schema, canonical, depth - 1)))
            .collect();
        compact.insert("properties".into(), Value::Object(fields));
        if let Some(required) = schema.get("required") {
            compact.insert("required".into(), required.clone());
        }
    }
    compact
}

/// Retain field names from the native owner without copying nested grammars.
fn summary(schema: &Value, canonical: &Value) -> Value {
    if let Some(reference) = schema.get("$ref").and_then(Value::as_str)
        && let Some(target) = canonical.pointer(reference.trim_start_matches('#'))
    {
        return summary(target, canonical);
    }
    let mut compact = serde_json::Map::new();
    for key in ["type", "enum", "const"] {
        if let Some(value) = schema.get(key) {
            compact.insert(key.into(), value.clone());
        }
    }
    for key in ["oneOf", "anyOf"] {
        if let Some(choices) = schema.get(key).and_then(Value::as_array) {
            compact.extend(choice_summary(choices, canonical));
        }
    }
    if schema.get("type") == Some(&json!("array")) {
        compact.insert("items".into(), json!({}));
    }
    Value::Object(compact)
}

fn choice_summary(choices: &[Value], canonical: &Value) -> serde_json::Map<String, Value> {
    let choices: Vec<_> = choices
        .iter()
        .map(|choice| summary(choice, canonical))
        .collect();
    let constants: Option<Vec<_>> = choices.iter().map(|choice| choice.get("const")).collect();
    if let Some(constants) = constants {
        return serde_json::Map::from_iter([("enum".into(), json!(constants))]);
    }
    let plain_types = choices.iter().all(|choice| {
        choice
            .as_object()
            .is_some_and(|fields| fields.keys().all(|key| key == "type" || key == "items"))
            && choice.get("type").is_some_and(Value::is_string)
    });
    if !plain_types {
        return serde_json::Map::from_iter([("anyOf".into(), json!(choices))]);
    }
    let mut types = Vec::new();
    for choice in &choices {
        let kind = choice.get("type").unwrap_or(&Value::Null);
        if !types.contains(&kind) {
            types.push(kind);
        }
    }
    serde_json::Map::from_iter([("type".into(), json!(types))])
}

fn descriptor_schema(tool: Tool, canonical: &Value) -> Value {
    let mut properties = canonical
        .pointer("/$defs/RequestItem/properties")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    properties.remove("original");
    properties.retain(|name, _| match name.as_str() {
        "images" => tool.images(),
        "options" => tool.allows_option("options_field"),
        "examples" | "seed_spans" => tool.allows_option("examples"),
        _ => true,
    });
    for schema in properties.values_mut() {
        *schema = summary(schema, canonical);
    }
    properties.insert("text".into(), json!({"type":"string"}));
    properties.insert("json".into(), json!({}));
    properties.insert("source".into(), source_schema(true, false, canonical));
    json!({"type":"object","additionalProperties":false,"properties":properties})
}

fn source_schema(file_only: bool, images: bool, canonical: &Value) -> Value {
    let unit = if file_only {
        json!({"const":"file"})
    } else {
        canonical
            .pointer("/$defs/RequestReader/properties/unit")
            .cloned()
            .unwrap_or_else(|| json!({}))
    };
    let media = if images {
        canonical
            .pointer("/$defs/RequestSource/properties/media")
            .cloned()
            .unwrap_or_else(|| json!({}))
    } else {
        json!({"const":"text","default":"text"})
    };
    let framing = canonical
        .pointer("/$defs/RequestSource/properties/framing")
        .cloned()
        .unwrap_or_else(|| json!({}));
    let mut source = json!({"type":"object","additionalProperties":false,"required":if file_only { vec!["paths","unit"] } else { vec!["paths"] },
        "properties":{"paths":{"type":"array","minItems":1,"items":{"type":"string","minLength":1}},
            "unit":summary(&unit, canonical),"window":{"type":"integer","minimum":1},"media":summary(&media, canonical),"framing":summary(&framing, canonical)},
        "description":"Ordered paths; window needs unit=window, images need unit=file."});
    if file_only
        && let Some(properties) = source.get_mut("properties").and_then(Value::as_object_mut)
    {
        properties.remove("window");
        properties.remove("framing");
    }
    if file_only && let Some(map) = source.as_object_mut() {
        map.remove("description");
    }
    source
}
