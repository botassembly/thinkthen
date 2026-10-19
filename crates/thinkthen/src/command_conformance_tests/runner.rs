//! Native scheduler fault injection stays because callers cannot request an invariant
//! failure, a native cancellation token, or an expired native deadline.

use super::{CASES, Document};
use crate::engine::Cancel;
use crate::engine::error::Error as EngineError;
use crate::public::complete::recognize::ordered::{self, Input, Outcome};
use std::time::Duration;

#[test]
fn shared_fault_cases_cross_the_native_scheduler() {
    let document: Document = serde_json::from_str(CASES).expect("shared document");
    for case in &document.cases {
        if let Some(expected) = &case.expect.error {
            run_fault(case, &expected.kind);
        }
    }
}

fn run_fault(case: &super::Case, expected: &str) {
    if case.question_form.is_some() {
        return;
    }
    let injection = &case.operation.as_ref().expect("fault injection").injection;
    let outcome = ordered::run(
        1,
        &Cancel::default(),
        None,
        || Some(Input::Item(())),
        &|()| Err::<ordered::Row<()>, _>(injected(injection)),
        |()| Ok(true),
        &|error| error,
        || EngineError::Defect("the reader ended"),
    )
    .expect("the runner itself remains sound");
    let Outcome::Stopped { cause, .. } = outcome else {
        panic!("{} did not stop", case.id);
    };
    assert_eq!(cause.kind().as_str(), expected, "{}", case.id);
    let public = crate::Error::from(cause);
    assert_eq!(public.kind().name(), expected, "{}", case.id);
    use crate::ErrorDiagnostic as D;
    match (injection.as_str(), public.diagnostic()) {
        ("invalid_arguments", Some(D::Usage(message))) => {
            assert_eq!(message, "injected invalid arguments");
        }
        ("response_refusal", Some(D::Status(status))) => assert_eq!(status, 422),
        ("recording_read_failure", Some(D::RecordingStorage))
        | ("cancel_token", Some(D::Cancelled)) => {}
        ("expired_deadline", Some(D::Deadline(message))) => {
            assert_eq!(message, "the deadline of 0 s passed before the call answered");
        }
        ("internal_invariant_failure", Some(D::Defect(message))) => {
            assert_eq!(message, "injected invariant failure");
        }
        (injection, diagnostic) => panic!("{injection} lost its diagnostic: {diagnostic:?}"),
    }
}

fn injected(name: &str) -> EngineError {
    match name {
        "invalid_arguments" => EngineError::Usage("injected invalid arguments"),
        "response_refusal" => EngineError::Status(422),
        "recording_read_failure" => EngineError::RecordingStorage,
        "cancel_token" => EngineError::Cancelled,
        "expired_deadline" => EngineError::Deadline(crate::engine::error::Budget(Duration::ZERO)),
        "internal_invariant_failure" => EngineError::Defect("injected invariant failure"),
        other => panic!("unknown injection {other}"),
    }
}
