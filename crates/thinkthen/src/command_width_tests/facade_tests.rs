//! Stays: process-wide width selection and cancellation races require direct scheduling checks.
//! The facade's width registration and a cancelled batch, under the real
//! process width in a child copy of the test binary.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

use super::{child, in_child};
use crate::public::complete::recognize::ordered::{self, Outcome};
use crate::core::{Backend, Evidence, Question, QuestionText};
use crate::engine::error::Error as EngineError;
use crate::engine::facade::{Engine, Key, Settings, Storage};
use crate::engine::{Cancel, Width, limits};

#[test]
fn the_facade_registers_its_width_and_stops_a_cancelled_batch() {
    in_child("facade_tests::facade_width_child");
}

#[test]
#[ignore = "runs alone in a child process"]
fn facade_width_child() {
    if !child() {
        return;
    }
    let loopback = conformance_backend::Backend::start().expect("loopback");
    let base = format!("{}/arm/held/v1", loopback.origin());
    let engine = Engine::new(settings(&base, Some(1))).expect("the first explicit width");
    assert_eq!(limits::process().widths.selected(), Width::new(1).ok());
    let Err(refused) = Engine::new(settings(&base, Some(2))) else {
        panic!("a conflicting width is refused")
    };
    assert!(
        matches!(refused, EngineError::WidthActive(_)),
        "{refused:?}"
    );
    assert_eq!(refused.kind().as_str(), "usage");
    assert_eq!(limits::process().widths.selected(), Width::new(1).ok());
    assert_eq!(loopback.count(), 0, "the refusal sent nothing");

    cancelled_batch_leaves_no_send(&engine, &loopback);
}

fn settings(base: &str, width: Option<u64>) -> Settings {
    Settings {
        backend: Backend::resolve(Some(base), None, "local-1").expect("backend"),
        profile: None,
        timeout: Duration::from_secs(30),
        max_retries: 0,
        retry_wait: Duration::from_millis(10),
        width: width.and_then(|width| Width::new(width).ok()),
        per_minute: None,
        storage: Storage::default(),
        key: std::sync::Arc::new(|| Ok(Key::of("sk-test-value"))),
        usage: Arc::default(),
    }
}

/// R5-19: one reply is held across a batch cancel. After its release no
/// later item sends, and the next call sends only its own request.
fn cancelled_batch_leaves_no_send(engine: &Engine, loopback: &conformance_backend::Backend) {
    let question = Question::Decide {
        text: QuestionText::new("Is it late?").expect("question"),
        yes: None,
        no: None,
    };
    let ask = |text: &str, cancel: &Cancel| {
        engine.judge(
            &question,
            None,
            Evidence::new(text).expect("evidence"),
            cancel,
        )
    };
    let batch = Cancel::default();
    // Each item asks under a token the batch cancel does not reach, so only
    // the ordered runner's stop keeps a later item from sending.
    let each = Cancel::default();
    let mut rows = Vec::new();

    let mut input = ["one", "two", "three"].into_iter();
    let outcome = thread::scope(|scope| {
        let run = scope.spawn(|| {
            ordered::run(
                1,
                &batch,
                None,
                || input.next().map(ordered::Input::Item).or(Some(ordered::Input::End)),
                &|text: &str| {
                    ask(text, &each).map(|judged| ordered::Row {
                        value: judged.answer.yes(),
                        replayed: false,
                    })
                },
                |row| {
                    rows.push(row);
                    Ok::<_, EngineError>(true)
                },
                &|stop| stop,
                || EngineError::Defect("the reader ended early"),
            )
        });
        assert_eq!(loopback.wait(1), 1, "the first item is held");
        batch.fire();
        loopback.release();
        run.join().expect("batch")
    });

    assert_eq!(loopback.count(), 1, "no later item sent after the cancel");
    assert!(
        matches!(
            outcome,
            Ok(Outcome::Stopped {
                cause: EngineError::Cancelled,
                ..
            })
        ),
        "{outcome:?}"
    );
    assert_eq!(rows, [Some(0.9)], "the held item finished");
    ask("four", &Cancel::default()).expect("the next call");
    assert_eq!(
        loopback.count(),
        2,
        "the next call sent only its own request"
    );
}

#[test]
fn native_selection_refuses_before_any_more_key_lookup_or_request() {
    in_child("facade_tests::native_selection_child");
}

#[test]
#[ignore = "runs alone in a child process"]
fn native_selection_child() {
    if !child() {
        return;
    }
    let listener = conformance_backend::Listener::answering(|body| {
        conformance_backend::Canned::ok(&super::generic(body))
    }).expect("listener");
    let cancel = Cancel::default();
    let question = Question::Decide {
        text: QuestionText::new("Is it late?").expect("question"), yes: None, no: None,
    };
    let usage = Arc::new(crate::engine::usage::Counters::default());
    let key_reads = Arc::new(AtomicUsize::new(0));
    let counted_settings = |width| {
        let mut settings = settings(listener.base(), width);
        settings.usage = Arc::clone(&usage);
        let key_reads = Arc::clone(&key_reads);
        settings.key = Arc::new(move || {
            key_reads.fetch_add(1, Ordering::SeqCst);
            Ok(Key::of("sk-test-value"))
        });
        settings
    };
    for width in [None, Some(4), Some(4)] {
        let engine = Engine::new(counted_settings(width)).expect("width admitted");
        assert_eq!(limits::process().widths.selected(), width.and_then(|width| Width::new(width).ok()));
        for text in ["one", "two"] {
            engine.judge(&question, None, Evidence::new(text).expect("evidence"), &cancel)
                .expect("answer");
        }
    }
    let before = (listener.connections(), listener.count(), cancel.keys(), usage.snapshot().requests_sent, key_reads.load(Ordering::SeqCst));
    assert_eq!(before.1, 6);
    assert_eq!(before.2, 6, "the real native producer counted its key lookups");
    assert_eq!(before.3, 6);
    assert_eq!(before.4, 6, "the configured key source ran for each request");
    let Err(refused) = Engine::new(counted_settings(Some(8))) else {
        panic!("a different width is refused")
    };
    assert!(matches!(refused, EngineError::WidthActive(_)));
    let after = (listener.connections(), listener.count(), cancel.keys(), usage.snapshot().requests_sent, key_reads.load(Ordering::SeqCst));
    assert_eq!(after, before, "no connection, request, key, or count");
    assert_eq!(limits::process().widths.selected(), Width::new(4).ok());
}
