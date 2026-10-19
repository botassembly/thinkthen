//! Stays: unreachable native fault projections need direct private diagnostic conversion while public producers cover reachable cases.
//! Native invariant faults cannot be requested through the command surface.
//! The library proves their actual retained public projections; these tests
//! pin the command's conversion of those projections without an injection API.

use crate::failure::{Failure, report};
use std::process::ExitCode;
use thinkthen::ErrorDiagnostic as D;

#[test]
fn native_fault_projections_keep_command_causes_and_exit_codes() {
    let cases = [
        (D::Usage("injected invalid arguments"), 2),
        (D::Status(422), 4),
        (D::RecordingStorage, 5),
        (D::Cancelled, 130),
        (
            D::Deadline("the deadline of 0 s passed before the call answered".to_owned()),
            70,
        ),
        (D::Defect("injected invariant failure"), 70),
    ];
    for (place, (diagnostic, code)) in cases.into_iter().enumerate() {
        let cause = super::diagnostic_failure(diagnostic, false);
        match (place, &cause) {
            (0, Failure::Usage(message)) => assert_eq!(*message, "injected invalid arguments"),
            (1, Failure::Status(status, retryable)) => {
                assert_eq!(*status, 422);
                assert!(!retryable);
            }
            (2, Failure::RecordingStorage) | (3, Failure::Cancelled) => {}
            (4, Failure::Defect(message)) => {
                assert_eq!(*message, "an unavailable deadline reached the command");
            }
            (5, Failure::Defect(message)) => assert_eq!(*message, "injected invariant failure"),
            (_, cause) => panic!("fault {place} has the wrong command cause: {cause:?}"),
        }
        let failure = Failure::Stopped {
            at: 1,
            finished: 0,
            replayed: 0,
            recording: false,
            held: false,
            cause: Box::new(cause),
        };
        let mut diagnostic = Vec::new();
        assert_eq!(report(&failure, &mut diagnostic), ExitCode::from(code));
        assert!(
            !diagnostic.is_empty(),
            "fault {place} crossed the CLI mapping"
        );
    }
}

#[test]
fn actual_public_status_and_cancel_errors_keep_their_command_mapping() {
    let listener =
        conformance_backend::Listener::answering(|_| conformance_backend::Canned::status(422, ""))
            .expect("loopback listener");
    let engine = thinkthen::Engine::builder()
        .base_url(listener.base())
        .expect("loopback address")
        .api_key("test-key-not-real")
        .expect("fake key")
        .no_cache()
        .max_retries(0)
        .build()
        .expect("engine");
    let question = thinkthen::Question::decide("Does this ask for a refund?")
        .expect("question")
        .cut();
    let error = engine
        .decide(&question, "Refund me.")
        .expect_err("status refusal");
    match error.diagnostic() {
        Some(D::Range { first, last, cause }) => {
            assert_eq!(first, 0);
            assert_eq!(last, 0);
            assert!(matches!(*cause, D::Status(422)));
        }
        diagnostic => panic!("status refusal lost its native range: {diagnostic:?}"),
    }
    let failure = Failure::from(error);
    assert!(matches!(failure, Failure::Status(422, false)));
    let mut diagnostic = Vec::new();
    assert_eq!(report(&failure, &mut diagnostic), ExitCode::from(4));
    assert!(!diagnostic.is_empty());
    assert_eq!(listener.count(), 1);

    let token = thinkthen::CancelToken::new();
    token.cancel();
    let error = engine
        .decide_with(
            &question,
            "Refund me.",
            thinkthen::CallOptions::new().cancel(&token),
        )
        .expect_err("cancelled call");
    assert_eq!(error.kind(), thinkthen::ErrorKind::Cancelled);
    assert!(
        error.diagnostic().is_none(),
        "pre-admission cancellation has no native cause"
    );
    let cause = Failure::from(error);
    assert!(matches!(cause, Failure::Cancelled));
    let failure = Failure::Stopped {
        at: 1,
        finished: 0,
        replayed: 0,
        recording: false,
        held: false,
        cause: Box::new(cause),
    };
    diagnostic.clear();
    assert_eq!(report(&failure, &mut diagnostic), ExitCode::from(130));
    assert!(!diagnostic.is_empty());
    assert_eq!(listener.count(), 1, "cancelled call sends nothing");
}
