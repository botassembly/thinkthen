//! Result/2 definitions derive the concrete native serialization documents.
use crate::core::complete_documents::{annotation, find, recognize, relate, wire};
use schemars::SchemaGenerator;
use serde_json::{Map, Value, json};
mod strict;

pub(super) fn register(generator: &mut SchemaGenerator) {
    let _registered = [
        generator.subschema_for::<wire::AtomicDocument<'_, Value>>(),
        generator.subschema_for::<annotation::Document<'_, Value>>(),
        generator.subschema_for::<find::Document<'_, Value>>(),
        generator.subschema_for::<recognize::Document<'_, Value>>(),
        generator.subschema_for::<relate::Document<'_, Value>>(),
        generator
            .subschema_for::<crate::public::CompleteCall<'_, wire::AtomicDocument<'_, Value>>>(),
        generator.subschema_for::<crate::public::batch::CompletedPrefix<'_, wire::AtomicDocument<'_, Value>>>(),
        generator.subschema_for::<crate::public::CompleteError<'_>>(),
        generator.subschema_for::<crate::public::Surface>(),
        generator.subschema_for::<crate::cli::intake::Position>(),
        generator.subschema_for::<crate::cli::intake::SourceFields<'_>>(),
        generator.subschema_for::<crate::cli::relate::source::Endpoint<'_>>(),
    ];
}
pub(super) fn finish(definitions: &mut Map<String, Value>) {
    super::paired(
        definitions
            .get_mut("completeAtomic")
            .expect("complete atomic"),
    );
    let mut call = definitions["CompleteCall"].clone();
    call["properties"]["value"] = json!({"anyOf": [
        {"$ref":"#/$defs/completeDetailed"},
        {"type":"array", "items":{"$ref":"#/$defs/completeDetailed"}}
    ]});
    definitions.insert("completeCallSuccess".into(), call);
    definitions.insert(
        "completeDetailed".into(),
        super::any_of(&[
            "completeAtomic",
            "completeAnnotation",
            "completeFind",
            "completeRecognition",
            "completeRelation",
        ]),
    );
    definitions.insert(
        "completeCall".into(),
        super::any_of(&[
            "completeCallSuccess",
            "CompleteError",
            "completeBatchFailure",
        ]),
    );
    definitions["completeBatchFailure"]["properties"]["completed"] =
        json!({"type":"array", "items":{"$ref":"#/$defs/completeDetailed"}});
    // Strict complete readers have their own derived graph; released definitions
    // keep their compatibility spelling and permissiveness.
    strict::graph(definitions, "completeCall");
    strict::graph(definitions, "Surface");
    definitions
        .get_mut("completeUsage")
        .expect("reported usage")["anyOf"] =
        json!([{"required":["input_tokens"]},{"required":["output_tokens"]}]);
    let meta = definitions.get_mut("completeMeta").expect("complete meta");
    meta["oneOf"] = json!([
        {"required":["question_sha256"],"not":{"required":["questions_sha256"]}},
        {"required":["questions_sha256"],"not":{"required":["question_sha256"]}}
    ]);
    meta["properties"]["context_sha256"]["pattern"] = json!("^[0-9a-f]{64}$");
    meta["properties"]["requests"]["items"]["pattern"] = json!("^[0-9a-f]{64}$");
    meta["allOf"] = json!([{
        "if":{"properties":{"observations":{"maxItems":0}}},
        "then":{
            "properties":{
                "origin":{"type":"null"}, "cached":{"const":false}, "requests_sent":{"const":0},
                "question_sources":{"maxItems":0}, "requests":{"maxItems":0}
            }, "not":{"required":["answered_by"]}
        },
        "else":{"properties":{"origin":{"type":"string"}}}
    }]);
    let find = definitions
        .get_mut("completeReadableQuestion2")
        .expect("find reading");
    find["properties"]["verb"] = json!({"const":"find"});
    find["required"]
        .as_array_mut()
        .expect("required")
        .push(json!("verb"));
    let find_answer = definitions
        .get_mut("completefindAnswer")
        .expect("find answer");
    find_answer["properties"]["kind"] = json!({"const":"find"});
    find_answer["required"]
        .as_array_mut()
        .expect("required")
        .push(json!("kind"));
    let relate = definitions
        .get_mut("completeReadableQuestion4")
        .expect("relate reading");
    relate["properties"]["verb"] = json!({"const":"relate"});
    let attempt = definitions
        .get_mut("completeAttempt")
        .expect("complete attempt");
    attempt["properties"]["ordinal"]["minimum"] = json!(1);
    attempt["properties"]["request_sha256"]["pattern"] = json!("^[0-9a-f]{64}$");
    locations(definitions);
    rank_members(definitions);
    functions(definitions);
}
fn locations(definitions: &mut Map<String, Value>) {
    let source = definitions
        .get_mut("completePhysicalSource")
        .expect("physical source");
    source["properties"]["first_line"]["minimum"] = json!(1);
    source["properties"]["last_line"]["minimum"] = json!(1);
    source["dependentRequired"] = json!({"first_line":["last_line"],"last_line":["first_line"]});
    for name in [
        "completeAtomic",
        "completeAnnotation",
        "completeFind",
        "completeRecognition",
        "completeRelation",
    ] {
        let row = definitions.get_mut(name).expect("row");
        // These existing located-row fields remain presentation only.
        row["properties"]["position"] = json!({"$ref":"#/$defs/completePosition"});
        if name == "completeAtomic" {
            row["properties"]["input_file"] = json!({"type":["string","null"]});
        }
    }
    strict::graph(definitions, "SourceFields");
    let fields = definitions["completeSourceFields"]["properties"]
        .as_object()
        .expect("source coordinates")
        .clone();
    for name in [
        "completeAtomic",
        "completeAnnotation",
        "completeFind",
        "completeRecognition",
        "completeRelation",
    ] {
        let properties = definitions.get_mut(name).expect("row")["properties"]
            .as_object_mut()
            .expect("properties");
        properties.extend(fields.clone());
    }
    definitions
        .get_mut("completeentity")
        .expect("recognized entity")["properties"]
        .as_object_mut()
        .expect("entity properties")
        .extend(fields);
    strict::graph(definitions, "sourceRelationEndpoint");
    for endpoint in ["source", "target"] {
        definitions
            .get_mut("completerelatedEntityEdge")
            .expect("relation edge")["properties"][endpoint] =
            super::any_of(&["completerelatedEntity", "completesourceRelationEndpoint"]);
    }
    definitions.insert("completePosition".into(), definitions["Position"].clone());
    strict::graph(definitions, "completePosition");
}
fn functions(definitions: &mut Map<String, Value>) {
    for (name, verb, value, threshold, input) in [
        (
            "completeDecide",
            "decide",
            json!(true),
            json!({"$ref":"#/$defs/completethreshold"}),
            false,
        ),
        (
            "completeChoose",
            "choose",
            json!({"type":["string","null"]}),
            json!({"$ref":"#/$defs/completethreshold"}),
            false,
        ),
        (
            "completeTag",
            "tag",
            json!({"type":"array","items":{"type":"string"}}),
            json!({"$ref":"#/$defs/completethreshold"}),
            false,
        ),
        (
            "completeScore",
            "score",
            json!({"type":"number"}),
            json!({"type":"null"}),
            false,
        ),
        (
            "completeFilter",
            "decide",
            json!({"type":"boolean"}),
            json!({"$ref":"#/$defs/completethreshold"}),
            true,
        ),
        (
            "completeRank",
            "",
            json!({"type":"integer","minimum":1}),
            json!({"type":"null"}),
            true,
        ),
    ] {
        let mut row = definitions["completeAtomic"].clone();
        if name != "completeRank" {
            row["properties"]
                .as_object_mut()
                .expect("properties")
                .remove("members");
        }
        row["properties"]["value"] = value;
        if matches!(name, "completeScore" | "completeRank") {
            row["properties"]["threshold"] = threshold;
        }
        if !verb.is_empty() {
            row["allOf"]
                .as_array_mut()
                .expect("paired rules")
                .push(json!({"properties":{"question":{"properties":{"verb":{"const":verb}}}}}));
        }
        if input {
            row["required"]
                .as_array_mut()
                .expect("required")
                .push(json!("input"));
        }
        definitions.insert(name.into(), row);
    }
}

