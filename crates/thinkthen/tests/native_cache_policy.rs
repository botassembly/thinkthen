//! Response storage instructions through actual public calls and the command.

use conformance_backend::{Canned, Listener};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use thinkthen::{BatchSetting, CallOptions, Engine, ErrorKind, Question};

const YES: &str = r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#;
const NO: &str = r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.1}}}"#;
const BODY: &[u8] = br#"{"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"Refund me.\". Refund?"}}}"#;

struct Folder(PathBuf);

impl Folder {
    fn new() -> std::io::Result<Self> {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
            "native-cache-policy-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        if path.exists() {
            std::fs::remove_dir_all(&path)?;
        }
        Ok(Self(path))
    }
}

impl Drop for Folder {
    fn drop(&mut self) {
        let _removed = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn repeated_fields_and_exceeded_parser_bounds_forbid_actual_cache_writes() {
    let limits = [
        vec!["max-age=30".to_owned(), "no-store".to_owned()],
        vec!["x".repeat(8193)],
        vec!["x".to_owned(); 9],
        vec![vec!["x"; 65].join(",")],
    ];
    for fields in limits {
        let folder = Folder::new().unwrap();
        let mut reply = Canned::ok(YES);
        for field in fields {
            reply = reply.asking("Cache-Control", &field);
        }
        let listener = Listener::serving(vec![reply, Canned::ok(NO)]).unwrap();
        let engine = crate::api::fixed(listener.base(), "fixture-storage-policy")
            .cache_at(&folder.0)
            .unwrap()
            .build()
            .unwrap();
        let question = Question::decide("Refund?").unwrap().cut();
        assert_eq!(
            engine.decide(&question, "Refund me.").unwrap().value(),
            &Some(true)
        );
        assert!(!folder.0.join("thinkthen.sqlite").exists());
        assert_eq!(
            engine.decide(&question, "Refund me.").unwrap().value(),
            &Some(false)
        );
        let requests = listener.requests();
        assert_eq!(requests.len(), 2);
        assert!(requests.iter().all(|request| request.body == BODY));
    }
}

#[test]
fn explicit_record_forbids_every_good_answer_of_a_nonstorable_partial_packed_reply() {
    let folder = Folder::new().unwrap();
    let listener =
        Listener::answering(|_| Canned::ok(YES).asking("Cache-Control", "no-store")).unwrap();
    let engine = crate::api::fixed(listener.base(), "fixture-storage-policy")
        .record(&folder.0)
        .unwrap()
        .build()
        .unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let options = CallOptions::new().batch(BatchSetting::Records(
        std::num::NonZeroUsize::new(2).unwrap(),
    ));
    let mut batch = crate::api::pull_decisions(
        &engine,
        &question,
        crate::api::plain_records(["alpha", "beta"]),
        options,
    );
    let errors = batch.by_ref().collect::<Vec<_>>();
    assert_eq!(errors.len(), 1); // The released batch stops at its first terminal failure.
    for row in errors {
        let error = row.unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Local);
        assert_eq!(
            error.to_string(),
            "the reply forbids storage, so the recording cannot be written"
        );
    }
    assert_eq!(batch.facts().unwrap().requests_sent(), 1);
    assert_eq!(listener.count(), 1);
    assert!(!folder.0.join("thinkthen.sqlite").exists());
    let requests = listener.requests();
    assert_eq!(requests[0].body, br#"{"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"alpha\". Refund?"},"q2":{"type":"noul","instructions":"The text is \"beta\". Refund?"}}}"#);
}

#[test]
fn native_refresh_marks_status_retries_and_split_children_without_changing_request_bodies() {
    let listener = Listener::serving(vec![
        Canned::status(503, "{}").asking("retry-after-ms", "1"),
        Canned::status(413, "{}"),
        Canned::ok(YES),
        Canned::ok(YES),
    ])
    .unwrap();
    let engine = crate::api::fixed(listener.base(), "fixture-storage-policy")
        .refresh_cache(true)
        .max_retries(1)
        .no_cache()
        .build()
        .unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let options = CallOptions::new().batch(BatchSetting::Records(
        std::num::NonZeroUsize::new(2).unwrap(),
    ));
    let mut batch = crate::api::pull_decisions(
        &engine,
        &question,
        crate::api::plain_records(["alpha", "beta"]),
        options,
    );
    assert_eq!(
        batch.by_ref().collect::<Result<Vec<_>, _>>().unwrap().len(),
        2
    );
    assert_eq!(batch.facts().unwrap().requests_sent(), 4);
    let requests = listener.requests();
    assert_eq!(requests.len(), 4);
    let parent = br#"{"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"alpha\". Refund?"},"q2":{"type":"noul","instructions":"The text is \"beta\". Refund?"}}}"#.as_slice();
    let expected = [parent, parent,
        br#"{"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"alpha\". Refund?"}}}"#.as_slice(),
        br#"{"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"beta\". Refund?"}}}"#.as_slice(),
    ];
    for (request, body) in requests.iter().zip(expected) {
        assert_eq!(request.header("Cache-Control"), Some("no-cache"));
        assert_eq!(request.body, body);
    }
}

#[test]
fn partial_nonstorable_refresh_evicts_accepted_answers_and_preserves_failed_question_history() {
    let folder = Folder::new().unwrap();
    let both = r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.9}}}"#;
    let listener = Listener::serving(vec![
        Canned::ok(both),
        Canned::ok(NO).asking("Cache-Control", "no-store"),
        Canned::ok(NO),
    ])
    .unwrap();
    let builder = || {
        Engine::builder()
            .base_url(listener.base())
            .and_then(|b| b.model("fixed"))
            .and_then(|b| b.api_key("fixture-storage-policy"))
            .and_then(|b| b.cache_at(&folder.0))
    };
    let engine = builder().unwrap().build().unwrap();
    let refresh = builder().unwrap().refresh_cache(true).build().unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let options = || {
        CallOptions::new().batch(BatchSetting::Records(
            std::num::NonZeroUsize::new(2).unwrap(),
        ))
    };
    assert_eq!(
        crate::api::pull_decisions(
            &engine,
            &question,
            crate::api::plain_records(["alpha", "beta"]),
            options()
        )
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
        .len(),
        2
    );
    let mut rows = crate::api::pull_decisions(
        &refresh,
        &question,
        crate::api::plain_records(["alpha", "beta"]),
        options(),
    );
    assert!(rows.next().unwrap().is_ok());
    assert_eq!(rows.next().unwrap().unwrap_err().kind(), ErrorKind::Backend);
    assert!(rows.next().is_none());
    let failed = engine.decide(&question, "beta").unwrap();
    assert_eq!(failed.value(), &Some(true));
    assert_eq!(failed.facts().requests_sent(), 0);
    let accepted = engine.decide(&question, "alpha").unwrap();
    assert_eq!(accepted.value(), &Some(false));
    assert_eq!(accepted.facts().requests_sent(), 1);
    let requests = listener.requests();
    assert_eq!(requests.len(), 3);
    let parent = br#"{"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"alpha\". Refund?"},"q2":{"type":"noul","instructions":"The text is \"beta\". Refund?"}}}"#.as_slice();
    assert_eq!(requests[0].body, parent);
    assert_eq!(requests[1].body, parent);
    assert_eq!(requests[2].body, br#"{"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"alpha\". Refund?"}}}"#);
}

