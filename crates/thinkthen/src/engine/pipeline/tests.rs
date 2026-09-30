//! Whole-call stops through the question pipeline, held against a counted
//! loopback listener.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

use conformance_backend::{Canned, Listener};

use super::{Answered, Asker, Failed, Flow, Packing, Port, reader as host};
use crate::core::pack::{self, Ask};
use crate::core::{Evidence, ModelName, Question, QuestionText, Url, quoted_plan};
use crate::engine::error::Error;
use crate::engine::pipeline::Input;
use crate::engine::{Cancel, Deadline};

/// Asks one decide question about `line N` for each number it reads.
struct Lines(Url);

impl Asker for Lines {
    type Input = usize;
    type Row = usize;
    type Error = ();

    fn label(&self, input: &usize) -> usize {
        *input
    }

    fn asks(&self, input: &usize) -> Result<Vec<Ask>, ()> {
        let question = Question::Decide {
            text: QuestionText::new("Does it name a place?").map_err(|_| ())?,
            yes: None,
            no: None,
        };
        let plan = quoted_plan(
            (
                ModelName::new(crate::core::DEFAULT_MODEL).map_err(|_| ())?,
                crate::core::Descriptions::Authored,
            ),
            Evidence::new(format!("line {input}")).map_err(|_| ())?,
            None,
            vec![question],
            None,
        )
        .map_err(|_| ())?;
        pack::asks(&self.0, &plan).map_err(|_| ())
    }

    fn row(&self, input: usize, _answers: Vec<Answered>) -> Result<usize, ()> {
        Ok(input)
    }
}

/// A reader that answers each ask with the next number, then the end, and
/// counts the asks.
fn reader(count: usize, asks: &Arc<AtomicUsize>) -> impl FnOnce(Receiver<()>, Port<usize, ()>) {
    let asks = Arc::clone(asks);
    move |asked, port| {
        thread::spawn(move || feed(count, &asked, &port, &asks));
    }
}

fn feed(count: usize, asked: &Receiver<()>, port: &Port<usize, ()>, asks: &AtomicUsize) {
    let mut next = (1..=count).map(Input::Item).chain([Input::End]);
    while asked.recv().is_ok()
        && let Some(input) = next.next()
    {
        asks.fetch_add(1, Ordering::SeqCst);
        let last = matches!(input, Input::End);
        if port.send(input).is_err() || last {
            break;
        }
    }
}

const ONE_EACH: Packing = Packing {
    questions: None,
    sized: true,
    inputs: Some(1),
    context: false,
    detailed: false,
    continues: false,
};

fn stop_of(failed: &Failed<()>) -> &'static str {
    match failed {
        Failed::Stopped(Error::Deadline(_))
        | Failed::Engine {
            error: Error::Deadline(_),
            ..
        } => "deadline",
        Failed::Stopped(Error::Cancelled) => "cancelled",
        _ => "other",
    }
}

#[test]
fn a_spent_deadline_or_a_fired_cancel_stops_before_reading() {
    let listener = Listener::answering(|_| Canned::ok("{}")).expect("listener");
    let engine = crate::engine::facade_tests::engine(listener.base());
    let asker = Lines(engine.backend().url().clone());
    let fired = Cancel::default();
    fired.fire();
    let spent = Cancel::default().with_deadline(Deadline::after(Duration::ZERO));
    for (cancel, expected) in [(spent, "deadline"), (fired, "cancelled")] {
        let asks = Arc::new(AtomicUsize::new(0));
        let mut emitted = Vec::new();
        engine
            .ask_all(
                &asker,
                ONE_EACH,
                host(reader(3, &asks), |_, row: Result<usize, Failed<()>>| {
                    emitted.push(row.map_err(|failed| stop_of(&failed)));
                    Flow::Stop
                }),
                &cancel,
            )
            .expect("the call runs");
        assert_eq!(emitted, [Err(expected)], "{expected}");
        assert_eq!(asks.load(Ordering::SeqCst), 0, "{expected}: no input read");
    }
    assert_eq!(listener.count(), 0);
}

/// A deadline that passes while requests are slow starts none of the
/// requests still waiting for a worker.
#[test]
fn a_deadline_starts_no_waiting_request() {
    let listener = Listener::answering(|_| Canned::ok("{}").after(600)).expect("listener");
    let engine = crate::engine::facade_tests::engine(listener.base());
    let asker = Lines(engine.backend().url().clone());
    let cancel = Cancel::default().with_deadline(Deadline::after(Duration::from_millis(300)));
    let asks = Arc::new(AtomicUsize::new(0));
    let mut emitted = Vec::new();
    engine
        .ask_all(
            &asker,
            ONE_EACH,
            host(reader(40, &asks), |_, row: Result<usize, Failed<()>>| {
                emitted.push(row.map_err(|failed| stop_of(&failed)));
                Flow::Stop
            }),
            &cancel,
        )
        .expect("the call runs");
    let sent = listener.count();
    thread::sleep(Duration::from_millis(100));
    assert_eq!(
        listener.count(),
        sent,
        "nothing starts after the call returns"
    );
    assert!(sent <= 4, "{sent} requests started past the workers");
    assert!(asks.load(Ordering::SeqCst) < 40, "the reader stopped");
    assert_eq!(emitted, [Err("deadline")]);
}
