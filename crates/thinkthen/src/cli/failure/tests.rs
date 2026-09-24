use super::{Failure, report};
use crate::core::recording::{Entry, Exchange as Recorded};
use crate::core::{
    LimitKind, ProfileError, ProfileLimit, ProfileName, QuestionSetError, RecordError, Url,
};
use crate::engine::error::{Error as EngineError, TransportKind};
use std::process::ExitCode;
use std::time::Duration;

/// The key and the evidence every case here is built from.
///
/// They are the same two markers `crates/thinkthen/tests/backend/secrecy.rs`
/// sweeps the compiled binary with. That sweep reads what a run writes, and
/// this one reads the `Debug` lines a message could be built from.
const KEY: &str = "sk-marker-2f9d41c6";
const EVIDENCE: &str = "marker-evidence-7b3ac5";

/// Every `Debug` line that could hold the key or the evidence holds neither.
///
/// A `{:?}` is how a key reaches a log by accident, so every type that
/// carries one is rendered here and read. A new type that holds either goes
/// in this list.
#[test]
fn no_debug_line_shows_the_key_or_the_evidence() {
    let key = crate::http::Key::of(KEY);
    let body = format!(r#"{{"state":"{EVIDENCE}"}}"#);
    let url = Url::new("http://127.0.0.1:1/v1/systemone").expect("an address");
    let recorded = Recorded::new(&url, body.as_bytes());
    let entry = Entry::of(&recorded, body.as_bytes()).expect("both bodies are JSON");
    let exchange = crate::http::Exchange {
        url: url.as_str(),
        body: body.as_bytes(),
        key: &key,
        max_retries: 2,
        retry_wait: Duration::from_secs(1),
    };
    let judged = crate::schedule::Judged {
        printed: Some(body.clone()),
        outcome: crate::core::Outcome::Yes,
        probability: Some(0.91),
        replayed: false,
        partial_failure: false,
        profile_mismatch: None,
    };
    let client = crate::http::Client::new(Duration::from_secs(1), false);
    // `rank` holds every record in memory until the input ends, so the
    // sink that holds them is the one new place a whole record could leak.
    let mut written = Vec::new();
    let ordered = crate::schedule::Output::Ordered {
        held: vec![crate::schedule::Judged {
            printed: Some(body.clone()),
            outcome: crate::core::Outcome::Yes,
            probability: Some(0.91),
            replayed: false,
            partial_failure: false,
            profile_mismatch: None,
        }],
        top: Some(2),
        writer: &mut written,
    };

    // `relate` reads its evidence as entity names and kinds.
    let entity = crate::core::RelationEntity::new(EVIDENCE, EVIDENCE).expect("an entity");
    let shown = format!(
        "{key:?} {exchange:?} {judged:?} {client:?} {recorded:?} {entry:?} \
             {ordered:?} {entity:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
        Failure::NoKey("THINKTHEN_API_KEY".to_owned()),
        Failure::Status(401),
        Failure::QuestionSet(QuestionSetError::Duplicate(format!("{KEY}.{EVIDENCE}"))),
        Failure::OpenProfile {
            path: "safe-profile.json".into(),
            error: std::io::Error::from(std::io::ErrorKind::NotFound),
        },
        Failure::Profile {
            path: "safe-profile.json".into(),
            error: ProfileError::Name,
        },
        Failure::ProfileLimit(ProfileLimit {
            name: ProfileName::new("safe-profile").expect("safe name"),
            kind: LimitKind::EvidenceBytes,
            limit: 1,
            actual: EVIDENCE.len(),
        }),
        Failure::Recognize(super::recognize::Error::LogicalQuestion),
        Failure::RecordingStorage,
        Failure::Relate(super::relate::Error::Config {
            file: true,
            error: crate::core::RelateConfigError::Relation,
        }),
        Failure::Relate(super::relate::Error::Logical),
    );

    assert!(!shown.contains(KEY), "{shown}");
    assert!(!shown.contains(EVIDENCE), "{shown}");
    assert!(shown.contains("withheld"), "{shown}");
}

/// No message a user reads holds the key or the evidence.
///
/// Every variant is reported, so a variant added later that quotes either
/// one fails here.
#[test]
fn no_diagnostic_holds_the_key_or_the_evidence() {
    let body = format!(r#"{{"state":"{EVIDENCE}"}}"#);
    let cases = [
        Failure::NoKey("THINKTHEN_API_KEY".to_owned()),
        Failure::Status(401),
        Failure::Transport(TransportKind::Refused),
        Failure::Record(RecordError::NotUtf8),
        Failure::Record(RecordError::TooLarge),
        Failure::ReplayMiss("abc.json".to_owned()),
        Failure::Entry("abc.json".to_owned(), "it records another".to_owned()),
        Failure::Stopped {
            at: 2,
            finished: 1,
            replayed: 0,
            recording: false,
            held: false,
            cause: Box::new(Failure::Record(RecordError::TooLarge)),
        },
        Failure::Stopped {
            at: 2,
            finished: 1,
            replayed: 0,
            recording: false,
            held: true,
            cause: Box::new(Failure::Record(RecordError::TooLarge)),
        },
        Failure::QuietOverKept("filter"),
        Failure::RawOverKept("rank"),
        Failure::NoFraming("filter"),
        Failure::TopIsZero,
        Failure::FindCount { none: false },
        Failure::FindCount { none: true },
        Failure::FindTooLarge,
        Failure::OpenProfile {
            path: "safe-profile.json".into(),
            error: std::io::Error::from(std::io::ErrorKind::NotFound),
        },
        Failure::Profile {
            path: "safe-profile.json".into(),
            error: ProfileError::Name,
        },
        Failure::ProfileLimit(ProfileLimit {
            name: ProfileName::new("safe-profile").expect("safe name"),
            kind: LimitKind::EvidenceBytes,
            limit: 1,
            actual: EVIDENCE.len(),
        }),
        Failure::Recognize(super::recognize::Error::LogicalQuestion),
        Failure::RecordingStorage,
        Failure::Defect("a ranked row carries no probability"),
    ];

    for failure in cases {
        let mut written = Vec::new();
        report(&failure, &mut written);
        let said = String::from_utf8(written).expect("a diagnostic is text");

        assert!(!said.contains(KEY), "{said}");
        assert!(!said.contains(EVIDENCE), "{said}");
        assert!(!said.contains(&body), "{said}");
    }
}

#[test]
fn recording_storage_has_one_fixed_secret_safe_diagnostic() {
    let mut written = Vec::new();
    assert_eq!(
        report(&Failure::RecordingStorage, &mut written),
        ExitCode::from(5)
    );
    assert_eq!(
        String::from_utf8(written).expect("a diagnostic is text"),
        "thinkthen: the recording folder could not be read or written; check its permissions and free space\n"
    );
    assert_eq!(
        format!("{:?}", Failure::RecordingStorage),
        "RecordingStorage"
    );
}

#[test]
fn a_common_failure_status_carries_the_phrase_the_specification_fixes() {
    let cases = [
        (401, "the key was refused"),
        (402, "the account has no credit"),
        (403, "the key may not use this model or address"),
        (404, "nothing answers at this address"),
        (
            422,
            "the backend refused the request as malformed or too large",
        ),
        (429, "the backend's rate limit was reached"),
    ];

    for (status, phrase) in cases {
        let mut written = Vec::new();
        report(&Failure::Status(status), &mut written);
        let message = String::from_utf8(written).expect("a diagnostic is text");
        assert_eq!(
            message,
            format!("thinkthen: the backend answered with status {status}: {phrase}\n")
        );
    }

    let mut written = Vec::new();
    report(&Failure::Status(418), &mut written);
    let message = String::from_utf8(written).expect("a diagnostic is text");
    assert_eq!(message, "thinkthen: the backend answered with status 418\n");
}

#[test]
fn common_request_statuses_give_fixed_actions() {
    let cases = [
        (
            400,
            "thinkthen: the backend answered with status 400: the backend refused the request; check --model and the request size\n",
        ),
        (
            500,
            "thinkthen: the backend answered with status 500: the backend failed after the allowed attempts; try again later or change --max-retries\n",
        ),
    ];
    for (status, expected) in cases {
        let mut written = Vec::new();
        assert_eq!(
            report(&Failure::Status(status), &mut written),
            ExitCode::from(4)
        );
        assert_eq!(
            String::from_utf8(written).expect("diagnostic is text"),
            expected
        );
    }
}

#[test]
fn transport_kinds_give_fixed_actions() {
    let cases = [
        (
            TransportKind::Timeout,
            "thinkthen: the backend timed out; increase --timeout or try again\n",
        ),
        (
            TransportKind::NameLookup,
            "thinkthen: the backend's host could not be found; check --url and the network\n",
        ),
        (
            TransportKind::Refused,
            "thinkthen: the backend refused the connection; check that it is running and that --url is correct\n",
        ),
        (
            TransportKind::PrematureClose,
            "thinkthen: the backend closed the connection before a reply and may have received the request; it was not sent again\n",
        ),
        (
            TransportKind::Other,
            "thinkthen: the backend could not be reached; check --url and the network\n",
        ),
    ];
    for (kind, expected) in cases {
        let mut written = Vec::new();
        assert_eq!(
            report(&Failure::Transport(kind), &mut written),
            ExitCode::from(4)
        );
        assert_eq!(
            String::from_utf8(written).expect("diagnostic is text"),
            expected
        );
    }
}

#[test]
fn every_failure_reaches_its_own_exit_code_and_says_what_stopped() {
    let cases = [
        (Failure::Record(RecordError::NotUtf8), 5, "not valid UTF-8"),
        (Failure::Status(503), 4, "status 503"),
        (Failure::NoKey("THINKTHEN_API_KEY".to_owned()), 4, "unset"),
        (Failure::Defect("a plan asks nothing"), 70, "defect"),
        (Failure::FindCount { none: false }, 2, "2 to 255"),
        (Failure::FindCount { none: true }, 2, "2 to 254"),
        (Failure::FindTooLarge, 2, "16 MiB"),
    ];

    for (failure, code, said) in cases {
        let mut written = Vec::new();
        let exit = report(&failure, &mut written);
        let message = String::from_utf8(written).expect("a diagnostic is text");

        assert_eq!(format!("{exit:?}"), format!("{:?}", ExitCode::from(code)));
        assert!(message.starts_with("thinkthen: "), "{message}");
        assert!(message.contains(said), "{message}");
    }
}

#[test]
fn cancellation_is_typed_and_silent_except_for_a_stopped_summary() {
    assert!(matches!(
        Failure::from(EngineError::Cancelled),
        Failure::Cancelled
    ));

    let mut bare = Vec::new();
    assert_eq!(report(&Failure::Cancelled, &mut bare), ExitCode::from(130));
    assert!(bare.is_empty());
}

#[test]
fn stopped_counts_use_record_only_at_one() {
    let cases = [
        (
            1,
            1,
            Failure::Record(RecordError::TooLarge),
            "thinkthen: stopped at record 2; 1 record finished, 1 record from a recording\n",
        ),
        (
            2,
            0,
            Failure::Record(RecordError::TooLarge),
            "thinkthen: stopped at record 3; 2 records finished, 0 records from a recording\n",
        ),
        (
            1,
            1,
            Failure::Cancelled,
            "thinkthen: stopped at record 2; 1 record finished, 1 record from a recording\n",
        ),
    ];
    for (finished, replayed, cause, summary) in cases {
        let cancelled = matches!(cause, Failure::Cancelled);
        let failure = Failure::Stopped {
            at: finished + 1,
            finished,
            replayed,
            recording: true,
            held: false,
            cause: Box::new(cause),
        };
        let mut written = Vec::new();
        let code = report(&failure, &mut written);
        let said = String::from_utf8(written).expect("a diagnostic is text");
        assert_eq!(code, ExitCode::from(if cancelled { 130 } else { 2 }));
        assert_eq!(said == summary, cancelled);
        assert!(said.ends_with(summary), "{said}");
    }
}
