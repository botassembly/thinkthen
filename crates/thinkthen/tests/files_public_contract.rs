#![cfg(feature = "command")]
#![cfg(test)]
//! File admission and access preserve their public codes.

use thinkthen::files::{FileFormat, FileJob, FileOutput};
use thinkthen::{Cancel, Client, SdkRequest};

#[test]
fn file_boundaries_keep_argument_and_access_failures_distinct() {
    let cases: serde_json::Value = serde_json::from_str(include_str!(
        "../../../conformance/public-contract/boundaries.json"
    ))
    .unwrap();
    let directory = tempfile::tempdir().unwrap();
    let client = Client::from_json(r#"{"adapter":"stub"}"#, |_| None).unwrap();
    let request = SdkRequest::from_json(
        r#"{"function":"decide","question":{"type":"inline","value":"Ready?"}}"#,
    )
    .unwrap();
    for case in cases["files"].as_array().unwrap() {
        let result = FileJob::new(
            directory.path().join("absent.csv"),
            FileFormat::Csv,
            None,
            serde_json::from_value(case["columns"].clone()).unwrap(),
            Some("result".to_owned()),
            FileOutput::NewFile(None),
        )
        .and_then(|job| job.execute(&request, &client, &Cancel::default()));
        assert_eq!(result.unwrap_err().code(), case["code"]);
    }
}
