#![cfg(feature = "command")]
#![cfg(test)]
//! Rust callers receive a complete file and the original named-outcome failure.

use loopback::{KEY, Loopback};
use std::fs;
use thinkthen::files::{FileFormat, FileJob, FileOutput};
use thinkthen::{BackendName, Cancel, Code, Key, Request, Settings};

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
    let request = Request::from_json(r#"{"question":{"function":"annotate","questions":{"bad":{"function":"decide","question":"Ready?","evidence":"/Missing"},"good":{"function":"decide","question":"Ready?","evidence":"/Body"}}}}"#).unwrap();
    let server = Loopback::noul(0.47).unwrap();
    let client = Settings::default()
        .url(server.base())
        .api_key(Key::new(KEY).unwrap())
        .engine(|_| None)
        .unwrap();
    let receipt = job.execute(&request, &client, &Cancel::default()).unwrap();
    assert_eq!(receipt.first_failure.unwrap().code(), Code::RecordMissing);
    assert!(receipt.backup.is_none());
    let output = fs::read_to_string(&receipt.output).unwrap();
    assert!(output.starts_with("Body,bad,bad_status,bad_probability,bad_error,bad_detail,bad_source,good,good_status,good_probability,good_error,good_detail,good_source\n"));
    assert!(output.contains(r#"alpha,,failed,,"{""code"":""record.missing"""#));
    assert!(output.contains(",FALSE,answered,0.47,,"));
    assert_eq!(fs::read_to_string(input).unwrap(), "Body\nalpha\n");
    assert_eq!(server.count(), 1);
}

#[test]
fn cancellation_publishes_nothing_and_record_failures_publish_a_complete_receipt() {
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
    let request = Request::from_json(r#"{"question":{"function":"rank","question":"Ready?","levels":["low","high"],"evidence":"/Missing"}}"#).unwrap();
    let client = Settings::default()
        .with_backend(BackendName::Stub)
        .engine(|_| None)
        .unwrap();
    let stop = Cancel::default();
    stop.cancel();
    assert!(job.execute(&request, &client, &stop).is_err());
    assert!(!directory.path().join("requests.thinkthen.csv").exists());
    let receipt = job.execute(&request, &client, &Cancel::default()).unwrap();
    assert_eq!(receipt.first_failure.unwrap().code(), Code::RecordMissing);
    assert!(receipt.output.exists());
    let output = fs::read_to_string(receipt.output).unwrap();
    assert!(output.contains("failed"));
    assert!(output.contains("record.missing"));
}
