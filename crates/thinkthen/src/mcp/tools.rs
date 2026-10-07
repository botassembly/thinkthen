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
        "question_reference":{"type":"string","minLength":1,"description":"Explicit native @ reference with working-directory path precedence."},
        "evidence":{"type":"string"},"records":{"type":"array"},
        "source":source_schema(false, tool.images()),
        "options":{"type":"object","additionalProperties":false,"properties":{
            "cancelled":{"type":"boolean","default":false,"description":"Initial native call cancellation state; true cancels before admission."},
            "deadline_ms":{"type":"integer","minimum":-1,"maximum":4294967295000u64},
            "max_requests_total":{"type":"integer","minimum":0},
            "batch":{"oneOf":[{"type":"integer","minimum":1},{"const":"max"}]},"context":{"type":"string"},
            "field":{"oneOf":[{"type":"string"},{"type":"array","items":{"type":"string"}}]},"context_field":{"type":"string"},"options_field":{"type":"string"},
            "proxy":{"description":"Reserved activation; native admission always refuses it."},
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
    if tool.images()
        && let Some(map) = properties.as_object_mut()
    {
        map.insert("images".into(), json!({"type":"array","minItems":1,"maxItems":crate::MAX_IMAGES,
            "items":{"type":"string","minLength":1},"description":"Explicit ordered attachments; duplicates survive."}));
    }
    if let Some(map) = properties.as_object_mut() {
        map.insert(
            "inputs".into(),
            json!({"type":"array","items":descriptor_schema(tool)}),
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
        "additionalProperties":false,"properties":properties,
        "allOf":[{"oneOf":question_choices},{"oneOf":input_choices}]})
}

fn descriptor_schema(tool: Tool) -> Value {
    let mut descriptor = json!({"type":"object","additionalProperties":false,"properties":{
        "text":{"type":"string"},"json":{},"source":source_schema(true, false),
        "context":{"oneOf":[{"type":"string"},{"type":"object"}]},
        "options":{"oneOf":[{"type":"array"},{"type":"object"}]},
        "images":{"type":"array","maxItems":crate::MAX_IMAGES,"items":{"oneOf":[
            {"type":"string","minLength":1},
            {"type":"object","additionalProperties":false,"required":["path","media"],"properties":{
                "path":{"type":"string","minLength":1},"media":{"enum":["image/jpeg","image/png"]}}}
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
        if tool != Tool::Choose {
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

fn source_schema(file_only: bool, images: bool) -> Value {
    let unit = if file_only {
        json!({"const":"file"})
    } else {
        json!({"enum":["line","window","file"],"default":"line"})
    };
    let media = if images {
        json!({"enum":["text","image"],"default":"text"})
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
    source
}
