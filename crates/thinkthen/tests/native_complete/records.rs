use super::*;
use std::rc::Rc;
use thinkthen::{BatchSetting, Description, Evidence, RecordInput, RecordOption, RecordOptions};

struct Original {
    id: usize,
    text: &'static str,
    local: Rc<()>,
}
impl Evidence for Original {
    fn evidence(&self) -> &str {
        self.text
    }
}
#[cfg(test)]
fn original(id: usize) -> Original {
    Original {
        id,
        text: "Same.",
        local: Rc::new(()),
    }
}

#[test]
fn native_originals_need_no_clone_send_or_serialization_and_context_controls_wire_identity() {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#)
    })
    .unwrap();
    let folder = folder();
    let build = || {
        Engine::builder()
            .base_url(listener.base())
            .unwrap()
            .model("fixed")
            .unwrap()
            .api_key("complete-private")
            .unwrap()
            .max_retries(0)
    };
    let engine = build().cache_at(&folder).unwrap().build().unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let inputs = || {
        [None, None, Some(""), Some("Other.")]
            .into_iter()
            .enumerate()
            .map(|(at, context)| RecordInput {
                original: original(at),
                context: context.map(Into::into),
                options: None,
            })
    };
    let call = engine
        .decide_records_complete_with(
            &question,
            inputs(),
            CallOptions::new()
                .batch(BatchSetting::Max)
                .context("Fallback.")
                .attempts(true),
        )
        .unwrap();
    let rows = call.value();
    assert_eq!(call.facts().records(), 4);
    for (at, row) in rows.iter().enumerate() {
        assert_eq!(row.ordinal(), at);
        assert_eq!(row.original().id, at);
        assert_eq!(Rc::strong_count(&row.original().local), 1);
    }
    assert_eq!(
        rows[0].result().identity().observations(),
        rows[1].result().identity().observations()
    );
    assert_ne!(rows[0].result().answer_id(), rows[1].result().answer_id());
    assert_ne!(
        rows[0].result().identity().observations(),
        rows[2].result().identity().observations()
    );
    assert_ne!(
        rows[2].result().identity().observations(),
        rows[3].result().identity().observations()
    );
    let mut expected: Vec<String> = ["Fallback.","Each question quotes the text it asks about.","Other."].into_iter().map(|state|json!({"state":state,"model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"Same.\". Refund?"}}}).to_string()).collect();
    let mut actual: Vec<String> = listener
        .requests()
        .into_iter()
        .map(|request| {
            serde_json::from_slice::<Value>(&request.body)
                .unwrap()
                .to_string()
        })
        .collect();
    expected.sort();
    actual.sort();
    assert_eq!(actual, expected);
    assert_eq!(listener.count(), 3);
    for row in rows {
        let doc: Value = serde_json::from_str(&row.result().to_json().unwrap()).unwrap();
        assert_eq!(doc["meta"]["attempts"].as_array().unwrap().len(), 1);
    }
    let replay = build().replay(&folder).unwrap().build().unwrap();
    let held = replay
        .decide_records_complete_with(&question, inputs(), CallOptions::new().context("Fallback."))
        .unwrap();
    for (one, other) in rows.iter().zip(held.value()) {
        assert_eq!(one.result().answer_id(), other.result().answer_id());
    }
    assert_eq!(listener.count(), 3);
    drop(replay);
    drop(engine);
    std::fs::remove_dir_all(folder).unwrap();
}

