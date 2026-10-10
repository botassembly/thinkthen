//! Native row results retain owned originals through Request admission.
use super::common;
use conformance_backend::{Canned, Listener};
use thinkthen::polars::prelude::{DataType, NamedFrom, Series};
use thinkthen::{CallOptions, InputEvidence, Question, QuestionInput, QuestionSet, RecordInput};

struct Original {
    id: u32,
    text: String,
}
impl InputEvidence for Original {
    fn question_input(&self) -> QuestionInput {
        QuestionInput::Text(self.text.clone())
    }
}
fn records(text: &str) -> Vec<Option<RecordInput<Original>>> {
    [Some(7), None, Some(9)]
        .into_iter()
        .map(|id| {
            id.map(|id| RecordInput {
                original: Original {
                    id,
                    text: text.into(),
                },
                context: Some("row guide".into()),
                options: None,
                examples: None,
                seed_spans: None,
            })
        })
        .collect()
}
fn set() -> Result<QuestionSet, thinkthen::Error> {
    QuestionSet::from_json(
        r#"{"version":1,"questions":{"relevant":{"decide":"Relevant?","threshold":"0.1:0.9"}}}"#,
    )
}
#[test]
fn complete_decide_and_annotate_keep_banded_results_owned_originals_and_context() {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.5}},"usage":{"input_tokens":49,"output_tokens":0}}"#)
    }).expect("listener");
    let engine = common::engine(listener.base());
    let question = Question::decide("Relevant?")
        .expect("decide")
        .band(0.1, 0.9)
        .expect("band");
    let column = Series::new("original".into(), [Some(7u32), None, Some(9)]);
    let (call, positions) = engine
        .decide_input_column_complete(&question, &column, records("same"), CallOptions::new())
        .expect("decisions");
    assert_eq!(positions, [0, 2]);
    for (at, row) in call.value().iter().enumerate() {
        assert_eq!(row.ordinal(), at);
        assert_eq!(row.original().id, [7, 9][at]);
        assert_eq!(row.result().value(), thinkthen::Answer::Unsure);
        assert!(!row.result().answer_id().as_str().is_empty());
    }
    assert_eq!(call.facts().records(), 2);
    assert_eq!(call.facts().input_tokens(), Some(49));
    let (call, positions) = engine
        .annotate_input_column_complete(
            &set().expect("set"),
            &column,
            records("same"),
            CallOptions::new(),
        )
        .expect("annotations");
    assert_eq!(positions, [0, 2]);
    for (at, row) in call.value().iter().enumerate() {
        assert_eq!(row.ordinal(), at);
        assert_eq!(row.original().id, [7, 9][at]);
        let member = row.result().members().next().expect("member");
        assert_eq!(
            member.value(),
            Some(thinkthen::Judgment::Decision(thinkthen::Answer::Unsure))
        );
        assert!(!row.result().answer_id().as_str().is_empty());
    }
    assert_eq!(call.facts().records(), 2);
    assert_eq!(column.dtype(), &DataType::UInt32);
    assert_eq!(listener.count(), 2);
    for request in listener.requests() {
        let body: serde_json::Value = serde_json::from_slice(&request.body).expect("body");
        assert!(body["state"].to_string().contains("row guide"));
    }
}
#[test]
fn complete_recognition_retains_saved_spans_with_nonclone_originals() {
    let corpus: serde_json::Value =
        serde_json::from_str(include_str!("../../../../conformance/cases.json")).expect("corpus");
    let case = corpus["cases"]
        .as_array()
        .expect("cases")
        .iter()
        .find(|case| case["id"] == "42-recognize-C01-relations")
        .expect("case");
    let question =
        thinkthen::Recognize::from_json(&case["question"].to_string()).expect("recognize");
    let replies = case["exchanges"].as_array().expect("exchanges");
    let responses = (0..2)
        .flat_map(|_| {
            replies
                .iter()
                .map(|r| Canned::ok(&r["response"].to_string()))
        })
        .collect();
    let listener = Listener::serving(responses).expect("listener");
    let engine = common::builder(listener.base())
        .no_cache()
        .model("jev-1.13.0")
        .expect("model")
        .build()
        .expect("engine");
    let column = Series::new("original".into(), [Some(7u32), None, Some(9)]);
    let (call, positions) = engine
        .recognize_input_column_complete(
            &question,
            &column,
            records(case["text"].as_str().expect("text")),
            CallOptions::new(),
        )
        .expect("recognition");
    assert_eq!(positions, [0, 2]);
    assert_eq!(call.value().len(), 2);
    for (at, row) in call.value().iter().enumerate() {
        assert_eq!(row.ordinal(), at);
        assert_eq!(row.original().id, [7, 9][at]);
        assert!(!row.result().answer_id().as_str().is_empty());
        assert_eq!(
            row.result()
                .value()
                .entities()
                .iter()
                .map(|e| (e.text(), e.start(), e.end(), e.kind()))
                .collect::<Vec<_>>(),
            [
                ("Maria Chen", 0, 10, "person"),
                ("Northwind Freight", 18, 35, "organization"),
                ("Chicago", 39, 46, "place"),
            ]
        );
        assert_eq!(
            row.result()
                .value()
                .relations()
                .expect("relations")
                .iter()
                .map(|r| r.relation())
                .collect::<Vec<_>>(),
            ["works_for"]
        );
    }
    assert_eq!(call.facts().records(), 2);
    assert_eq!(column.dtype(), &DataType::UInt32);
    assert_eq!(listener.requests().len(), replies.len() * 2);
}
#[test]
fn complete_row_admission_refuses_late_controls_and_cancellation_without_sends() {
    let listener = Listener::answering(|_| Canned::ok("{}")).expect("listener");
    let engine = common::engine(listener.base());
    let decide = Question::decide("Relevant?").expect("decide").cut();
    let recognize = thinkthen::Recognize::from_json(
        r#"{"version":1,"recognize":{"kinds":{"person":"A person name."}}}"#,
    )
    .expect("recognize");
    for cells in [vec![], vec![None, None]] {
        let column = Series::new("text".into(), cells as Vec<Option<&str>>);
        let (call, positions) = engine
            .decide_series_complete(&decide, &column, CallOptions::new())
            .expect("empty decide");
        assert!(call.value().is_empty());
        assert!(positions.is_empty());
        assert_eq!(call.facts().requests_sent(), 0);
        let (call, positions) = engine
            .annotate_series_complete(&set().expect("set"), &column, CallOptions::new())
            .expect("empty annotate");
        assert!(call.value().is_empty());
        assert!(positions.is_empty());
        assert_eq!(call.facts().requests_sent(), 0);
        let (call, positions) = engine
            .recognize_series_complete(&recognize, &column, CallOptions::new())
            .expect("empty recognize");
        assert!(call.value().is_empty());
        assert!(positions.is_empty());
        assert_eq!(call.facts().requests_sent(), 0);
    }
    let column = Series::new("original".into(), [Some(7u32), None, Some(9)]);
    for which in 0..3 {
        let mut invalid = records("same");
        invalid[2].as_mut().expect("last").options = Some(
            thinkthen::RecordOptions::new(vec![
                thinkthen::RecordOption {
                    name: "forbidden".into(),
                    description: None,
                },
                thinkthen::RecordOption {
                    name: "other".into(),
                    description: None,
                },
            ])
            .expect("options"),
        );
        let error = match which {
            0 => engine
                .decide_input_column_complete(&decide, &column, invalid, CallOptions::new())
                .err(),
            1 => engine
                .annotate_input_column_complete(
                    &set().expect("set"),
                    &column,
                    invalid,
                    CallOptions::new(),
                )
                .err(),
            _ => engine
                .recognize_input_column_complete(&recognize, &column, invalid, CallOptions::new())
                .err(),
        }
        .expect("late invalid control");
        assert_eq!(error.kind(), thinkthen::ErrorKind::Usage);
        let token = thinkthen::CancelToken::new();
        token.cancel();
        let options = CallOptions::new().cancel(&token);
        let error = match which {
            0 => engine
                .decide_input_column_complete(&decide, &column, records("same"), options)
                .err(),
            1 => engine
                .annotate_input_column_complete(
                    &set().expect("set"),
                    &column,
                    records("same"),
                    options,
                )
                .err(),
            _ => engine
                .recognize_input_column_complete(&recognize, &column, records("same"), options)
                .err(),
        }
        .expect("cancelled");
        assert_eq!(error.kind(), thinkthen::ErrorKind::Cancelled);
    }
    assert_eq!(listener.count(), 0);
}
