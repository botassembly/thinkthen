//! Canonical decoding and native header behavior at the request boundary.
use super::*;

#[test]
fn committed_request_schema_is_derived_from_deserialization_types() {
    let schema = schemars::generate::SchemaSettings::draft2020_12()
        .for_deserialize()
        .into_generator()
        .into_root_schema_for::<Request>();
    let text = serde_json::to_string_pretty(&schema).unwrap() + "\n";
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../specification/request.schema.json"
    );
    if std::fs::read_to_string(path).is_ok_and(|committed| committed == text) {
        return;
    }
    if std::env::var_os("THINKTHEN_WRITE_SCHEMA").is_some_and(|v| v == "1") {
        std::fs::write(path, text).unwrap();
    }
    panic!(
        "generated request schema differs; rewrite with THINKTHEN_WRITE_SCHEMA=1 cargo test -p thinkthen --lib committed_request_schema, then rerun"
    );
}
#[test]
fn canonical_objects_refuse_duplicate_unknown_and_null_controls() {
    let valid = r#"{"schema":"thinkthen.request/1","call":{"function":"decide","question":{"kind":"text","text":"Does it fit?"},"input":{"kind":"text","text":"yes"}}}"#;
    assert!(Request::from_json(valid).unwrap().admit().is_ok());
    for text in [
        valid.replace(
            "\"schema\":\"thinkthen.request/1\"",
            "\"schema\":\"thinkthen.request/2\"",
        ),
        valid.replace(
            "\"function\":\"decide\"",
            "\"function\":\"decide\",\"function\":\"decide\"",
        ),
        valid.replace(
            "\"kind\":\"text\",\"text\":\"yes\"",
            "\"kind\":\"text\",\"text\":\"yes\",\"secret\":true",
        ),
        valid.replace("\"input\":", "\"options\":{\"context\":null},\"input\":"),
        valid.replace(
            "\"input\":",
            "\"options\":{\"context\":\"\",\"context\":\"\"},\"input\":",
        ),
    ] {
        assert!(Request::from_json(&text).is_err(), "{text}");
    }
}

#[test]
fn generated_deserialization_schema_agrees_with_canonical_control_shapes() {
    use crate::test_deadline::child::ChildEnvironment as _;
    use std::io::Write as _;
    use std::process::{Command, Stdio};
    let valid = r#"{"schema":"thinkthen.request/1","call":{"function":"decide","question":{"kind":"text","text":"Fits?"},"input":{"kind":"text","text":"Alpha."}}}"#;
    let texts = [
        valid.to_owned(),
        valid.replace("\"input\":", "\"options\":{\"context\":\"\"},\"input\":"),
        valid.replace("thinkthen.request/1", "thinkthen.request/2"),
        valid.replace("\"input\":", "\"options\":{\"context\":null},\"input\":"),
        valid.replace("\"input\":", "\"options\":{\"examples\":null},\"input\":"),
        valid.replace("\"input\":", "\"options\":{\"unexpected\":true},\"input\":"),
        valid.replace(
            "\"kind\":\"text\",\"text\":\"Alpha.\"",
            "\"kind\":\"text\",\"text\":\"Alpha.\",\"extra\":true",
        ),
    ];
    let cases = texts
        .iter()
        .map(|text| {
            (
                serde_json::from_str::<serde_json::Value>(text).unwrap(),
                Request::from_json(text).is_ok(),
            )
        })
        .collect::<Vec<_>>();
    let mut child = Command::new("python3").clear_environment()
        .arg("-c").arg("import json,sys; from jsonschema import Draft202012Validator; schema=json.load(open(sys.argv[1])); Draft202012Validator.check_schema(schema); validator=Draft202012Validator(schema); cases=json.load(sys.stdin); assert all(validator.is_valid(value)==expected for value,expected in cases)")
        .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/../../specification/request.schema.json"))
        .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(&serde_json::to_vec(&cases).unwrap())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
