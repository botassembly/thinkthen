#![cfg(feature = "command")]
#![cfg(test)]
//! Rust callers receive a complete file and the original named-outcome failure.

use std::fs;
use thinkthen::files::{FileFormat, FileJob, FileOutput};
use thinkthen::{Cancel, Client, ErrorKind, SdkRequest};

#[test]
fn completed_annotations_keep_good_and_failed_cells_with_typed_failure() {
    let directory = tempfile::TempDir::new().unwrap();
    let input = directory.path().join("requests.csv");
    fs::write(&input, "Body\nalpha\n").unwrap();
    let job = FileJob::new(
        input.clone(),
        FileFormat::Csv,
        None,
        vec!["Body".into()],
        None,
        FileOutput::NewFile(None),
    )
    .unwrap();
    let request = SdkRequest::from_json(r#"{"function":"annotate","question":{"type":"inline","value":{"questions":{"bad":{"decide":"Ready?","on":"/Missing"},"good":{"decide":"Ready?","on":"/Body"}}}}}"#).unwrap();
    let client = Client::from_json(r#"{"adapter":"stub"}"#, |_| None).unwrap();
    let receipt = job.execute(&request, &client, &Cancel::default()).unwrap();
    assert_eq!(receipt.first_failure.unwrap().kind(), ErrorKind::Missing);
    assert!(receipt.backup.is_none());
    let output = fs::read_to_string(&receipt.output).unwrap();
    assert!(output.starts_with("Body,bad,bad_status,bad_p,bad_error,bad_detail,good,good_status,good_p,good_error,good_detail\n"));
    assert!(output.contains(r#"alpha,,failed,,"{""code"":""usage.missing"""#));
    assert!(output.contains(",FALSE,answered,0.47,,"));
    assert_eq!(fs::read_to_string(input).unwrap(), "Body\nalpha\n");
}

#[test]
fn cancelled_and_final_failed_operations_publish_nothing() {
    let directory = tempfile::TempDir::new().unwrap();
    let input = directory.path().join("requests.csv");
    fs::write(&input, "Body,Other\nalpha,beta\n").unwrap();
    let job = FileJob::new(
        input,
        FileFormat::Csv,
        None,
        vec!["Body".into(), "Other".into()],
        Some("rank".into()),
        FileOutput::NewFile(None),
    )
    .unwrap();
    let request = SdkRequest::from_json(r#"{"function":"rank","question":{"type":"inline","value":"Ready?"},"options":{"on":"/Missing"}}"#).unwrap();
    let client = Client::from_json(r#"{"adapter":"stub"}"#, |_| None).unwrap();
    let stop = Cancel::default();
    stop.cancel();
    assert!(job.execute(&request, &client, &stop).is_err());
    assert!(!directory.path().join("requests.thinkthen.csv").exists());
    let failure = job
        .execute(&request, &client, &Cancel::default())
        .unwrap_err();
    assert_eq!(failure.code(), "usage.missing");
    assert!(!directory.path().join("requests.thinkthen.csv").exists());
}
