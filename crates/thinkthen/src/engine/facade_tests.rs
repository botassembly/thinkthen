//! The private facade reaches every function through the production owners.
//!
//! Each test drives the loopback conformance backend and counts what it read.

use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

use conformance_backend::Backend as Loopback;

use crate::core::recording::Exchange as Recorded;
use crate::core::{
    AnswerOutcome, Backend, Evidence, Find, ModelName, Plan, Question, QuestionSet, QuestionText,
    RecognizeSpec, RelateSpec, Value,
};
use crate::engine::Cancel;
use crate::engine::error::Error;
use crate::engine::facade::{Engine, Input, Key, RunOutcome, Settings, Storage, relations};
use crate::engine::schedule::{Completed, InputPort};

pub(super) const TEST_KEY: &str = "sk-facade-test-7f3a";

pub(super) fn settings(base: &str) -> Settings {
    Settings {
        backend: Backend::resolve(Some(base), None, "jev-latest").expect("backend"),
        profile: None,
        timeout: Duration::from_secs(5),
        max_retries: 0,
        retry_wait: Duration::from_millis(10),
        width: None,
        storage: Storage::default(),
        key: || Ok(Key::of(TEST_KEY)),
        usage: Arc::default(),
    }
}

pub(super) fn engine(base: &str) -> Engine {
    Engine::new(settings(base)).expect("engine")
}

pub(super) fn generic(loopback: &Loopback) -> String {
    format!("{}/generic/v1", loopback.origin())
}

pub(super) fn decide(text: &str) -> Question {
    Question::Decide {
        text: QuestionText::new(text).expect("question"),
        yes: None,
        no: None,
    }
}

pub(super) fn evidence(text: &str) -> Evidence {
    Evidence::new(text).expect("evidence")
}

/// Start a reader that hands one item per request, then the end.
pub(super) fn reader<T: Send + 'static, R: Send + 'static>(
    items: Vec<T>,
) -> impl FnOnce(Receiver<()>, InputPort<T, R, Error>) {
    move |requests, events| {
        thread::spawn(move || feed(items, &requests, &events));
    }
}

fn feed<T, R>(items: Vec<T>, requests: &Receiver<()>, events: &InputPort<T, R, Error>) {
    let mut inputs = items
        .into_iter()
        .map(Input::Item)
        .chain(std::iter::once(Input::End));
    while requests.recv().is_ok() {
        let Some(input) = inputs.next() else { return };
        if events.send(input).is_err() {
            return;
        }
    }
}

/// The digest one request body is filed under at this backend.
pub(super) fn digest(engine_base: &str, body: &[u8]) -> String {
    let backend = Backend::resolve(Some(engine_base), None, "jev-latest").expect("backend");
    Recorded::new(backend.url(), body)
        .digest()
        .as_str()
        .to_owned()
}

#[test]
fn a_scalar_judgment_crosses_the_facade_once() {
    let loopback = Loopback::start().expect("loopback");
    let base = generic(&loopback);
    let engine = engine(&base);

    let judged = engine
        .judge(
            &decide("Does this ask for a refund?"),
            None,
            evidence("Please refund my order."),
            &Cancel::default(),
        )
        .expect("the judgment");

    assert_eq!(judged.value, Value::YesNo(Some(true)));
    assert_eq!(judged.answer.yes(), Some(0.9));
    assert_eq!(judged.answered.requests_sent, 1);
    assert!(!judged.answered.replayed);
    assert_eq!(judged.answered.reply.model().as_str(), "jev-latest");
    assert_eq!(loopback.count(), 1);
}

#[test]
fn an_ordinary_bulk_call_answers_each_record_in_input_order() {
    let loopback = Loopback::start().expect("loopback");
    let engine = engine(&generic(&loopback));
    let question = decide("Does this ask for a refund?");
    let mut rows = Vec::new();

    let outcome = engine
        .records(
            false,
            &Cancel::default(),
            reader(vec!["one", "two", "three"]),
            &|text: &&str| {
                engine
                    .judge(&question, None, evidence(text), &Cancel::default())
                    .map(|judged| Completed {
                        value: ((*text).to_owned(), judged.answer.yes()),
                        replayed: judged.answered.replayed,
                        partial_failure: false,
                    })
            },
            |row| {
                rows.push(row);
                Ok(true)
            },
        )
        .expect("the run");

    assert!(matches!(outcome, RunOutcome::Complete));
    assert_eq!(
        rows,
        [
            ("one".to_owned(), Some(0.9)),
            ("two".to_owned(), Some(0.9)),
            ("three".to_owned(), Some(0.9)),
        ]
    );
    assert_eq!(loopback.count(), 3);
}

