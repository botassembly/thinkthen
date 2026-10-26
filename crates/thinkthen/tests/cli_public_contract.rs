#![cfg(feature = "command")]
#![cfg(test)]
//! Command translation of shared usage failures.
mod support;

#[test]
fn malformed_annotation_and_missing_key_exit_as_usage() {
    let directory = tempfile::tempdir().unwrap();
    let malformed = directory.path().join("annotation.json");
    std::fs::write(&malformed, "{").unwrap();
    for (arguments, code) in [
        (
            vec!["annotate", malformed.to_str().unwrap(), "--backend", "stub"],
            "usage.question",
        ),
        (vec!["decide", "Ready?"], "usage.key-missing"),
    ] {
        let output = support::command().args(arguments).output().unwrap();
        assert_eq!(output.status.code(), Some(2));
        assert_eq!(support::reported(&output), code);
    }
}

#[test]
fn invalid_native_input_is_a_failed_record() {
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
        .args(["decide", "Ready?", "--backend", "stub"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(&input).unwrap();
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(6));
    let row: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(row["error"]["code"], "record.invalid");
}

#[test]
fn dynamic_question_and_toolbox_admission_preserve_library_messages() {
    let cases: serde_json::Value = serde_json::from_str(include_str!(
        "../../../conformance/public-contract/usage-admission.json"
    ))
    .unwrap();
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("malformed.json");
    for case in cases.as_array().unwrap() {
        assert_eq!(case["kind"], "admission");
        let (input, error) = match case["boundary"].as_str().unwrap() {
            "question" => {
                // The command admits a saved request, which adds its own path.
                let input = serde_json::json!({"question":case["input"]}).to_string();
                let error = thinkthen::Request::from_json(&input).unwrap_err();
                assert_eq!(
                    error.message(),
                    format!("request.{}", case["message"].as_str().unwrap())
                );
                (input, error)
            }
            "toolbox" => {
                let input = case["input"].to_string();
                let error = thinkthen::Toolbox::from_json(&input).unwrap_err();
                assert_eq!(error.message(), case["message"].as_str().unwrap());
                (input, error)
            }
            // The command has no arbitrary settings JSON or retry option.
            "settings" => continue,
            _ => unreachable!(),
        };
        assert_eq!(error.code().as_str(), case["code"].as_str().unwrap());
        std::fs::write(&path, input).unwrap();
        let output = support::command()
            .args(["run", path.to_str().unwrap(), "--backend", "stub"])
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2), "{}", case["name"]);
        assert!(output.stdout.is_empty());
        assert_eq!(
            String::from_utf8(output.stderr).unwrap(),
            format!(
                "thinkthen: {}: {}\n",
                error.code().as_str(),
                error.message()
            ),
            "{}",
            case["name"]
        );
    }
}
