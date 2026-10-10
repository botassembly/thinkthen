//! Reordered native results retain nullable frame associations and owned inputs.
use super::common;
use conformance_backend::{Canned, Listener};
use std::{cell::Cell, rc::Rc};
use thinkthen::polars::prelude::{DataType, NamedFrom, Series};
use thinkthen::{CallOptions, InputEvidence, Question, QuestionInput, RankSet, RecordInput};

struct Original {
    id: u32,
    drops: Rc<Cell<usize>>,
}
impl InputEvidence for Original {
    fn question_input(&self) -> QuestionInput {
        QuestionInput::Text(if self.id == 7 { "weak" } else { "strong" }.into())
    }
}
impl Drop for Original {
    fn drop(&mut self) {
        self.drops.set(self.drops.get() + 1);
    }
}
fn records(drops: &Rc<Cell<usize>>) -> Vec<Option<RecordInput<Original>>> {
    [Some(7), None, Some(9), Some(11)]
        .into_iter()
        .map(|id| {
            id.map(|id| RecordInput {
                original: Original {
                    id,
                    drops: drops.clone(),
                },
                context: Some("row guide".into()),
                options: None,
                examples: None,
                seed_spans: None,
            })
        })
        .collect()
}
fn rank() -> Result<Question, thinkthen::Error> {
    Question::rank_from_json(r#"{"decide":"Relevant?","threshold":"0.5"}"#)
}
fn set() -> Result<RankSet, thinkthen::Error> {
    RankSet::from_json(
        r#"{"version":1,"questions":{"first":{"decide":"First?"},"second":{"decide":"Second?"}}}"#,
    )
}
#[test]
fn typed_rank_restores_reordered_and_omitted_nonclone_originals() {
    let listener = Listener::answering(|body| {
        let body: serde_json::Value = serde_json::from_slice(body).expect("body");
        let answers = body["questions"].as_object().expect("questions").iter().map(|(key, question)| {
            let probability = if question.to_string().contains("weak") { 0.2 } else { 0.9 };
            (key.clone(), serde_json::json!({"type":"noul", "noul":probability}))
        }).collect::<serde_json::Map<_, _>>();
        Canned::ok(&serde_json::json!({"model":"jev-latest","answers":answers,"usage":{"input_tokens":10,"output_tokens":2}}).to_string())
    }).expect("listener");
    let engine = common::engine(listener.base());
    let column = Series::new("original".into(), [Some(7u32), None, Some(9), Some(11)]);
    let drops = Rc::new(Cell::new(0));
    let singleton = || {
        CallOptions::new().batch(thinkthen::BatchSetting::Records(
            std::num::NonZeroUsize::MIN,
        ))
    };
    let (call, positions) = engine
        .rank_input_column_complete(
            &rank().expect("rank question"),
            &column,
            records(&drops),
            singleton(),
        )
        .expect("rank");
    assert_eq!(positions, [0, 2, 3]);
    assert_eq!(column.dtype(), &DataType::UInt32);
    assert_eq!(
        call.value()
            .iter()
            .map(|row| (row.ordinal(), row.original().id, row.result().value()))
            .collect::<Vec<_>>(),
        [(1, 9, 1), (2, 11, 2)]
    );
    assert_eq!(drops.get(), 1);
    assert_eq!(call.facts().records(), 3);
    for row in call.value() {
        assert!(!row.result().answer_id().as_str().is_empty());
    }
    drop(call);
    assert_eq!(drops.get(), 3);
    let (call, positions) = engine
        .rank_set_input_column_complete(
            &set().expect("rank set"),
            &column,
            records(&drops),
            singleton(),
        )
        .expect("rank set");
    assert_eq!(positions, [0, 2, 3]);
    assert_eq!(
        call.value()
            .iter()
            .map(|row| (
                row.ordinal(),
                row.original().id,
                row.result().result().value()
            ))
            .collect::<Vec<_>>(),
        [(1, 9, 1), (2, 11, 2), (0, 7, 3)]
    );
    for row in call.value() {
        assert_eq!(row.result().members().len(), 2);
        assert!(!row.result().result().answer_id().as_str().is_empty());
    }
    assert_eq!(call.facts().records(), 3);
    assert_eq!(drops.get(), 3);
    drop(call);
    assert_eq!(drops.get(), 6);
    assert!(
        listener
            .requests()
            .iter()
            .all(|body| String::from_utf8_lossy(&body.body).contains("row guide"))
    );
}
#[test]
fn typed_rank_admits_all_controls_before_sending() {
    let listener = Listener::answering(|_| Canned::ok("{}")).expect("listener");
    let engine = common::engine(listener.base());
    let drops = Rc::new(Cell::new(0));
    let column = Series::new("original".into(), [Some(7u32), None, Some(9), Some(11)]);
    for ranked_set in [false, true] {
        let mut inputs = records(&drops);
        inputs[3].as_mut().expect("last row").options = Some(
            thinkthen::RecordOptions::new(
                ["one", "two"]
                    .into_iter()
                    .map(|name| thinkthen::RecordOption {
                        name: name.into(),
                        description: None,
                    })
                    .collect(),
            )
            .expect("options"),
        );
        let error = if ranked_set {
            engine
                .rank_set_input_column_complete(
                    &set().expect("rank set"),
                    &column,
                    inputs,
                    CallOptions::new(),
                )
                .map(|_| ())
                .expect_err("invalid set")
        } else {
            engine
                .rank_input_column_complete(
                    &rank().expect("rank question"),
                    &column,
                    inputs,
                    CallOptions::new(),
                )
                .map(|_| ())
                .expect_err("invalid rank")
        };
        assert_eq!(error.kind(), thinkthen::ErrorKind::Usage);
        let token = thinkthen::CancelToken::new();
        token.cancel();
        let error = if ranked_set {
            engine
                .rank_set_input_column_complete(
                    &set().expect("rank set"),
                    &column,
                    records(&drops),
                    CallOptions::new().cancel(&token),
                )
                .map(|_| ())
                .expect_err("cancel set")
        } else {
            engine
                .rank_input_column_complete(
                    &rank().expect("rank question"),
                    &column,
                    records(&drops),
                    CallOptions::new().cancel(&token),
                )
                .map(|_| ())
                .expect_err("cancel rank")
        };
        assert_eq!(error.kind(), thinkthen::ErrorKind::Cancelled);
        for cells in [vec![], vec![None, None]] {
            let text = Series::new("body".into(), cells as Vec<Option<&str>>);
            let (count, facts, positions) = if ranked_set {
                let (call, positions) = engine
                    .rank_set_series_complete(&set().expect("rank set"), &text, CallOptions::new())
                    .expect("empty set");
                (call.value().len(), call.facts().requests_sent(), positions)
            } else {
                let (call, positions) = engine
                    .rank_series_complete(
                        &rank().expect("rank question"),
                        &text,
                        CallOptions::new(),
                    )
                    .expect("empty rank");
                (call.value().len(), call.facts().requests_sent(), positions)
            };
            assert_eq!(count, 0);
            assert_eq!(facts, 0);
            assert!(positions.is_empty());
        }
    }
    assert_eq!(listener.count(), 0);
}