fn rank_members(definitions: &mut Map<String, Value>) {
    // A member serializes a standalone judgment and never another set or original.
    let mut child = definitions["completeAtomic"].clone();
    let properties = child["properties"]
        .as_object_mut()
        .expect("member properties");
    for field in ["members", "question_name", "input", "index"] {
        properties.remove(field);
    }
    properties.insert("value".into(), json!({"type":"integer","minimum":1}));
    properties.insert("threshold".into(), json!({"type":"null"}));
    child["allOf"]
        .as_array_mut()
        .expect("paired rules")
        .push(json!({
            "properties":{
                "question":{"properties":{"verb":{"const":"decide"}}},
                "answer":{"properties":{"kind":{"const":"yes_no"}}}
            }
        }));
    definitions.insert("completeRankMemberResult".into(), child);
    let member = definitions.get_mut("completeRankMember").expect("member");
    member["properties"]["name"]["minLength"] = json!(1);
    member["properties"]["result"] = json!({"$ref":"#/$defs/completeRankMemberResult"});
    let parent = definitions.get_mut("completeAtomic").expect("atomic");
    parent["properties"]["members"] = json!({
        "type":"array","minItems":1,"items":{"$ref":"#/$defs/completeRankMember"}
    });
    parent["allOf"]
        .as_array_mut()
        .expect("paired rules")
        .push(json!({
            "if":{"required":["members"]},
            "then":{
                "required":["question_name"],
                "properties":{
                    "question_name":{"type":"string","minLength":1},
                    "value":{"type":"integer","minimum":1},
                    "threshold":{"type":"null"},
                    "question":{"properties":{"verb":{"const":"decide"}}},
                    "answer":{"properties":{"kind":{"const":"yes_no"}}}
                }
            }
        }));
}