#[test]
fn grouped_annotation_keeps_named_order_through_split_requests() {
    let loopback = Loopback::start().expect("loopback");
    let engine = engine(&generic(&loopback));
    let set = QuestionSet::parse(
        r#"{"version":1,"questions":{"refund":{"decide":"Does this ask for a refund?"},"team":{"choose":"Which team owns this?","options":["billing","other"]}}}"#,
    )
    .expect("question set");
    let mut places = Vec::new();

    for group in set.groups() {
        let questions = group
            .iter()
            .map(|place| set.questions()[*place].question().clone())
            .collect();
        let plan = Plan::new(
            evidence("Refund the card."),
            ModelName::new("jev-latest").expect("model"),
            questions,
        )
        .expect("plan");
        let chunks = engine.split(&plan).expect("chunks");
        let mut group = group.into_iter();
        engine
            .ask_chunks(chunks, &Cancel::default(), |answered| {
                for outcome in answered.reply.outcomes() {
                    assert!(matches!(outcome, AnswerOutcome::Answered(_)));
                    places.push(group.next().expect("a place for each answer"));
                }
                Ok::<(), Error>(())
            })
            .expect("the group");
    }

    assert_eq!(places, [0, 1]);
    assert_eq!(loopback.count(), 1);
}

#[test]
fn an_aggregate_find_returns_its_selection_and_every_candidate() {
    let loopback = Loopback::start().expect("loopback");
    let engine = engine(&generic(&loopback));
    let units = [evidence("First passage."), evidence("Second passage.")];
    let find = Find::new(
        QuestionText::new("Which passage answers?").expect("question"),
        &units,
        ModelName::new("jev-latest").expect("model"),
        true,
    )
    .expect("find");

    let found = engine.find(&find, &Cancel::default()).expect("found");

    assert_eq!(found.selection.selected(), Some(0));
    assert_eq!(found.answered.requests_sent, 1);
    assert_eq!(loopback.count(), 1);
}

#[test]
fn recognition_runs_its_stages_through_the_facade() {
    let loopback = Loopback::start().expect("loopback");
    let engine = engine(&generic(&loopback));
    let spec = RecognizeSpec::parse(
        r#"{"version":1,"recognize":{"kinds":{"person":"A person's name.","place":"A place name."},"relations":[{"name":"lives_in","source":"person","target":"place"}]}}"#,
    )
    .expect("spec");

    let recognition = engine
        .recognize(&spec, "Ada lives in Paris.", &Cancel::default())
        .expect("recognition");

    assert_eq!(
        recognition.inputs.len(),
        crate::core::tokenize("Ada lives in Paris.").len()
    );
    assert!(recognition.value.relations.is_some());
    assert_eq!(recognition.meta.requests.len(), loopback.count());
    assert_eq!(
        usize::try_from(recognition.meta.requests_sent).expect("count"),
        loopback.count()
    );
}

#[test]
fn relating_given_entities_runs_the_shared_planner_through_the_facade() {
    let loopback = Loopback::start().expect("loopback");
    let base = generic(&loopback);
    let engine = engine(&base);
    let spec = RelateSpec::parse(
        r#"{"version":1,"relate":{"relations":[{"name":"caused_by","source":"alert","target":"alert"}]}}"#,
    )
    .expect("spec");
    let entities = spec
        .admit(&[
            ("Checkout fails.".to_owned(), "alert".to_owned()),
            ("The disk is full.".to_owned(), "alert".to_owned()),
        ])
        .expect("entities");
    let backend = Backend::resolve(Some(&base), None, "jev-latest").expect("backend");
    let prepared = relations(&entities, &spec, &backend, None).expect("prepared");
    let bodies: Vec<Vec<u8>> = prepared
        .iter()
        .flat_map(|relation| relation.chunks.iter())
        .map(|chunk| chunk.request.body.clone())
        .collect();

    let execution = engine
        .relate(prepared, &entities, 0.5, &Cancel::default())
        .expect("relations");

    assert_eq!(execution.failed, 0);
    assert_eq!(execution.logical.len(), execution.answered);
    assert_eq!(
        execution.requests,
        bodies
            .iter()
            .map(|body| digest(&base, body))
            .collect::<Vec<_>>()
    );
    assert_eq!(loopback.count(), bodies.len());
}
