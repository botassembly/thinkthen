//! Validate actual public calls with the same strict schema packaged for hosts.
use super::child::ChildEnvironment as _;
use super::*;
use serde::Serialize;
use std::io::Write;
use std::process::{Command, Stdio};

#[cfg(test)]
pub(super) fn call<T: Serialize>(call: &thinkthen::Call<T>, definition: &str) {
    check(
        &serde_json::to_value(call.complete().unwrap()).unwrap(),
        definition,
    );
}
#[cfg(test)]
pub(super) fn check(document: &Value, definition: &str) {
    check_batch(std::slice::from_ref(document), definition);
}
#[cfg(test)]
pub(super) fn check_batch(documents: &[Value], definition: &str) {
    let script = r#"
import json, sys
from jsonschema import Draft202012Validator
case = json.load(sys.stdin)
schema = case['schema']
Draft202012Validator.check_schema(schema)
validator = Draft202012Validator(schema)
value_validator = Draft202012Validator({'$ref':'#/$defs/'+case['definition'], '$defs':schema['$defs']}) if case['definition'] else None
for document in case['documents']:
    validator.validate(document)
    if value_validator:
        rows = document['value']
        if not isinstance(rows, list): rows = [rows]
        for row in rows: value_validator.validate(row)
"#;
    let mut child = Command::new("python3")
        .clear_environment()
        .env("PATH", std::env::var_os("PATH").unwrap_or_default())
        .args(["-c", script])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let input = json!({"schema":serde_json::from_str::<Value>(thinkthen::complete_call_schema()).unwrap(),"documents":documents,"definition":definition});
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.to_string().as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{definition}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
