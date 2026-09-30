//! The result schema, derived from the Rust types that serialize each result.
//!
//! ADR 0112: `specification/result.schema.json` is this test's output. On a
//! difference the test fails. With `THINKTHEN_WRITE_SCHEMA=1` it rewrites the
//! file and still fails, so a green run always compares.

use schemars::Schema;
use schemars::generate::SchemaSettings;
use schemars::transform::RecursiveTransform;
use serde_json::{Map, Value, json};

use crate::cli::annotate::error_row::Row;
use crate::cli::recognize::Detailed;
use crate::cli::relate::result::Details;
use crate::core::{
    AnnotateResult, DecisionResult, FindResult, NamedValues, RecordValue, RelationEdge,
    RelationEntity, Value as Bare,
};
use crate::engine::facade::Recognized;
use crate::public::{Counters, DoorReply, ErrorKind};

/// One failed call as a JSON reader would see it: its kind, the retry signal,
/// and a message safe to log. No surface prints it yet, so it lives only in
/// the schema.
#[derive(schemars::JsonSchema)]
#[schemars(rename = "callError")]
#[expect(
    dead_code,
    reason = "the schema reads the fields; no surface builds one"
)]
struct CallError {
    kind: ErrorKind,
    retryable: bool,
    message: String,
}

const FILE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../specification/result.schema.json"
);
const REWRITE: &str = "THINKTHEN_WRITE_SCHEMA=1 cargo test -p thinkthen --lib schema_tests";

/// A union of named definitions, for an output no one Rust type prints.
fn any_of(names: &[&str]) -> Value {
    let references: Vec<Value> = names
        .iter()
        .map(|name| json!({"$ref": format!("#/$defs/{name}")}))
        .collect();
    json!({"anyOf": references})
}

/// A decision row's answer kind follows its question's verb.
fn paired(details: &mut Value) {
    let pairs = [
        ("decide", "yes_no"),
        ("choose", "choice"),
        ("tag", "tag"),
        ("score", "score"),
    ];
    let rules: Vec<Value> = pairs
        .into_iter()
        .map(|(verb, kind)| {
            json!({
                "if": {"properties": {"question": {"properties": {"verb": {"const": verb}}}}},
                "then": {"properties": {"answer": {"properties": {"kind": {"const": kind}}}}}
            })
        })
        .collect();
    details["allOf"] = rules.into();
}

fn generated() -> String {
    let mut generator = SchemaSettings::draft2020_12()
        .for_serialize()
        .with_transform(RecursiveTransform(|schema: &mut Schema| {
            schema.remove("title");
            schema.remove("description");
        }))
        .into_generator();
    // The bare value each verb prints at the C door, from the type it prints.
    let edges = generator.subschema_for::<Vec<RelationEdge<RelationEntity>>>();
    let named = [
        (
            "decide",
            generator.subschema_for::<Option<bool>>().to_value(),
        ),
        (
            "choose",
            generator.subschema_for::<Option<String>>().to_value(),
        ),
        ("tag", generator.subschema_for::<Vec<String>>().to_value()),
        ("score", generator.subschema_for::<f64>().to_value()),
        (
            "filter",
            generator.subschema_for::<Vec<String>>().to_value(),
        ),
        ("rank", generator.subschema_for::<Vec<String>>().to_value()),
        (
            "find",
            generator.subschema_for::<Option<String>>().to_value(),
        ),
        (
            "annotate",
            generator.subschema_for::<Vec<NamedValues>>().to_value(),
        ),
        (
            "relate",
            json!({"type": "object", "required": ["edges"], "properties": {"edges": edges}}),
        ),
        ("details", any_of(&["decisionDetails", "findDetails"])),
        (
            "detailed",
            any_of(&[
                "details",
                "annotateDetails",
                "recognizeDetails",
                "relateDetails",
            ]),
        ),
    ];
    // Each result type below is a named definition.
    let _registered = [
        generator.subschema_for::<Recognized>(),
        generator.subschema_for::<DecisionResult>(),
        generator.subschema_for::<FindResult>(),
        generator.subschema_for::<AnnotateResult>(),
        generator.subschema_for::<Detailed<'_>>(),
        generator.subschema_for::<Details<'_>>(),
        generator.subschema_for::<RecordValue<Bare>>(),
        generator.subschema_for::<Row<'_>>(),
        generator.subschema_for::<Counters>(),
        generator.subschema_for::<DoorReply>(),
        generator.subschema_for::<CallError>(),
    ];
    let mut definitions: Map<String, Value> = generator.take_definitions(true);
    let decision = definitions
        .get_mut("decisionDetails")
        .expect("decision details");
    paired(decision);
    // The C door's record rows always carry the record they answered.
    let mut record = decision.clone();
    record["required"]
        .as_array_mut()
        .expect("required")
        .push("input".into());
    for (name, schema) in named.into_iter().chain([("callRecordDetails", record)]) {
        assert!(
            definitions.insert(name.to_owned(), schema).is_none(),
            "{name} is named twice"
        );
    }
    let root = json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "urn:thinkthen:result",
        "title": "thinkthen result and JSON door structures",
        "description": format!("Generated from the Rust types that serialize each result; do not edit. Rewrite with: {REWRITE}. Validate a C door value against its verb's definition; the root checks one detailed thinkthen.result/1 row."),
        "$ref": "#/$defs/detailed",
        "$defs": definitions,
    });
    serde_json::to_string_pretty(&root).expect("schema text") + "\n"
}

#[test]
fn the_committed_result_schema_is_the_one_the_rust_types_derive() {
    let schema = generated();
    if std::fs::read_to_string(FILE).is_ok_and(|committed| committed == schema) {
        return;
    }
    if std::env::var_os("THINKTHEN_WRITE_SCHEMA").is_some_and(|value| value == "1") {
        std::fs::write(FILE, schema).expect("rewrite the result schema");
        panic!("schema rewritten; rerun");
    }
    panic!(
        "specification/result.schema.json differs from the Rust types; rewrite it with: {REWRITE}"
    );
}
