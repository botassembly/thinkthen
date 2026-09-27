use super::{Failure, report};
use crate::core::recording::{Entry, Exchange as Recorded};
use crate::core::{
    LimitKind, ProfileError, ProfileLimit, ProfileName, QuestionSetError, RecordError, Url,
};
use crate::engine::error::{Error as EngineError, TransportKind};
use crate::engine::{http::Client, process_width};
use std::{process::ExitCode, time::Duration};

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
    let key = crate::engine::http::Key::of(KEY);
    let body = format!(r#"{{"state":"{EVIDENCE}"}}"#);
    let url = Url::new("http://127.0.0.1:1/v1/systemone").expect("an address");
    let recorded = Recorded::new(&url, body.as_bytes());
    let entry = Entry::of(&recorded, body.as_bytes()).expect("both bodies are JSON");
    let exchange = crate::engine::http::Exchange {
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
    let client = Client::new(Duration::from_secs(1), false, process_width());
    // `rank` holds every record in memory until the input ends, so the
    // sink that holds them is the one new place a whole record could leak.
    let mut written = Vec::new();
    let usage = crate::engine::usage::Counters::new(None);
    let mut ordered = crate::schedule::Output::ordered(&mut written, Some(2), &usage);
    ordered
        .take(crate::schedule::Judged {
            printed: Some(body.clone()),
            outcome: crate::core::Outcome::Yes,
            probability: Some(0.91),
            replayed: false,
            partial_failure: false,
            profile_mismatch: None,
        })
        .expect("row held");

    // `relate` reads its evidence as entity names and kinds.
    let entity = crate::core::RelationEntity::new(EVIDENCE, EVIDENCE).expect("an entity");
    let shown = format!(
        "{key:?} {exchange:?} {judged:?} {client:?} {recorded:?} {entry:?} \
             {ordered:?} {entity:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
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
    );

    assert!(!shown.contains(KEY), "{shown}");
    assert!(!shown.contains(EVIDENCE), "{shown}");
    assert!(shown.contains("withheld"), "{shown}");
}

/// `choose`, `tag`, and `score` can read their labels from a record. The
/// `--plan` document carries the request, the engine hands back answers with
/// those labels, and a result row holds both. No `Debug` line shows them.
#[test]
fn no_record_label_request_or_answer_debug_line_shows_the_evidence() {
    use crate::core::Question;
    let record = crate::core::Reading::new(crate::core::Framing::Jsonl, Vec::new())
        .expect("a JSON reading")
        .record(format!(r#"{{"labels":["{EVIDENCE}","other"]}}"#).as_bytes())
        .expect("a JSON record");
    let labels = record
        .choices(&crate::core::Pointer::new("/labels").expect("a pointer"))
        .expect("two labels");
    let text = crate::core::QuestionText::new("Which one fits?").expect("a question");
    let questions = [
        Question::Choose {
            text: text.clone(),
            options: labels.clone(),
        },
        Question::Tag {
            text: text.clone(),
            labels: labels.clone(),
        },
        Question::Score {
            text,
            levels: labels,
        },
    ];
    let answers = [
        format!(r#""q1":{{"type":"choice","probabilities":{{"{EVIDENCE}":0.75,"other":0.25}}}}"#),
        r#""q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.1}"#.to_owned(),
        r#""q1":{"type":"score","probabilities":{"0":0.75,"1":0.25}}"#.to_owned(),
    ];
    let (listener, backend, engine) = loopback_engine(&answers);
    let evidence = crate::core::Evidence::new(EVIDENCE).expect("evidence");
    let plan = crate::core::Plan::new(
        evidence.clone(),
        backend.model().clone(),
        questions.to_vec(),
    )
    .expect("a plan");
    let document = crate::core::PlanDocument::of(&backend, &plan).expect("a plan document");
    let mut shown = format!("{plan:?} {plan:#?} {document:?} {document:#?}");
    let mut judged = Vec::new();
    for question in questions {
        let judgment = engine
            .judge(
                &question,
                None,
                evidence.clone(),
                &crate::engine::Cancel::default(),
            )
            .expect("a judgment");
        let reply = &judgment.answered.reply;
        let meta = crate::core::Meta::new(
            "0.0.0",
            String::new(),
            backend.url().clone(),
            reply.model().clone(),
            reply.usage(),
            crate::core::RequestMeta::new(false, 1, Vec::new()),
        );
        let row = crate::core::DecisionResult::new(
            judgment.value.clone(),
            question,
            judgment.answer.clone(),
            None,
            meta,
        );
        shown.push_str(&format!("{reply:?} {reply:#?} {row:?} {row:#?}"));
        judged.push(format!("{:?} {:?}", judgment.answer, judgment.value));
    }
    assert_eq!(listener.requests().len(), 3);
    assert!(!shown.contains(EVIDENCE), "{shown}");
    assert_eq!(
        judged,
        [
            "Answer(Choice { pick: <22 bytes withheld>, probabilities: \
             Distribution { labels: <27 bytes withheld>, probabilities: [0.75, 0.25] }, \
             confidence: None }) Choice(Some(<22 bytes withheld>))",
            "Answer(Tag { probabilities: TagProbabilities { labels: <27 bytes withheld>, \
             probabilities: [0.9, 0.1] } }) Tag([<22 bytes withheld>])",
            "Answer(Score { level: <22 bytes withheld>, probabilities: \
             Distribution { labels: <27 bytes withheld>, probabilities: [0.75, 0.25] }, \
             confidence: None }) Score(0.25)",
        ]
    );
}

/// An engine over a loopback listener that answers each request with the next `answers`.
fn loopback_engine(
    answers: &[String],
) -> (
    conformance_backend::Listener,
    crate::core::Backend,
    crate::engine::facade::Engine,
) {
    let listener = conformance_backend::Listener::serving(
        answers
            .iter()
            .map(|answers| {
                conformance_backend::Canned::ok(&format!(
                    r#"{{"model":"jev-latest","answers":{{{answers}}}}}"#
                ))
            })
            .collect(),
    )
    .expect("a loopback listener");
    let backend =
        crate::core::Backend::resolve(Some(listener.base()), None, "jev-latest").expect("backend");
    let engine = crate::engine::facade::Engine::new(crate::engine::facade::Settings {
        backend: backend.clone(),
        profile: None,
        timeout: Duration::from_secs(5),
        max_retries: 0,
        retry_wait: Duration::from_millis(10),
        width: None,
        storage: crate::engine::facade::Storage::default(),
        key: std::sync::Arc::new(|| Ok(crate::engine::facade::Key::new(KEY.to_owned()))),
        usage: std::sync::Arc::default(),
    })
    .expect("an engine");
    (listener, backend, engine)
}

/// `recognize` reads its evidence as names and edge labels, in plain and pretty `Debug`.
#[test]
fn no_recognize_debug_line_shows_the_evidence() {
    let name = crate::core::RecognizedName {
        text: EVIDENCE.to_owned(),
        start: 0,
        end: 1,
        length: 1,
        kind: "person".to_owned(),
        strength: 0.9,
    };
    let edge = crate::core::RelationEdge {
        relation: "works_for".to_owned(),
        source: name.clone(),
        target: name.clone(),
        probability: 0.8,
    };
    let edges = crate::core::Odds(vec![(EVIDENCE.to_owned(), 0.9)]);
    let lines = crate::core::Reading::new(crate::core::Framing::Lines, Vec::new())
        .expect("a text reading")
        .record(format!("\u{e9}{EVIDENCE}").as_bytes())
        .expect("a text record");
    let object =
        crate::core::Record::string_fields(vec![(EVIDENCE.to_owned(), EVIDENCE.to_owned())]);
    let shown = format!(
        "{name:?} {name:#?} {edge:?} {edge:#?} {edges:?} {edges:#?} \
         {lines:?} {lines:#?} {object:?} {object:#?}"
    );
    assert!(!shown.contains(EVIDENCE), "{shown}");
    assert_eq!(shown.matches("withheld").count(), 12, "{shown}");
    assert_eq!(format!("{edges:?}"), "Odds([(<22 bytes withheld>, 0.9)])");
    assert_eq!(format!("{lines:?}"), "Record(text, <24 bytes withheld>)");
    assert_eq!(format!("{object:?}"), "Record(json, <51 bytes withheld>)");
    assert!(
        shown.contains(r#"start: 0, end: 1, length: 1, kind: "person", strength: 0.9"#),
        "{shown}"
    );
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
            TransportKind::Tls,
            "thinkthen: the TLS connection or certificate check failed; check --url and the backend's certificate trust\n",
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
            "thinkthen: stopped by a signal; 1 record finished, 1 record from a recording\n",
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
