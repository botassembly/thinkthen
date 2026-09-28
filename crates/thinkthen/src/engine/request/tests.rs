//! Focused request cancellation and recording behavior.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, atomic::AtomicUsize, atomic::Ordering, mpsc};
use std::thread;
use std::time::Duration;

use crate::core::{Backend, Evidence, ModelName, Plan, Question, QuestionText};
use crate::engine::cache_lock;
use crate::engine::error::Error;
use crate::engine::http::Key;
use crate::engine::prepared_request::{Answered, PreparedRequest};
use crate::engine::recorder::Recorder;
use crate::engine::usage::Counters;
use crate::public::SendBudget;

fn request() -> (Backend, Plan, PreparedRequest) {
    let backend = Backend::resolve(Some("http://127.0.0.1:1/v1/systemone"), None, "jev-latest")
        .expect("backend");
    let plan = Plan::new(
        Evidence::new("evidence").expect("evidence"),
        ModelName::new("jev-latest").expect("model"),
        vec![Question::Decide {
            text: QuestionText::new("Is this relevant?").expect("question"),
            yes: None,
            no: None,
        }],
    )
    .expect("plan");
    let prepared = PreparedRequest::new(&backend, &plan).expect("request");
    (backend, plan, prepared)
}

type Counts = (Arc<AtomicUsize>, Arc<AtomicUsize>);

fn folder(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!("thinkthen-cancel-{label}-{}", std::process::id()))
}

fn has_partial(folder: &Path) -> bool {
    let prefix = format!(".{}.", std::process::id());
    fs::read_dir(folder)
        .expect("recording folder")
        .filter_map(Result::ok)
        .any(|entry| entry.file_name().to_string_lossy().starts_with(&prefix))
}

