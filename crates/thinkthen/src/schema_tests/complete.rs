//! Result/2 definitions derive the concrete native serialization documents.
use crate::core::complete_documents::{annotation, find, recognize, relate, wire};
use schemars::SchemaGenerator;
use serde_json::{Map, Value, json};
mod strict;

pub(super) fn register(generator: &mut SchemaGenerator) {
    let _registered = [
        generator.subschema_for::<wire::AtomicDocument<'_, Value>>(),
        generator.subschema_for::<wire::AtomicDocument<'_, Value, wire::DecideValue<'_>>>(),
        generator.subschema_for::<wire::AtomicDocument<'_, Value, Option<&str>>>(),
        generator.subschema_for::<wire::AtomicDocument<'_, Value, &[String]>>(),
        generator.subschema_for::<wire::AtomicDocument<'_, Value, f64>>(),
        generator.subschema_for::<wire::AtomicDocument<'_, Value, bool>>(),
        generator.subschema_for::<wire::AtomicDocument<'_, Value, std::num::NonZeroUsize>>(),
        generator.subschema_for::<annotation::Document<'_, Value>>(),
        generator.subschema_for::<find::Document<'_, Value>>(),
        generator.subschema_for::<recognize::Document<'_, Value>>(),
        generator.subschema_for::<relate::Document<'_, Value>>(),
        generator
            .subschema_for::<crate::public::CompleteCall<'_, wire::AtomicDocument<'_, Value>>>(),
        generator.subschema_for::<crate::public::batch::CompletedPrefix<'_, wire::AtomicDocument<'_, Value>>>(),
        generator.subschema_for::<crate::SessionPacketDocument<'_>>(),
        generator.subschema_for::<crate::public::CompleteError<'_>>(),
        generator.subschema_for::<crate::public::Surface>(),
        generator.subschema_for::<crate::cli::intake::Position>(),
        generator.subschema_for::<crate::cli::intake::SourceFields<'_>>(),
        generator.subschema_for::<crate::cli::relate::source::Endpoint<'_>>(),
    ];
}
pub(super) fn finish(definitions: &mut Map<String, Value>) {
    definitions.insert(
        "completeAtomic".into(),
        definitions["completeAtomic_DecideValue"].clone(),
    );
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
    strict::graph(definitions, "plan");
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
    flatten_packet(definitions);
    strict::graph(definitions, "sessionPacket");
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
    for (name, verb, document, input) in [
        (
            "completeDecide",
            "decide",
            "completeAtomic_DecideValue",
            false,
        ),
        (
            "completeChoose",
            "choose",
            "completeAtomic_Nullable_string",
            false,
        ),
        (
            "completeTag",
            "tag",
            "completeAtomic_Array_of_string",
            false,
        ),
        ("completeScore", "score", "completeAtomic_double", false),
        ("completeFilter", "decide", "completeAtomic_boolean", true),
        ("completeRank", "", "completeAtomic_NonZeroUsize", true),
    ] {
        strict::graph(definitions, document);
        let mut row = definitions[document].clone();
        row["allOf"] = definitions["completeAtomic"]["allOf"].clone();
        let common = definitions["completeAtomic"]["properties"]
            .as_object()
            .expect("atomic properties");
        let properties = row["properties"].as_object_mut().expect("properties");
        for (field, schema) in common {
            if !properties.contains_key(field) {
                properties.insert(field.clone(), schema.clone());
            }
        }
        if name != "completeRank" {
            row["properties"]
                .as_object_mut()
                .expect("properties")
                .remove("members");
        }
        if matches!(name, "completeScore" | "completeRank") {
            row["properties"]["threshold"] = json!({"type":"null"});
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
    strict::graph(definitions, "completeAtomic_NonZeroUsize");
    let mut child = definitions["completeAtomic_NonZeroUsize"].clone();
    super::paired(&mut child);
    let properties = child["properties"]
        .as_object_mut()
        .expect("member properties");
    for field in ["members", "question_name", "input", "index"] {
        properties.remove(field);
    }
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

fn flatten_packet(definitions: &mut Map<String, Value>) {
    let alternatives = definitions["sessionPacket"]["oneOf"]
        .as_array()
        .expect("packet variants")
        .clone();
    let mut flattened = Vec::new();
    for mut packet in alternatives {
        let Some(reference) = packet.get("$ref").and_then(Value::as_str) else {
            flattened.push(packet);
            continue;
        };
        let name = reference
            .strip_prefix("#/$defs/")
            .expect("packet reference");
        let members = definitions[name]["oneOf"]
            .as_array()
            .expect("function variants");
        packet.as_object_mut().expect("packet").remove("$ref");
        for member in members {
            let mut merged = member.clone();
            merged["properties"]
                .as_object_mut()
                .expect("function properties")
                .extend(
                    packet["properties"]
                        .as_object()
                        .expect("packet properties")
                        .clone(),
                );
            merged["required"]
                .as_array_mut()
                .expect("function required")
                .extend(
                    packet["required"]
                        .as_array()
                        .expect("packet required")
                        .clone(),
                );
            flattened.push(merged);
        }
    }
    definitions["sessionPacket"]["oneOf"] = flattened.into();
}
