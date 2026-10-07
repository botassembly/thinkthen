//! Exactly the ten judging tools; no administrative or acting capabilities.

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub(super) enum Tool {
    Decide,
    Choose,
    Tag,
    Score,
    Filter,
    Rank,
    Find,
    Annotate,
    Recognize,
    Relate,
}

impl Tool {
    pub(super) const ALL: [Self; 10] = [
        Self::Decide,
        Self::Choose,
        Self::Tag,
        Self::Score,
        Self::Filter,
        Self::Rank,
        Self::Find,
        Self::Annotate,
        Self::Recognize,
        Self::Relate,
    ];
    pub(super) const fn images(self) -> bool {
        matches!(self, Self::Decide | Self::Choose | Self::Score)
    }
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
            Self::Recognize => "Recognize names and physical text spans.",
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
    let mut properties = json!({
        "question":{"oneOf":[{"type":"string"},{"type":"object"}],
            "description":"Literal text or the ordinary native question/set/plan JSON grammar; @ stays text."},
        "question_file":{"type":"string","minLength":1},
        "question_name":{"type":"string","minLength":1},
        "evidence":{"type":"string"},"records":{"type":"array"},
        "source":{"type":"object","additionalProperties":false,"required":["paths"],
            "properties":{"paths":{"type":"array","minItems":1,"items":{"type":"string","minLength":1}},
                "unit":{"enum":["line","window","file"],"default":"line"},
                "window":{"type":"integer","minimum":1},"media":{"enum":["text","image"],"default":"text"}}},
        "options":{"type":"object","additionalProperties":false,"properties":{
            "deadline_ms":{"type":"integer","minimum":-1,"maximum":4294967295000u64},
            "max_requests_total":{"type":"integer","minimum":0},
            "batch":{"type":"integer","minimum":1},"context":{"type":"string"},
            "field":{"type":"string"},"context_field":{"type":"string"},"options_field":{"type":"string"},
            "model":{"type":"string"},"attempts":{"type":"boolean"},
            "threshold":{"oneOf":[{"type":"number"},{"type":"string"}]},"top":{"type":"integer","minimum":0},"files_only":{"type":"boolean"},"none":{"type":"boolean"}}}
    });
    if !matches!(tool, Tool::Decide | Tool::Filter | Tool::Rank | Tool::Find)
        && let Some(question) = properties.get_mut("question")
    {
        *question = json!({"type":"object","description":"The ordinary native question, set or plan JSON grammar."});
    }
    if let Some(options) = properties
        .get_mut("options")
        .and_then(|value| value.get_mut("properties"))
        .and_then(Value::as_object_mut)
    {
        if !matches!(tool, Tool::Decide | Tool::Choose | Tool::Tag | Tool::Filter) {
            options.remove("threshold");
        }
        if tool != Tool::Choose {
            options.remove("options_field");
        }
        if tool != Tool::Rank {
            options.remove("top");
        }
        if tool != Tool::Filter {
            options.remove("files_only");
        }
        if tool != Tool::Find {
            options.remove("none");
        }
    }
    if tool.images() {
        if let Some(map) = properties.as_object_mut() {
            map.insert("images".into(), json!({"type":"array","minItems":1,"maxItems":crate::MAX_IMAGES,
            "items":{"type":"string","minLength":1},"description":"Explicit ordered attachments; duplicates survive."}));
        }
    } else {
        if let Some(media) = properties
            .get_mut("source")
            .and_then(|source| source.get_mut("properties"))
            .and_then(|properties| properties.get_mut("media"))
        {
            *media = json!({"const":"text","default":"text"});
        }
    }
    json!({"$schema":"https://json-schema.org/draft/2020-12/schema","type":"object",
    "additionalProperties":false,"properties":properties,
    "allOf":[
        {"oneOf":[{"required":["question"],"not":{"anyOf":[{"required":["question_file"]},{"required":["question_name"]}]}},
            {"required":["question_file"],"not":{"anyOf":[{"required":["question"]},{"required":["question_name"]}]}},
            {"required":["question_name"],"not":{"anyOf":[{"required":["question"]},{"required":["question_file"]}]}}]},
        {"oneOf":[{"required":["evidence"],"not":{"anyOf":[{"required":["records"]},{"required":["source"]}]}},
            {"required":["records"],"not":{"anyOf":[{"required":["evidence"]},{"required":["source"]},{"required":["images"]}]}},
            {"required":["source"],"not":{"anyOf":[{"required":["evidence"]},{"required":["records"]},{"required":["images"]}]}},
            {"required":["images"],"not":{"anyOf":[{"required":["evidence"]},{"required":["records"]},{"required":["source"]}]}}]}
    ]})
}