fn wait_on_request(
    request: (Backend, Plan, PreparedRequest),
    recorder: Recorder,
    observed: (crate::engine::Cancel<'static>, Counts),
) -> thread::JoinHandle<Result<Answered, Error>> {
    let (backend, plan, prepared) = request;
    let (cancel, counts) = observed;
    thread::spawn(move || {
        super::ask_prepared(
            &backend,
            &plan,
            prepared,
            &recorder,
            &cancel,
            &Counters::default(),
            || {
                counts.0.fetch_add(1, Ordering::SeqCst);
                Ok(Key::of("unused"))
            },
            |_, _| {
                counts.1.fetch_add(1, Ordering::SeqCst);
                Err(Error::Defect("send unexpectedly reached"))
            },
        )
    })
}

#[test]
fn a_writer_seen_after_an_early_key_refusal_keeps_hit_or_mismatch() {
    for (name, winner_url, matching, zero_limit, outer_key) in [
        (
            "zero-budget-race-hit",
            "http://127.0.0.1:1/v1/systemone",
            true,
            true,
            "outer key",
        ),
        (
            "zero-budget-race-mismatch",
            "http://127.0.0.1:2/v1/systemone",
            false,
            true,
            "outer key",
        ),
        (
            "line-break-race-hit",
            "http://127.0.0.1:1/v1/systemone",
            true,
            false,
            "outer\nkey",
        ),
        (
            "line-break-race-mismatch",
            "http://127.0.0.1:2/v1/systemone",
            false,
            false,
            "outer\nkey",
        ),
    ] {
        let path = folder(name);
        let _absent = fs::remove_dir_all(&path);
        let backend = Backend::resolve(Some("http://127.0.0.1:1/v1/systemone"), None, "local-1")
            .expect("outer backend");
        let winner = Backend::resolve(Some(winner_url), None, "local-1").expect("winner backend");
        let plan = Plan::new(
            Evidence::new("evidence").expect("evidence"),
            ModelName::new("local-1").expect("model"),
            vec![Question::Decide {
                text: QuestionText::new("Is this relevant?").expect("question"),
                yes: None,
                no: None,
            }],
        )
        .expect("plan");
        let prepared = PreparedRequest::new(&backend, &plan).expect("outer request");
        let recorder = Recorder::of(Some(&path), Some(&path)).expect("outer cache");
        let budget = SendBudget::new();
        let cancel = if zero_limit {
            crate::engine::Cancel::default().with_send_budget(Some((budget, Some(0))))
        } else {
            crate::engine::Cancel::default()
        };
        let sends = Arc::new(AtomicUsize::new(0));
        let winner_sends = Arc::clone(&sends);
        let result: Result<Answered, Error> = super::ask_prepared(
            &backend,
            &plan,
            prepared,
            &recorder,
            &cancel,
            &Counters::default(),
            || {
                let prepared = PreparedRequest::new(&winner, &plan).expect("winner request");
                let recorder = Recorder::of(Some(&path), Some(&path)).expect("winner cache");
                let answered: Result<Answered, Error> = super::ask_prepared(
                    &winner,
                    &plan,
                    prepared,
                    &recorder,
                    &crate::engine::Cancel::default(),
                    &Counters::default(),
                    || Ok(Key::of("winner key")),
                    |_, _| {
                        winner_sends.fetch_add(1, Ordering::SeqCst);
                        Ok(crate::engine::http::HttpAnswer {
                            body: br#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}}}"#.to_vec(),
                            requests_sent: 1,
                        })
                    },
                );
                answered.expect("winner fills cache during outer key lookup");
                Ok(Key::of(outer_key))
            },
            |_, _| Err(Error::Defect("outer send reached")),
        );
        if matching {
            assert!(result.expect("matching answer").replayed);
        } else {
            assert!(matches!(result, Err(Error::RecordingBackendMismatch(..))));
        }
        assert_eq!(sends.load(Ordering::SeqCst), 1, "only winner sent");
        assert!(path.join(".thinkthen-backend.json").is_file());
        fs::remove_dir_all(path).expect("fixture removed");
    }
}

#[test]
fn a_stop_after_writable_admission_keeps_the_bound_marker() {
    let path = folder("after-admission-stop");
    let _absent = fs::remove_dir_all(&path);
    let (backend, plan, prepared) = request();
    let entry = prepared.digest.file_name();
    let recorder = Recorder::of(Some(&path), Some(&path)).expect("cache recorder");
    let result: Result<Answered, Error> = super::ask_prepared(
        &backend,
        &plan,
        prepared,
        &recorder,
        &crate::engine::Cancel::default(),
        &Counters::default(),
        || Ok(Key::of("valid key")),
        |_, _| {
            assert!(path.join(".thinkthen-backend.json").is_file());
            Err(Error::Cancelled)
        },
    );
    assert!(matches!(result, Err(Error::Cancelled)));
    assert!(path.join(".thinkthen-backend.json").is_file());
    assert!(!path.join(entry).exists());
    fs::remove_dir_all(path).expect("fixture removed");
}

#[test]
fn a_stop_during_an_early_bad_key_wins_without_binding() {
    let path = folder("stop-during-bad-key");
    let _absent = fs::remove_dir_all(&path);
    let (backend, plan, prepared) = request();
    let recorder = Recorder::of(Some(&path), Some(&path)).expect("cache recorder");
    let cancel = crate::engine::Cancel::default();
    let result: Result<Answered, Error> = super::ask_prepared(
        &backend,
        &plan,
        prepared,
        &recorder,
        &cancel,
        &Counters::default(),
        || {
            cancel.fire();
            Ok(Key::of("first\nsecond"))
        },
        |_, _| Err(Error::Defect("send unexpectedly reached")),
    );
    assert!(matches!(result, Err(Error::Cancelled)));
    assert!(!path.exists());
}

#[test]
fn a_held_folder_cancels_through_the_prepared_request_path() {
    let path = folder("folder");
    let _absent = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).expect("recording folder");
    // A bound folder waits before key lookup. An unbound empty folder now
    // inspects the key first under ADR 0099.
    let (backend, _, prepared) = request();
    let seed = Recorder::of(Some(&path), None).expect("recording owner");
    let crate::engine::recorder::PreparedRecording::Live(permit) = seed
        .prepare_cancelled(
            &prepared.recorded(&backend),
            &prepared.digest,
            &crate::engine::Cancel::default(),
        )
        .expect("folder bound")
    else {
        unreachable!("empty recording cannot replay");
    };
    permit.cancel().expect("abandon seed write");
    let owner = cache_lock::exclusive_folder(&path).expect("exclusive owner");
    let (blocked_send, blocked) = mpsc::channel();
    let cancel = crate::engine::Cancel::observed(blocked_send);
    let counts = Counts::default();
    let waiter = wait_on_request(
        request(),
        Recorder::of(Some(&path), None).expect("recorder"),
        (cancel.clone(), counts.clone()),
    );

    blocked
        .recv_timeout(Duration::from_secs(2))
        .expect("shared lock received WouldBlock");
    cancel.fire();
    assert!(matches!(
        waiter.join().expect("waiter"),
        Err(Error::Cancelled)
    ));
    assert_eq!(counts.0.load(Ordering::SeqCst), 0);
    assert_eq!(counts.1.load(Ordering::SeqCst), 0);
    drop(owner);
    assert!(!has_partial(&path));
    fs::remove_dir_all(path).expect("fixture removed");
}

