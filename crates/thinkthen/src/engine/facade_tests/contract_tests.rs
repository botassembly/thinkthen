//! The facade's contract checks against the shared cases, its one state
//! accessor, and key secrecy.

use conformance_backend::{Backend as Loopback, Canned, Listener};

use super::{Scratch, TEST_KEY, ask, decide, engine, evidence, reader, settings};
use crate::core::QuestionSet;
use crate::engine::Cancel;
use crate::engine::error::Error;
use crate::engine::facade::{Completed, Engine, RunOutcome, Settings, Storage};

const CASES: &str = include_str!("../../../../../conformance/cases.json");

#[test]
fn bulk_and_one_question_annotate_answers_match_the_shared_cases() {
    let document: serde_json::Value = serde_json::from_str(CASES).expect("cases");
    let case = |id: &str| {
        document["cases"]
            .as_array()
            .expect("cases")
            .iter()
            .find(|case| case["id"] == id)
            .expect("case")
            .clone()
    };
    let loopback = Loopback::start().expect("loopback");
    let cancel = Cancel::default();

    let many = case("27-decide-many");
    let bulk_engine = engine(&format!("{}/case/27-decide-many/v1", loopback.origin()));
    let texts = many["exchanges"]
        .as_array()
        .expect("exchanges")
        .iter()
        .map(|exchange| {
            &*exchange["evidence"]
                .as_str()
                .expect("text")
                .to_owned()
                .leak()
        })
        .collect::<Vec<&'static str>>();
    let question = decide(many["question"]["decide"].as_str().expect("question"));
    let mut rows = Vec::new();
    let outcome = bulk_engine.records(
        crate::engine::schedule::RecordFlow::Streaming,
        &cancel,
        reader(texts),
        &|text: &&str| {
            bulk_engine
                .judge(&question, None, evidence(text), &cancel)
                .map(|judged| Completed::one(judged.answer.yes(), false, false))
        },
        |row| {
            rows.push(row);
            Ok(true)
        },
    );
    assert!(matches!(outcome, Ok(RunOutcome::Complete)), "{outcome:?}");
    let details = many["expect"]["success"]["answers"]
        .as_array()
        .expect("answers")
        .iter()
        .map(|answer| answer["details"]["answer"]["probability"].as_f64())
        .collect::<Vec<_>>();
    assert_eq!(
        rows, details,
        "each bulk row carries the details probability"
    );

    for id in [
        "37-annotate-choose-one",
        "38-annotate-score-one",
        "39-annotate-tag-one",
    ] {
        let annotate = case(id);
        let set = QuestionSet::parse(&annotate["question_set"].to_string()).expect("set");
        let [named] = set.questions() else {
            panic!("{id} asks one question")
        };
        let engine = engine(&format!("{}/case/{id}/v1", loopback.origin()));
        let text = annotate["exchanges"][0]["evidence"].as_str().expect("text");
        let scalar = engine
            .judge(named.question(), named.threshold(), evidence(text), &cancel)
            .expect("scalar");
        assert_eq!(
            serde_json::to_value(&scalar.value).expect("value"),
            annotate["expect"]["success"]["answers"][0]["bare"],
            "{id}"
        );
    }
}

#[test]
fn only_the_one_accessor_reads_the_retained_state() {
    let sources = [
        include_str!("../facade.rs"),
        include_str!("../facade/annotate.rs"),
        include_str!("../facade/recognize.rs"),
        include_str!("../facade/relate.rs"),
    ];
    let reads = sources
        .iter()
        .map(|source| {
            source
                .match_indices("self.state")
                .filter(|(at, _)| !source[at + "self.state".len()..].starts_with('('))
                .count()
        })
        .sum::<usize>();
    assert_eq!(reads, 1, "only `Engine::state` names the state field");
}

#[test]
fn no_key_reaches_a_result_an_error_a_recording_or_a_count() {
    let listener = Listener::serving(vec![
        Canned::ok(r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9}}}"#),
        Canned::close_without_reply(),
    ])
    .expect("listener");
    let folder = Scratch::new("secrecy");
    let engine = Engine::new(Settings {
        storage: Storage {
            record: Some(folder.0.clone()),
            ..Storage::default()
        },
        ..settings(listener.base())
    })
    .expect("engine");
    let cancel = Cancel::default();

    let judged = engine
        .judge(&decide("Refund?"), None, evidence("Refund me."), &cancel)
        .expect("judged");
    let failed = ask(&engine, "broken", &cancel).expect_err("closed");
    let refused = Engine::new(Settings {
        key: std::sync::Arc::new(|| Err(Error::NoKey("THINKTHEN_API_KEY"))),
        // Loopback takes a request with no key, so this engine names an
        // address the rules cannot prove is this machine.
        ..settings("https://127.0.0.2:9/v1")
    })
    .and_then(|engine| ask(&engine, "no key here", &cancel))
    .expect_err("no key");
    let seen = [
        format!("{:?} {:?}", judged.answer, judged.answered.reply),
        format!("{failed:?}"),
        format!("{refused:?}"),
        format!("{:?}", engine.usage()),
        folder.files().concat(),
    ];

    assert!(listener.requests().iter().all(|request| {
        request
            .header("authorization")
            .is_some_and(|value| value.contains(TEST_KEY))
    }));
    for text in seen {
        assert!(!text.contains(TEST_KEY), "{text}");
        assert!(!text.contains("sk-facade"), "{text}");
    }
}

/// A folder recorded for another backend refuses every call on a long-lived
/// engine, not only its first, and none of them sends.
#[test]
fn a_folder_of_another_backend_refuses_each_call() {
    let listener = Listener::serving(vec![Canned::ok(
        r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9}}}"#,
    )])
    .expect("listener");
    let folder = Scratch::new("other-backend");
    let recording = |base: &str| Settings {
        storage: Storage {
            record: Some(folder.0.clone()),
            ..Storage::default()
        },
        ..settings(base)
    };
    let cancel = Cancel::default();
    let first = Engine::new(recording(listener.base())).expect("engine");
    ask(&first, "Refund me.", &cancel).expect("the folder's own backend");
    let other = Engine::new(recording(&format!("{}/other", listener.base()))).expect("engine");

    for call in ["first", "second"] {
        let refused = ask(&other, call, &cancel);
        assert!(
            matches!(refused, Err(Error::RecordingBackendMismatch(..))),
            "{call}: {refused:?}"
        );
    }
    assert_eq!(
        listener.requests().len(),
        1,
        "only the folder's own backend sent"
    );
}
