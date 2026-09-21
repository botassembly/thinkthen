use super::{Failure, report};
use crate::core::recording::{Entry, Exchange as Recorded};
use crate::core::{QuestionSetError, RecordError, Url};
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
        }],
        top: Some(2),
        writer: &mut written,
    };

    let shown = format!(
        "{key:?} {exchange:?} {judged:?} {client:?} {recorded:?} {entry:?} \
             {ordered:?} {:?} {:?} {:?}",
        Failure::NoKey("THINKTHEN_API_KEY".to_owned()),
        Failure::Status(401),
        Failure::QuestionSet(QuestionSetError::Duplicate(format!("{KEY}.{EVIDENCE}"))),
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
        Failure::Transport("connection refused".to_owned()),
        Failure::Record(RecordError::NotUtf8),
        Failure::Record(RecordError::TooLarge),
        Failure::ReplayMiss("abc.json".to_owned()),
        Failure::Entry("abc.json".to_owned(), "it records another".to_owned()),
        Failure::Stopped {
            at: 2,
            finished: 1,
            replayed: 0,
            held: false,
            cause: Box::new(Failure::Record(RecordError::TooLarge)),
        },
        Failure::Stopped {
            at: 2,
            finished: 1,
            replayed: 0,
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
fn stopped_counts_use_record_only_at_one() {
    let cases = [
        (
            1,
            1,
            "thinkthen: stopped at record 2; 1 record finished, 1 record from a recording\n",
        ),
        (
            2,
            0,
            "thinkthen: stopped at record 3; 2 records finished, 0 records from a recording\n",
        ),
    ];
    for (finished, replayed, summary) in cases {
        let failure = Failure::Stopped {
            at: finished + 1,
            finished,
            replayed,
            held: false,
            cause: Box::new(Failure::Record(RecordError::TooLarge)),
        };
        let mut written = Vec::new();
        report(&failure, &mut written);
        let said = String::from_utf8(written).expect("a diagnostic is text");
        assert!(said.ends_with(summary), "{said}");
    }
}
