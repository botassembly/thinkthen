//! Whole-set native results retain owned inputs and nullable column positions.
use super::common;
use conformance_backend::{Canned, Listener};
use std::{cell::Cell, rc::Rc};
use thinkthen::polars::prelude::{DataType, NamedFrom, Series};
use thinkthen::{CallOptions, InputEvidence, Question, QuestionInput, RecordInput, Relate};

struct Original {
    id: u32,
    drops: Rc<Cell<usize>>,
}
impl InputEvidence for Original {
    fn question_input(&self) -> QuestionInput {
        QuestionInput::Text(if self.id == 7 { "Ada" } else { "Bo" }.into())
    }
}
impl Drop for Original {
    fn drop(&mut self) {
        self.drops.set(self.drops.get() + 1);
    }
}
fn records(drops: &Rc<Cell<usize>>, ids: &[Option<u32>]) -> Vec<Option<RecordInput<Original>>> {
    ids.iter()
        .map(|id| {
            id.map(|id| RecordInput {
                original: Original {
                    id,
                    drops: drops.clone(),
                },
                context: None,
                options: None,
                examples: None,
                seed_spans: None,
            })
        })
        .collect()
}
fn relation() -> Result<Relate, thinkthen::Error> {
    Relate::from_json(
        r#"{"version":1,"relate":{"relations":[{"name":"knows","source":"*","target":"*"}]}}"#,
    )
}
#[test]
fn typed_find_keeps_duplicate_candidates_none_and_nonclone_originals() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"jev-latest","answers":{"q1":{"type":"choice","probabilities":{"u001":0.1,"u002":0.8,"u003":0.05,"none":0.05}}},"usage":{"input_tokens":10,"output_tokens":2}}"#)).expect("listener");
    let engine = common::engine(listener.base());
    let ids = [Some(7u32), None, Some(9), Some(11)];
    let column = Series::new("original".into(), ids);
    let drops = Rc::new(Cell::new(0));
    let question = Question::find("Which?")
        .and_then(Question::offering_none)
        .expect("find");
    let (call, positions) = engine
        .find_input_column_complete(
            &question,
            &column,
            records(&drops, &ids),
            CallOptions::new().context("whole set"),
        )
        .expect("find");
    assert_eq!(positions, [0, 2, 3]);
    assert_eq!(column.dtype(), &DataType::UInt32);
    assert_eq!(call.value().selection(), thinkthen::FindSelection::Unit(1));
    assert_eq!(call.value().selected().expect("selected").id, 9);
    assert_eq!(
        call.value()
            .candidates()
            .iter()
            .map(|row| (row.input().map(|input| input.id), row.probability()))
            .collect::<Vec<_>>(),
        [
            (Some(7), 0.1),
            (Some(9), 0.8),
            (Some(11), 0.05),
            (None, 0.05)
        ]
    );
    assert!(!call.value().answer_id().as_str().is_empty());
    assert!(!call.value().identity().observations().is_empty());
    assert_eq!(call.facts().requests_sent(), 1);
    assert_eq!(drops.get(), 0);
    drop(call);
    assert_eq!(drops.get(), 3);
    assert!(String::from_utf8_lossy(&listener.requests()[0].body).contains("whole set"));
    let column = Series::new("body".into(), [Some("Ada"), None, Some("Bo"), Some("Bo")]);
    let (call, positions) = engine
        .find_series_complete(&question, &column, CallOptions::new())
        .expect("wrapper");
    assert_eq!(positions, [0, 2, 3]);
    assert_eq!(call.value().selected().map(String::as_str), Some("Bo"));
}
#[test]
fn typed_relate_keeps_ordered_owned_set_and_native_edges() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.8}},"usage":{"input_tokens":10,"output_tokens":2}}"#)).expect("listener");
    let engine = common::engine(listener.base());
    let ids = [Some(7u32), None, Some(9)];
    let column = Series::new("original".into(), ids);
    let drops = Rc::new(Cell::new(0));
    let question = relation().expect("relate");
    let (call, positions) = engine
        .relate_input_column_complete(
            &question,
            &column,
            records(&drops, &ids),
            CallOptions::new().context("whole set"),
        )
        .expect("relate");
    assert_eq!(positions, [0, 2]);
    assert_eq!(column.dtype(), &DataType::UInt32);
    assert_eq!(call.value().ordinal(), 0);
    assert_eq!(
        call.value()
            .original()
            .iter()
            .map(|input| input.id)
            .collect::<Vec<_>>(),
        [7, 9]
    );
    assert_eq!(
        call.value()
            .result()
            .value()
            .iter()
            .map(|edge| (edge.source().name(), edge.target().name()))
            .collect::<Vec<_>>(),
        [("Ada", "Bo"), ("Bo", "Ada")]
    );
    assert!(!call.value().result().answer_id().as_str().is_empty());
    assert!(!call.value().result().identity().observations().is_empty());
    assert_eq!(call.facts().requests_sent(), 1);
    assert_eq!(drops.get(), 0);
    drop(call);
    assert_eq!(drops.get(), 2);
    assert!(String::from_utf8_lossy(&listener.requests()[0].body).contains("whole set"));
    let column = Series::new("body".into(), [Some("Ada"), None, Some("Bo")]);
    let (call, positions) = engine
        .relate_series_complete(&question, &column, CallOptions::new())
        .expect("wrapper");
    assert_eq!(positions, [0, 2]);
    assert_eq!(call.value().original(), &["Ada", "Bo"]);
}
#[test]
fn typed_aggregates_refuse_late_controls_and_cancel_without_sending() {
    let listener = Listener::answering(|_| Canned::ok("{}")).expect("listener");
    let engine = common::engine(listener.base());
    let ids = [Some(7u32), None, Some(9)];
    let column = Series::new("original".into(), ids);
    let drops = Rc::new(Cell::new(0));
    for function in ["find", "relate"] {
        for cancelled in [false, true] {
            let mut inputs = records(&drops, &ids);
            let token = thinkthen::CancelToken::new();
            let options = if cancelled {
                token.cancel();
                CallOptions::new().cancel(&token)
            } else {
                inputs[2].as_mut().expect("last").context = Some("invalid per-row context".into());
                CallOptions::new()
            };
            let error = if function == "find" {
                engine
                    .find_input_column_complete(
                        &Question::find("Which?").expect("find"),
                        &column,
                        inputs,
                        options,
                    )
                    .expect_err("refuse")
            } else {
                engine
                    .relate_input_column_complete(
                        &relation().expect("relate"),
                        &column,
                        inputs,
                        options,
                    )
                    .expect_err("refuse")
            };
            assert_eq!(
                error.kind(),
                if cancelled {
                    thinkthen::ErrorKind::Cancelled
                } else {
                    thinkthen::ErrorKind::Usage
                }
            );
            assert!(error.facts().is_none_or(|facts| facts.requests_sent() == 0));
        }
    }
    assert_eq!(drops.get(), 8);
    assert_eq!(listener.count(), 0);
}