#[test]
fn nonstorable_answers_succeed_without_reuse_and_valid_quoted_text_still_caches() {
    for header in ["No-StOrE", "private=\"unterminated", "max-age=30, no-store"] {
        let folder = Folder::new().unwrap();
        let listener = Listener::serving(vec![
            Canned::ok(YES).asking("Cache-Control", header),
            Canned::ok(NO),
        ])
        .unwrap();
        let engine = crate::api::fixed(listener.base(), "fixture-storage-policy")
            .cache_at(&folder.0)
            .unwrap()
            .build()
            .unwrap();
        let question = Question::decide("Refund?").unwrap().cut();
        let first = engine.decide(&question, "Refund me.").unwrap();
        assert_eq!(first.value(), &Some(true));
        let second = engine.decide(&question, "Refund me.").unwrap();
        assert_eq!(second.value(), &Some(false));
        assert_eq!(second.facts().requests_sent(), 1);
        let held = engine.decide(&question, "Refund me.").unwrap();
        assert_eq!(held.value(), &Some(false));
        assert_eq!(held.facts().requests_sent(), 0);
        let requests = listener.requests();
        assert_eq!(requests.len(), 2);
        assert!(requests.iter().all(|request| request.body == BODY));
    }
    let folder = Folder::new().unwrap();
    let listener = Listener::answering(|_| {
        Canned::ok(YES).asking("Cache-Control", "private=\"no-store\", max-age=30")
    })
    .unwrap();
    let engine = crate::api::fixed(listener.base(), "fixture-storage-policy")
        .cache_at(&folder.0)
        .unwrap()
        .build()
        .unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    assert_eq!(
        engine.decide(&question, "Refund me.").unwrap().value(),
        &Some(true)
    );
    assert_eq!(
        engine
            .decide(&question, "Refund me.")
            .unwrap()
            .facts()
            .requests_sent(),
        0
    );
    assert_eq!(listener.count(), 1);
}

#[test]
fn explicit_record_refuses_after_one_paid_send_with_started_facts_and_no_answer_write() {
    let folder = Folder::new().unwrap();
    let listener =
        Listener::answering(|_| Canned::ok(YES).asking("Cache-Control", "no-store")).unwrap();
    let engine = crate::api::fixed(listener.base(), "fixture-storage-policy")
        .record(&folder.0)
        .unwrap()
        .max_retries(3)
        .build()
        .unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let error = engine.decide(&question, "Refund me.").unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Local);
    assert_eq!(
        error.to_string(),
        "the reply forbids storage, so the recording cannot be written"
    );
    let facts = error.facts().unwrap();
    assert_eq!(facts.requests_sent(), 1);
    assert!(facts.call_id().is_some());
    assert_eq!(listener.count(), 1);
    assert!(!folder.0.join("thinkthen.sqlite").exists());
    assert!(!folder.0.join("thinkthen.jsonl").exists());
}

#[path = "support/api.rs"]
mod api;
