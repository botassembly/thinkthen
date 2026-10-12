//! Native pull admission keeps snapshots, borrowed originals and caller callbacks together.
use super::*;
use std::cell::Cell;
use std::num::NonZeroUsize;
use std::rc::Rc;
use std::sync::Mutex;
use thinkthen::{BatchSetting, InputEvidence, QuestionInput, QuestionSet, RecordInput};

#[test]
fn seven_native_batches_refuse_recognition_controls_when_pulled_without_sending() {
    let listener = Listener::answering(|_| Canned::ok("unused")).unwrap();
    let engine = engine(&listener);
    let decide = Question::decide("Fits?").unwrap().cut();
    let choose = Question::from_json(r#"{"choose":"Which?","options":["a","b"]}"#).unwrap();
    let tag = Question::from_json(r#"{"tag":"Which?","labels":["a","b"]}"#).unwrap();
    let thinkthen::LoadedQuestion::Question(score) =
        Question::from_json(r#"{"score":"How?","levels":["low","high"]}"#).unwrap()
    else {
        panic!("score")
    };
    let dynamic = Question::choose_records("Which?").unwrap();
    let set =
        QuestionSet::from_json(r#"{"version":1,"questions":{"a":{"decide":"Fits?"}}}"#).unwrap();
    let pulled = Cell::new(0);
    let records = || {
        std::iter::once_with(|| {
            pulled.set(pulled.get() + 1);
            Ok(RecordInput {
                seed_spans: Some(vec![]),

                ..crate::api::record("First.")
            })
        })
    };
    macro_rules! probe {
        ($method:ident, $question:expr) => {
            refuses_controls(
                crate::api::$method(&engine, $question, records(), CallOptions::new()),
                &pulled,
                &listener,
            )
        };
    }
    probe!(pull_decisions, &decide);
    probe!(pull_choices, &choose);
    probe!(pull_tags, &tag);
    probe!(pull_scores, &score);
    probe!(pull_filters, &decide);
    refuses_controls(
        crate::api::pull_choices(&engine, &dynamic, records(), CallOptions::new()),
        &pulled,
        &listener,
    );
    let batch = crate::api::annotation_rows(
        crate::api::rows(
            &engine,
            thinkthen::RequestFunction::Annotate,
            set.into(),
            records(),
            CallOptions::new(),
        )
        .unwrap(),
    )
    .unwrap();
    refuses_controls(batch, &pulled, &listener);
}

#[cfg(test)]
fn refuses_controls<T: std::fmt::Debug>(
    mut batch: thinkthen::Batch<'_, T>,
    pulled: &Cell<usize>,
    listener: &Listener,
) {
    assert_eq!(pulled.get(), 0);
    assert_eq!(listener.count(), 0);
    let error = batch.next().unwrap().unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert!(error.detail().message().contains("recognition controls"));
    assert_eq!(pulled.get(), 1);
    assert!(batch.next().is_none());
    assert_eq!(listener.count(), 0);
    pulled.set(0);
}

struct Original<'a> {
    snapshots: &'a Cell<usize>,
    drops: &'a Cell<usize>,
    local: Rc<()>,
}
impl InputEvidence for Original<'_> {
    fn question_input(&self) -> QuestionInput {
        let before = self.snapshots.replace(self.snapshots.get() + 1);
        QuestionInput::Text(if before == 0 { "First." } else { "Changed." }.into())
    }
}
impl Drop for Original<'_> {
    fn drop(&mut self) {
        self.drops.set(self.drops.get() + 1);
    }
}

#[test]
fn borrowed_local_original_snapshots_once_and_early_drop_keeps_callbacks_on_the_caller() {
    let listener =
        crate::api::answering(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#);
    let engine = engine(&listener);
    let question = Question::decide("Fits?").unwrap().cut();
    let pulled = Cell::new(0);
    let snapshots = Cell::new(0);
    let drops = Cell::new(0);
    let local = Rc::new(());
    let caller = std::thread::current().id();
    let interruptions = Mutex::new(0);
    let observations = Mutex::new(0);
    let interrupt = || {
        assert_eq!(std::thread::current().id(), caller);
        *interruptions.lock().unwrap() += 1;
        false
    };
    let observer = |_: thinkthen::RecordObservation<'_>| {
        assert_eq!(std::thread::current().id(), caller);
        *observations.lock().unwrap() += 1;
    };
    let records = (0..3).map(|_| {
        pulled.set(pulled.get() + 1);
        Ok(crate::api::record(Original {
            snapshots: &snapshots,
            drops: &drops,
            local: Rc::clone(&local),
        }))
    });
    let mut batch = crate::api::pull_decisions(
        &engine,
        &question,
        records,
        CallOptions::new()
            .batch(BatchSetting::Records(NonZeroUsize::new(1).unwrap()))
            .interrupt(&interrupt)
            .observe(&observer)
            .attempts(true),
    );
    assert_eq!(pulled.get(), 0);
    assert_eq!(listener.count(), 0);
    let row = batch.next().unwrap().unwrap();
    assert_eq!(row.ordinal(), 0);
    assert_eq!(row.result().value(), Answer::Yes);
    assert_eq!(Rc::strong_count(&row.original().local), 2);
    assert_eq!(snapshots.get(), 1);
    drop(batch);
    assert_eq!(pulled.get(), 1);
    assert_eq!(drops.get(), 0);
    drop(row);
    assert_eq!(drops.get(), 1);
    assert_eq!(Rc::strong_count(&local), 1);
    assert!(*interruptions.lock().unwrap() > 0);
    assert!(*observations.lock().unwrap() > 0);
    assert_eq!(listener.count(), 1);
    let request: Value = serde_json::from_slice(&listener.requests()[0].body).unwrap();
    assert!(
        request["questions"]["q1"]["instructions"]
            .as_str()
            .unwrap()
            .contains("First.")
    );
}
