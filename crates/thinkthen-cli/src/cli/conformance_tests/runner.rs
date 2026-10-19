//! Stays: saved-response parser and fault tables exercise private command preparation without provider calls.
//! Direct fault injection stays because CLI callers cannot request an invariant failure,
//! a native cancellation token, or an expired native deadline.

use super::{CASES, Document, command};
use crate::cli::schedule::ordered::{self, Input, Outcome};
use crate::engine::Cancel;
use crate::engine::error::Error as EngineError;
use crate::failure::{Failure, report};
use std::thread;
use std::time::Duration;

#[test]
fn shared_fault_cases_cross_the_command_mapping() {
    let document: Document = serde_json::from_str(CASES).expect("shared document");
    for case in &document.cases {
        if let Some(expected) = &case.expect.error {
            run_fault(case, &expected.kind);
        }
    }
}

fn run_fault(case: &super::Case, expected: &str) {
    if case.question_form.is_some() {
        command::form(case);
        return;
    }
    let injection = &case.operation.as_ref().expect("fault injection").injection;
    let outcome = ordered::run(
        1,
        &Cancel::default(),
        |requests, events| {
            thread::spawn(move || {
                requests.recv().expect("one injected input");
                events
                    .send(Input::Item(()))
                    .expect("the runner receives input");
            });
        },
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
    let failure = Failure::Stopped {
        at: 1,
        finished: 0,
        replayed: 0,
        recording: false,
        held: false,
        cause: Box::new(Failure::from(cause)),
    };
    let mut diagnostic = Vec::new();
    let _code = report(&failure, &mut diagnostic);
    assert!(
        !diagnostic.is_empty(),
        "{} crossed the CLI mapping",
        case.id
    );
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
