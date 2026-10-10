//! Nullable typed batches retain caller originals and rejected filter observations.
use super::common;
use conformance_backend::{Canned, Listener};
use std::cell::Cell;
use std::rc::Rc;
use thinkthen::polars::prelude::{DataType, NamedFrom, Series};
use thinkthen::{CallOptions, InputEvidence, Question, QuestionInput, RecordInput};

struct Original<'a> {
    id: u32,
    snapshots: &'a Cell<usize>,
    drops: Rc<Cell<usize>>,
}
impl InputEvidence for Original<'_> {
    fn question_input(&self) -> QuestionInput {
        self.snapshots.set(self.snapshots.get() + 1);
        QuestionInput::Text("Same.".into())
    }
}
impl Drop for Original<'_> {
    fn drop(&mut self) {
        self.drops.set(self.drops.get() + 1);
    }
}
fn records<'a>(
    snapshots: &'a Cell<usize>,
    drops: &Rc<Cell<usize>>,
) -> Vec<Option<RecordInput<Original<'a>>>> {
    [Some(7), None, Some(9)]
        .into_iter()
        .map(|id| {
            id.map(|id| RecordInput {
                original: Original {
                    id,
                    snapshots,
                    drops: Rc::clone(drops),
                },
                context: Some("row guide".into()),
                options: None,
                examples: None,
                seed_spans: None,
            })
        })
        .collect()
}
#[test]
fn typed_pulled_filter_keeps_nullable_positions_snapshots_and_rejected_originals() {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.1}}}"#)
    })
    .expect("listener");
    let engine = common::builder(listener.base())
        .no_cache()
        .build()
        .expect("engine");
    let question = Question::decide("Fits?").expect("question").cut();
    let column = Series::new("original".into(), [Some(7u32), None, Some(9)]);
    let snapshots = Cell::new(0);
    let drops = Rc::new(Cell::new(0));
    let (batch, positions) = engine
        .filter_input_column_batch(
            &question,
            &column,
            records(&snapshots, &drops),
            CallOptions::new(),
        )
        .expect("batch");
    assert_eq!(positions, [0, 2]);
    assert_eq!(column.dtype(), &DataType::UInt32);
    assert_eq!(snapshots.get(), 0);
    assert_eq!(listener.count(), 0);
    let call = batch.into_call().expect("complete filter");
    assert_eq!(snapshots.get(), 2);
    assert_eq!(call.facts().records(), 2);
    assert_eq!(call.facts().requests_sent(), 1);
    for (at, row) in call.value().iter().enumerate() {
        assert_eq!(row.ordinal(), at);
        assert_eq!(row.original().id, [7, 9][at]);
        assert!(!row.result().value());
        assert!(!row.result().answer_id().as_str().is_empty());
    }
    assert_eq!(drops.get(), 0);
    drop(call);
    assert_eq!(drops.get(), 2);
    assert_eq!(listener.count(), 1);
    let body: serde_json::Value =
        serde_json::from_slice(&listener.requests()[0].body).expect("body");
    assert_eq!(body["state"], "row guide");
}
