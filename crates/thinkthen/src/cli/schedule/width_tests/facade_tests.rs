//! The facade's width registration and a cancelled batch, under the real
//! process width in a child copy of the test binary.

use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

use super::{child, in_child};
use crate::cli::schedule::ordered::{self, Outcome, Port};
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
    let settings = |width| Settings {
        backend: Backend::resolve(Some(&base), None, "local-1").expect("backend"),
        profile: None,
        timeout: Duration::from_secs(5),
        max_retries: 0,
        retry_wait: Duration::from_millis(10),
        width: Width::new(width).ok(),
        per_minute: None,
        storage: Storage::default(),
        key: std::sync::Arc::new(|| Ok(Key::of("sk-test-value"))),
        usage: Arc::default(),
    };
    let engine = Engine::new(settings(1)).expect("the first explicit width");
    assert_eq!(limits::process().widths.selected(), Width::new(1).ok());
    let Err(refused) = Engine::new(settings(2)) else {
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

/// Hand the batch its three items, one per request.
fn feed(requests: &Receiver<()>, events: &Port<&'static str, Option<f64>, EngineError>) {
    for text in ["one", "two", "three"] {
        if requests.recv().is_err() || events.send(ordered::Input::Item(text)).is_err() {
            return;
        }
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

    let outcome = thread::scope(|scope| {
        let run = scope.spawn(|| {
            ordered::run(
                1,
                &batch,
                |requests, events| {
                    thread::spawn(move || feed(&requests, &events));
                },
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
