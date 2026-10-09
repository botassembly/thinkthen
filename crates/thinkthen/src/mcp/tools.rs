//! Exactly the ten judging tools; no administrative or acting capabilities.

use serde_json::{Value, json};

pub(super) type Tool = crate::RequestFunction;

impl Tool {
    fn description(self) -> &'static str {
        match self {
            Self::Decide => "Answer yes, no or unsure.",
            Self::Choose => "Pick one declared option, or null.",
            Self::Tag => "Return every applicable declared label.",
            Self::Score => "Place evidence on declared levels.",
            Self::Filter => "Keep records passing a yes/no question.",
            Self::Rank => "Order the complete records by the declared question or set.",
            Self::Find => "Select one unit from a complete set, or none.",
            Self::Annotate => "Ask an ordered ordinary question set for each record.",
            Self::Recognize => "Recognize caller-defined literal entities and physical text spans.",
            Self::Relate => "Judge relationships over the complete entity set.",
        }
    }
}

/// Complete output schema comes from the native owner, never an adapter-made
/// result/2 substitute. The same typed input object is enforced by admission.
pub(super) fn list(output_schema: &Value) -> Value {
    let tools: Vec<_> = Tool::ALL
        .into_iter()
        .map(|tool| {
            json!({
                "name":tool,"description":tool.description(),
                "inputSchema":input_schema(tool),"outputSchema":output_schema,
                "execution":{"taskSupport":"forbidden"}
            })
        })
        .collect();
    json!({"tools":tools})
}

fn input_schema(tool: Tool) -> Value {
    let canonical: Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../specification/request.schema.json"
    )))
    .unwrap_or_else(|_| json!({}));
    let mut options = canonical
        .pointer("/$defs/RequestOptions")
        .cloned()
        .unwrap_or_else(|| json!({}));
    if let Some(map) = options.get_mut("properties").and_then(Value::as_object_mut) {
        map.retain(|name, _| {
            tool.allows_option(name)
                && !matches!(
                    name.as_str(),
                    "details" | "examples" | "examples_field" | "seed_spans" | "seed_spans_field"
                )
        });
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
        "question":{"oneOf":[{"type":"string"},{"$ref":"#/$defs/RequestDefinition"}]},
        "question_file":{"type":"string","minLength":1},
        "question_name":{"type":"string","minLength":1},
        "question_reference":{"type":"string","minLength":1},
        "evidence":{"type":"string"},"records":{"type":"array"},
        "source":source_schema(false, tool.images(), &canonical),"options":options
    });
    if !matches!(tool, Tool::Decide | Tool::Filter | Tool::Rank | Tool::Find)
        && let Some(question) = properties.get_mut("question")
    {
        *question = json!({"$ref":"#/$defs/RequestDefinition"});
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
    let questions = [
        "question",
        "question_file",
        "question_name",
        "question_reference",
    ];
    let question_choices = questions.iter().map(|name| json!({"required":[name],"not":{"anyOf":questions.iter().filter(|other| *other != name).map(|other| json!({"required":[other]})).collect::<Vec<_>>()}})).collect::<Vec<_>>();
    let forms = ["evidence", "records", "source", "images", "inputs"];
    let input_choices = forms.iter().map(|name| {
        let others = forms.iter().filter(|other| *other != name && !(*name == "evidence" && **other == "images"));
        json!({"required":[name],"not":{"anyOf":others.map(|other| json!({"required":[other]})).collect::<Vec<_>>()}})
    }).collect::<Vec<_>>();
    json!({"$schema":"https://json-schema.org/draft/2020-12/schema","type":"object",
        "additionalProperties":false,"properties":properties,"$defs":canonical.get("$defs"),
        "allOf":[{"oneOf":question_choices},{"oneOf":input_choices}]})
}

fn descriptor_schema(tool: Tool, canonical: &Value) -> Value {
    let mut descriptor = json!({"type":"object","additionalProperties":false,"properties":{
        "text":{"type":"string"},"json":{},"source":source_schema(true, false, canonical),
        "context":canonical.pointer("/$defs/RequestItem/properties/context"),
        "options":{"$ref":"#/$defs/Authored_options"},
        "images":{"type":"array","maxItems":crate::MAX_IMAGES,"items":{"oneOf":[
            {"type":"string","minLength":1},
            {"type":"object","additionalProperties":false,"required":["path","media"],"properties":{
                "path":{"type":"string","minLength":1},"media":{"$ref":"#/$defs/ImageMedia"}}}
        ]}}
    },"oneOf":[
        {"required":["text"],"not":{"anyOf":[{"required":["json"]},{"required":["source"]}]}},
        {"required":["json"],"not":{"anyOf":[{"required":["text"]},{"required":["source"]}]}},
        {"required":["source"],"not":{"anyOf":[{"required":["text"]},{"required":["json"]}]}},
        {"required":["images"],"properties":{"images":{"minItems":1}},"not":{"anyOf":[{"required":["text"]},{"required":["json"]},{"required":["source"]}]}}
    ]});
    if let Some(map) = descriptor
        .get_mut("properties")
        .and_then(Value::as_object_mut)
    {
        if !tool.images() {
            map.remove("images");
        }
        if !tool.allows_option("options_field") {
            map.remove("options");
        }
    }
    if !tool.images()
        && let Some(choices) = descriptor.get_mut("oneOf").and_then(Value::as_array_mut)
    {
        choices.pop();
    }
    descriptor
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
    let mut source = json!({"type":"object","additionalProperties":false,"required":if file_only { vec!["paths","unit"] } else { vec!["paths"] },
        "properties":{"paths":{"type":"array","minItems":1,"items":{"type":"string","minLength":1}},
            "unit":unit,"window":{"type":"integer","minimum":1},"media":media}});
    if file_only
        && let Some(properties) = source.get_mut("properties").and_then(Value::as_object_mut)
    {
        properties.remove("window");
    }
    if !file_only && let Some(map) = source.as_object_mut() {
        map.insert(
            "allOf".into(),
            json!([
                {"if":{"properties":{"unit":{"const":"window"}},"required":["unit"]},
                    "then":{"required":["window"]}},
                {"if":{"required":["window"]},
                    "then":{"properties":{"unit":{"const":"window"}},"required":["unit"]}},
                {"if":{"properties":{"media":{"const":"image"}},"required":["media"]},
                    "then":{"properties":{"unit":{"const":"file"}},"required":["unit"]}}
            ]),
        );
    }
    source
}