#[test]
fn a_held_digest_cancels_through_the_prepared_request_path() {
    let path = folder("digest");
    let _absent = fs::remove_dir_all(&path);
    let (owner_backend, _, owner_prepared) = request();
    let recorder = Recorder::of(Some(&path), None).expect("recorder");
    let owner_recorded = owner_prepared.recorded(&owner_backend);
    let crate::engine::recorder::PreparedRecording::Live(owner) = recorder
        .prepare_cancelled(
            &owner_recorded,
            &owner_prepared.digest,
            &crate::engine::Cancel::default(),
        )
        .expect("owner prepared")
    else {
        unreachable!("empty recording cannot replay");
    };
    let (blocked_send, blocked) = mpsc::channel();
    let cancel = crate::engine::Cancel::observed(blocked_send);
    let counts = Counts::default();
    let (backend, plan, prepared) = request();
    let digest = prepared.digest.as_str().to_owned();
    let waiter = wait_on_request(
        (backend, plan, prepared),
        recorder,
        (cancel.clone(), counts.clone()),
    );

    blocked
        .recv_timeout(Duration::from_secs(2))
        .expect("digest lock received WouldBlock");
    cancel.fire();
    assert!(matches!(
        waiter.join().expect("waiter"),
        Err(Error::Cancelled)
    ));
    assert_eq!(counts.0.load(Ordering::SeqCst), 0);
    assert_eq!(counts.1.load(Ordering::SeqCst), 0);
    owner.cancel().expect("owner cleanup");
    assert!(!has_partial(&path));
    assert!(path.join(".locks").join(digest).exists());
    fs::remove_dir_all(path).expect("fixture removed");
}

#[test]
fn cleanup_failure_at_the_cancel_boundary_wins_and_keeps_owned_state() {
    let (backend, _plan, prepared) = request();
    let folder = folder("prepared");
    let _absent = fs::remove_dir_all(&folder);
    let recorder = Recorder::of(Some(&folder), None).expect("recorder");
    let cancel = crate::engine::Cancel::default();
    let operation = recorder
        .prepare_cancelled(&prepared.recorded(&backend), &prepared.digest, &cancel)
        .expect("prepared recording");
    let lock = folder.join(".locks").join(prepared.digest.as_str());
    assert!(has_partial(&folder));

    crate::engine::recorder::fail_cleanup();
    cancel.fire();
    let result = super::observe_cancel::<Error>(operation, &cancel);

    assert!(matches!(result, Err(Error::RecordingStorage)));
    assert!(has_partial(&folder));
    assert!(lock.exists());
    fs::remove_dir_all(folder).expect("fixture removed");
}

#[test]
fn a_received_usage_report_survives_a_refused_logical_reply() {
    let (backend, plan, prepared) = request();
    let facts = crate::engine::CallFacts::new();
    let cancel = crate::engine::Cancel::default().with_facts(facts.clone());
    let recorder = Recorder::of(None, None).expect("uncached recorder");
    let usage = Counters::new(None);
    let result: Result<Answered, Error> = super::ask_prepared(
        &backend,
        &plan,
        prepared,
        &recorder,
        &cancel,
        &usage,
        || Ok(Key::of("sk-test")),
        |_, _| {
            Ok(crate::engine::http::HttpAnswer {
                body: br#"{"model":"jev-latest","answers":{},"usage":{"input_tokens":7,"output_tokens":3}}"#.to_vec(),
                requests_sent: 1,
            })
        },
    );
    assert!(result.is_err(), "the reply has no answer for q1");
    assert_eq!(
        facts
            .snapshot()
            .tokens
            .map(crate::core::Usage::token_counts),
        Some((7, 3))
    );
    assert_eq!(usage.snapshot().input_tokens, 7);
    assert_eq!(usage.snapshot().output_tokens, 3);
}

#[test]
fn a_complete_live_refresh_that_fails_before_rename_preserves_old_entry() {
    let (backend, plan, prepared) = request();
    let folder = folder("refresh-install");
    let _absent = fs::remove_dir_all(&folder);
    let name = prepared.digest.file_name();
    let recorder = Recorder::of(Some(&folder), Some(&folder)).expect("cache recorder");
    let cancel = crate::engine::Cancel::default();
    let usage = Counters::new(None);
    let reply = |model: &str| {
        format!(r#"{{"model":"{model}","answers":{{"q1":{{"type":"noul","noul":0.9}}}}}}"#)
    };
    let first: Result<Answered, Error> = super::ask_prepared(
        &backend,
        &plan,
        prepared,
        &recorder,
        &cancel,
        &usage,
        || Ok(Key::of("sk-test")),
        |_, _| {
            Ok(crate::engine::http::HttpAnswer {
                body: reply("jev-1.13.0").into_bytes(),
                requests_sent: 1,
            })
        },
    );
    assert!(first.is_ok(), "seed a complete old cache entry");
    let entry = folder.join(name);
    let before = fs::read(&entry).expect("old entry");
    let (backend, plan, prepared) = request();
    let second: Result<Answered, Error> = super::ask_prepared(
        &backend,
        &plan,
        prepared,
        &recorder,
        &cancel,
        &usage,
        || Ok(Key::of("sk-test")),
        |_, _| {
            let answer = crate::engine::http::HttpAnswer {
                body: reply("jev-1.14.0").into_bytes(),
                requests_sent: 1,
            };
            crate::engine::recorder::fail_install();
            Ok(answer)
        },
    );
    assert!(matches!(second, Err(Error::RecordingStorage)));
    assert_eq!(fs::read(&entry).expect("old entry after refusal"), before);
    assert!(!has_partial(&folder));
    fs::remove_dir_all(folder).expect("fixture removed");
}