#[test]
fn replacement_shortlists_remain_whole_ordered_and_per_question_coalescing_keeps_distinct_occurrences()
 {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"choice","probabilities":{"b":0.7,"a":0.3}},"q2":{"type":"choice","probabilities":{"d":0.8,"c":0.2}}}}"#)).unwrap();
    let engine = engine(&listener);
    let fixed = Question::choose_labels("Which?")
        .unwrap()
        .label("x", None)
        .unwrap()
        .label("y", None)
        .unwrap()
        .build()
        .unwrap();
    let first = RecordOptions::new(vec![
        RecordOption {
            name: "b".into(),
            description: Some(Description::from_json(r#"["bold",{"what":"B"}]"#).unwrap()),
        },
        RecordOption {
            name: "a".into(),
            description: None,
        },
    ])
    .unwrap();
    let second = RecordOptions::project(r#"{"choices":{"d":"D","c":null}}"#, "/choices").unwrap();
    let rows = engine
        .choose_records_complete_with(
            &fixed,
            [
                RecordInput {
                    original: original(0),
                    context: None,
                    options: Some(first.clone()),
                },
                RecordInput {
                    original: original(1),
                    context: None,
                    options: Some(second),
                },
                RecordInput {
                    original: original(2),
                    context: None,
                    options: Some(first),
                },
            ],
            CallOptions::new().batch(BatchSetting::Max),
        )
        .unwrap();
    assert_eq!(
        rows.value()
            .iter()
            .map(|row| row.result().value())
            .collect::<Vec<_>>(),
        [Some("b"), Some("d"), Some("b")]
    );
    assert_eq!(
        rows.value()[0].result().identity().observations(),
        rows.value()[2].result().identity().observations()
    );
    assert_ne!(
        rows.value()[0].result().answer_id(),
        rows.value()[2].result().answer_id()
    );
    assert_eq!(listener.count(), 1);
    assert_eq!(listener.questions(), 2);
    assert_eq!(
        std::str::from_utf8(&listener.requests()[0].body).unwrap(),
        r#"{"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"choice","instructions":"The text is \"Same.\". Which?","criteria":{"b":["bold",{"what":"B"}],"a":null}},"q2":{"type":"choice","instructions":"The text is \"Same.\". Which?","criteria":{"d":"D","c":null}}}}"#
    );
    let invalid = engine
        .decide_records_complete_with(
            &Question::decide("Refund?").unwrap().cut(),
            [RecordInput {
                original: original(0),
                context: None,
                options: Some(
                    RecordOptions::project(r#"{"options":["a","b"]}"#, "/options").unwrap(),
                ),
            }],
            CallOptions::new(),
        )
        .unwrap_err();
    assert_eq!(invalid.kind(), ErrorKind::Usage);
    assert_eq!(listener.count(), 1);
}

#[test]
fn numeric_rank_positions_keep_original_occurrences_stable_ties_and_saved_score_values() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"score","probabilities":{"0":0.8,"1":0.2}},"q2":{"type":"score","probabilities":{"0":0.1,"1":0.9}}}}"#)).unwrap();
    let engine = engine(&listener);
    let score = Question::rank_from_json(r#"{"score":"Grade?","levels":["low","high"]}"#).unwrap();
    let rows = engine
        .rank_complete_with(
            &score,
            ["first", "second", "first"],
            CallOptions::new().batch(BatchSetting::Max),
        )
        .unwrap();
    assert_eq!(
        rows.value()
            .iter()
            .map(|row| row.ordinal())
            .collect::<Vec<_>>(),
        [1, 0, 2]
    );
    assert_eq!(
        rows.value()
            .iter()
            .map(|row| row.result().value())
            .collect::<Vec<_>>(),
        [1, 2, 3]
    );
    schema::call(&rows, "completeRank");
    for row in rows.value() {
        let doc: Value = serde_json::from_str(&row.result().to_json().unwrap()).unwrap();
        assert_eq!(doc["schema"], "thinkthen.result/2");
        assert_eq!(doc["value"], row.result().value());
        assert_eq!(doc["threshold"], Value::Null);
        assert_eq!(doc["question"]["verb"], "score");
    }
    assert_ne!(
        rows.value()[1].result().answer_id(),
        rows.value()[2].result().answer_id()
    );
    assert_eq!(
        rows.value()[1].result().identity().observations(),
        rows.value()[2].result().identity().observations()
    );
    assert_eq!(listener.count(), 1);
    assert_eq!(listener.questions(), 2);
    let refused = engine
        .rank_complete_with(
            &Question::decide("Refund?").unwrap().cut(),
            ["text"],
            CallOptions::new(),
        )
        .unwrap_err();
    assert_eq!(refused.kind(), ErrorKind::Usage);
    assert_eq!(listener.count(), 1);
}

#[test]
fn eager_invalid_later_evidence_refuses_the_whole_native_source_before_sending() {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#)
    })
    .unwrap();
    let engine = engine(&listener);
    let error = engine
        .decide_records_complete_with(
            &Question::decide("Refund?").unwrap().cut(),
            [
                RecordInput {
                    original: original(0),
                    context: None,
                    options: None,
                },
                RecordInput {
                    original: Original {
                        id: 1,
                        text: " ",
                        local: Rc::new(()),
                    },
                    context: None,
                    options: None,
                },
            ],
            CallOptions::new(),
        )
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(listener.count(), 0);
}
