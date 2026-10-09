//! The result schema, derived from the Rust types that serialize each result.
//!
//! ADR 0112: `specification/result.schema.json` is this test's output. On a
//! difference the test fails. With `THINKTHEN_WRITE_SCHEMA=1` it rewrites the
//! file and still fails, so a green run always compares.

mod complete;
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
use crate::public::{
    Counters, DoorReply, ErrorKind, Picked, PlanEstimate, QuestionJson, RankedRow,
};

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

fn generated(reference: &str) -> String {
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
        (
            "rank",
            generator.subschema_for::<Vec<RankedRow<'_>>>().to_value(),
        ),
        (
            "find",
            generator.subschema_for::<Option<Picked<'_>>>().to_value(),
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
    register(&mut generator);
    complete::register(&mut generator);
    let mut definitions: Map<String, Value> = generator.take_definitions(true);
    complete::finish(&mut definitions);
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
    if reference == "completeCall" {
        definitions.retain(|name, _| name.starts_with("complete"));
    }
    let root = json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": if reference == "completeCall" { "urn:thinkthen:complete-call" } else { "urn:thinkthen:result" },
        "title": "thinkthen native results and compatibility structures",
        "description": format!("Generated from the Rust types that serialize each result; do not edit. Rewrite with: {REWRITE}. Validate a C door value against its verb's definition; released definitions retain result/1 and complete definitions describe result/2 and its additive call envelope."),
        "$ref": format!("#/$defs/{reference}"),
        "$defs": definitions,
    });
    serde_json::to_string_pretty(&root).expect("schema text") + "\n"
}

fn register(generator: &mut schemars::SchemaGenerator) {
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
        generator.subschema_for::<QuestionJson<'_>>(),
        generator.subschema_for::<PlanEstimate>(),
    ];
}

#[test]
fn the_committed_result_schema_is_the_one_the_rust_types_derive() {
    let schemas = [
        (FILE, generated("detailed")),
        (
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/public/results/complete.schema.json"
            ),
            generated("completeCall"),
        ),
    ];
    let mut changed = false;
    for (path, schema) in schemas {
        if std::fs::read_to_string(path).is_ok_and(|committed| committed == schema) {
            continue;
        }
        changed = true;
        if std::env::var_os("THINKTHEN_WRITE_SCHEMA").is_some_and(|value| value == "1") {
            std::fs::write(path, schema).expect("rewrite the result schema");
        }
    }
    assert!(
        !changed,
        "generated result schemas differ; rewrite with {REWRITE}, then rerun"
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let status = crate::test_deadline::child::command("python3", &[])
        .env("LC_ALL", "C.UTF-8")
        .arg("-I")
        .arg(root.join("sdlc/generators/results/generate.py"))
        .arg("--check")
        .status()
        .expect("run the result declaration generator");
    assert!(status.success(), "generated host results differ from Rust");
}

#[test]
fn complete_plan_schema_reads_native_previews_and_requires_the_nullable_body() {
    use std::io::Write as _;
    use std::process::Stdio;

    let engine = crate::Engine::builder().no_cache().build().expect("engine");
    let question = crate::Question::decide("Fits?").expect("question").cut();
    let mut cases = Vec::new();
    for records in [vec!["Alpha."], vec![]] {
        let plan = engine.plan(&question, records).expect("native plan");
        let value = serde_json::to_value(plan).expect("serialized plan");
        cases.push((value.clone(), true));
        let mut missing = value;
        missing
            .as_object_mut()
            .expect("plan object")
            .remove("first_body_utf8");
        cases.push((missing, false));
    }
    let mut schema: Value = serde_json::from_str(&generated("completeCall")).expect("schema");
    schema["$ref"] = json!("#/$defs/completeplan");
    let mut child = crate::test_deadline::child::command("python3", &[])
        .arg("-c")
        .arg("import json,sys; from jsonschema import Draft202012Validator; schema,cases=json.load(sys.stdin); Draft202012Validator.check_schema(schema); validator=Draft202012Validator(schema); assert all(validator.is_valid(value)==expected for value,expected in cases), 'native plan presence contract'")
        .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped())
        .spawn().expect("schema validator");
    child
        .stdin
        .take()
        .expect("validator input")
        .write_all(&serde_json::to_vec(&(schema, cases)).expect("cases"))
        .expect("write cases");
    let output = child.wait_with_output().expect("validator output");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
