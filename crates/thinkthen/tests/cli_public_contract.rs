#![cfg(feature = "command")]
#![cfg(test)]
//! Command translation of shared usage failures.
mod support;

#[test]
fn malformed_annotation_and_missing_key_exit_as_usage() {
    for (arguments, code) in [
        (vec!["annotate", "{", "--adapter", "stub"], "usage.question"),
        (vec!["decide", "Ready?"], "usage.key-missing"),
    ] {
        let output = support::command().args(arguments).output().unwrap();
        assert_eq!(output.status.code(), Some(2));
        assert_eq!(support::reported(&output), code);
    }
}

#[test]
fn invalid_native_input_keeps_the_local_exit() {
    use std::io::Write;
    use std::process::Stdio;
    let cases: serde_json::Value = serde_json::from_str(include_str!(
        "../../../conformance/public-contract/boundaries.json"
    ))
    .unwrap();
    let case = &cases["input"];
    let input: Vec<u8> = case["bytes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u8)
        .collect();
    let mut child = support::command()
        .args(["decide", "Ready?", "--adapter", "stub"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(&input).unwrap();
    let output = child.wait_with_output().unwrap();
    assert_eq!(
        output.status.code().unwrap(),
        case["exit"].as_i64().unwrap() as i32
    );
    assert_eq!(support::reported(&output), case["code"]);
}
