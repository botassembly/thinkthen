//! Named current-native C helpers run under the existing sanitizer consumer.
use super::*;
use conformance_backend::{Canned, Listener};
#[test]
fn counted_inputs_and_typed_results_own_their_storage() {
    let backend = Listener::answering(|_| {
        Canned::ok(r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9}}}"#)
    })
    .expect("loopback");
    let base = backend.base().to_owned();
    let output = run(&compile(&crate_dir().join("tests/c/current.c")), &base, b"");
    assert_eq!(
        (output.status.code(), text(&output.stderr)),
        (Some(0), String::new())
    );
    assert_eq!(backend.count(), 1);
    let body = backend.requests();
    assert!(text(&body[0].body).contains("Refund me."));
    assert!(text(&body[0].body).contains("asks for a refund"));
}

#[test]
fn ten_named_current_calls_expose_native_values_and_owned_structures() {
    let backend = Backend::start().expect("loopback");
    let base = format!("{}/generic/v1", backend.origin());
    let output = run(
        &compile(&crate_dir().join("tests/c/current_calls.c")),
        &base,
        b"T",
    );
    assert_eq!(
        (output.status.code(), text(&output.stderr)),
        (Some(0), String::new())
    );
    assert_eq!(
        backend.count(),
        13,
        "actual packed sends for ten nonempty typed functions"
    );
}
#[test]
fn typed_file_calls_reuse_the_shared_native_documents() {
    let backend = Backend::start().expect("loopback");
    let base = format!("{}/generic/v1", backend.origin());
    let documents = crate_dir().join("../../specification/fixtures/files/documents");
    let output = run_with(
        &compile(&crate_dir().join("tests/c/current_calls.c")),
        &base,
        b"F",
        &[("TYPED_FILES", &documents)],
    );
    assert_eq!(
        (output.status.code(), text(&output.stderr)),
        (Some(0), String::new())
    );
    assert_eq!(backend.count(), 13);
}
#[test]
fn typed_argument_deadline_and_cancel_refusals_preserve_outputs_and_send_nothing() {
    let backend = Backend::start().expect("loopback");
    let base = format!("{}/generic/v1", backend.origin());
    let output = run(
        &compile(&crate_dir().join("tests/c/current_calls.c")),
        &base,
        b"R",
    );
    assert_eq!(
        (output.status.code(), text(&output.stderr)),
        (Some(0), String::new())
    );
    assert_eq!(backend.count(), 0);
}

#[test]
fn successful_null_and_authored_content_are_typed_and_empty_calls_have_no_provenance() {
    let consumer = compile(&crate_dir().join("tests/c/current_states.c"));
    for (mode, probability, sends) in [(b"N".as_slice(), "0.5", 1), (b"A".as_slice(), "0.9", 2)] {
        let reply = format!(
            r#"{{"model":"jev-latest","answers":{{"q1":{{"type":"noul","noul":{probability}}}}}}}"#
        );
        let backend = Listener::answering(move |_| Canned::ok(&reply)).expect("loopback");
        let output = run(&consumer, backend.base(), mode);
        assert_eq!(
            (output.status.code(), text(&output.stderr)),
            (Some(0), String::new())
        );
        assert_eq!(backend.count(), sends);
    }
}
#[test]
fn shared_immutable_handles_return_independent_owned_results_on_two_threads() {
    let backend = Backend::start().expect("loopback");
    let base = format!("{}/generic/v1", backend.origin());
    let output = run(
        &compile(&crate_dir().join("tests/c/current_states.c")),
        &base,
        b"C",
    );
    assert_eq!(
        (output.status.code(), text(&output.stderr)),
        (Some(0), String::new())
    );
    assert_eq!(backend.count(), 2);
}
#[test]
fn typed_started_failures_keep_native_facts_and_accessors_do_not_replace_errors() {
    let backend = Backend::start().expect("loopback");
    let base = format!("{}/arm/status/401/v1", backend.origin());
    let output = run(
        &compile(&crate_dir().join("tests/c/current_states.c")),
        &base,
        b"E",
    );
    assert_eq!(
        (output.status.code(), text(&output.stderr)),
        (Some(0), String::new())
    );
    assert_eq!(backend.count(), 1);
}
#[test]
fn typed_question_load_uses_the_shared_native_question_set() {
    let backend = Backend::start().expect("loopback");
    let base = format!("{}/generic/v1", backend.origin());
    let question = crate_dir().join("../../specification/fixtures/files/questions.json");
    let output = run_with(
        &compile(&crate_dir().join("tests/c/current_states.c")),
        &base,
        b"L",
        &[("TYPED_QUESTION", &question)],
    );
    assert_eq!(
        (output.status.code(), text(&output.stderr)),
        (Some(0), String::new())
    );
    assert_eq!(backend.count(), 1);
}

#[test]
fn typed_annotation_distinguishes_member_failure_from_success_and_preserves_authored_meaning() {
    let backend = Backend::start().expect("loopback");
    let base = format!("{}/arm/malformed/missing_probability/v1", backend.origin());
    let output = run(
        &compile(&crate_dir().join("tests/c/current_states.c")),
        &base,
        b"M",
    );
    assert_eq!(
        (output.status.code(), text(&output.stderr)),
        (Some(0), String::new())
    );
    assert_eq!(backend.count(), 1);
}
#[test]
fn typed_recording_replay_keeps_owned_values_and_reports_zero_sends_and_empty_attempts() {
    let backend = Backend::start().expect("loopback");
    let base = format!("{}/generic/v1", backend.origin());
    let folder = scratch("typed-recording").join("recording");
    let record = serde_json::json!({"base_url":base,"cache":false,"record":folder});
    let replay = serde_json::json!({"base_url":base,"cache":false,"replay":folder});
    let script = format!("P\n{record}\n{replay}\n");
    let output = run(
        &compile(&crate_dir().join("tests/c/current_states.c")),
        &base,
        script.as_bytes(),
    );
    assert_eq!(
        (output.status.code(), text(&output.stderr)),
        (Some(0), String::new())
    );
    assert_eq!(backend.count(), 1, "the replay sent nothing");
}

#[test]
fn typed_line_sources_preserve_unicode_crlf_and_physical_lines_without_sending_paths() {
    let backend=Listener::answering(|_| Canned::ok(r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.9}}}"#)).expect("loopback");
    let path = scratch("typed-lines").join("named-input.txt");
    std::fs::write(&path, "é\r\n\r\n😀 tail\n").expect("source");
    let output = run_with(
        &compile(&crate_dir().join("tests/c/current_files.c")),
        backend.base(),
        b"",
        &[("TYPED_FILE", &path)],
    );
    assert_eq!(
        (output.status.code(), text(&output.stderr)),
        (Some(0), String::new())
    );
    assert_eq!(backend.count(), 1);
    let requests = backend.requests();
    let body: serde_json::Value = serde_json::from_slice(&requests[0].body).expect("wire");
    assert_eq!(
        body["state"],
        "Each question quotes the text it asks about."
    );
    assert!(
        body["questions"]["q1"]["instructions"]
            .as_str()
            .expect("quoted evidence")
            .contains("é")
    );
    assert!(
        body["questions"]["q2"]["instructions"]
            .as_str()
            .expect("quoted evidence")
            .contains("😀 tail")
    );
    assert!(!text(&requests[0].body).contains("named-input.txt"));
}
