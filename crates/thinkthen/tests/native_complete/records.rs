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
    let listener =
        crate::api::answering(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#);
    let folder = folder();
    let build = || crate::api::fixed(listener.base(), "complete-private").max_retries(0);
    let engine = build().cache_at(&folder).unwrap().build().unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let inputs = || {
        [None, None, Some(""), Some("Other.")]
            .into_iter()
            .enumerate()
            .map(|(at, context)| RecordInput {
                context: context.map(Into::into),

                ..crate::api::record(original(at))
            })
    };
    let call = crate::api::decide_records(
        &engine,
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
    let held = crate::api::decide_records(
        &replay,
        &question,
        inputs(),
        CallOptions::new().context("Fallback."),
    )
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
    let rows = crate::api::choose(
        &engine,
        &fixed,
        [
            RecordInput {
                options: Some(first.clone()),

                ..crate::api::record(original(0))
            },
            RecordInput {
                options: Some(second),

                ..crate::api::record(original(1))
            },
            RecordInput {
                options: Some(first),

                ..crate::api::record(original(2))
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
    let invalid = crate::api::decide_records(
        &engine,
        &Question::decide("Refund?").unwrap().cut(),
        [RecordInput {
            options: Some(RecordOptions::project(r#"{"options":["a","b"]}"#, "/options").unwrap()),
            ..crate::api::record(original(0))
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
    let rows = crate::api::eager_rank(
        &engine,
        &score,
        crate::api::plain_inputs(["first", "second", "first"]),
        CallOptions::new().batch(BatchSetting::Max),
    )
    .unwrap();
    let ranked = crate::api::ranked(&rows).unwrap();
    assert_eq!(
        ranked.iter().map(|row| row.ordinal()).collect::<Vec<_>>(),
        [1, 0, 2]
    );
    assert_eq!(
        ranked
            .iter()
            .map(|row| row.result().value())
            .collect::<Vec<_>>(),
        [1, 2, 3]
    );
    schema::call(&rows, "completeRank");
    for row in ranked {
        let doc: Value = serde_json::from_str(&row.result().to_json().unwrap()).unwrap();
        assert_eq!(doc["schema"], "thinkthen.result/2");
        assert_eq!(doc["value"], row.result().value());
        assert_eq!(doc["threshold"], Value::Null);
        assert_eq!(doc["question"]["verb"], "score");
    }
    assert_ne!(
        ranked[1].result().answer_id(),
        ranked[2].result().answer_id()
    );
    assert_eq!(
        ranked[1].result().identity().observations(),
        ranked[2].result().identity().observations()
    );
    assert_eq!(listener.count(), 1);
    assert_eq!(listener.questions(), 2);
    let refused = engine
        .rank_with(
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
    let listener =
        crate::api::answering(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#);
    let engine = engine(&listener);
    let error = crate::api::eager_decisions(
        &engine,
        &Question::decide("Refund?").unwrap().cut(),
        [
            crate::api::record(original(0)),
            crate::api::record(Original {
                id: 1,
                text: " ",
                local: Rc::new(()),
            }),
        ],
        CallOptions::new(),
    )
    .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(listener.count(), 0);
}

fn seed_records(seeds: &[thinkthen::RecognitionSeedSpan]) -> [RecordInput<&'static str>; 2] {
    [None, Some(seeds.to_vec())].map(|seed_spans| RecordInput {
        seed_spans,

        ..crate::api::record("First.")
    })
}

fn assert_seed_refusal(error: thinkthen::Error, listener: &Listener, door: &str, message: &str) {
    assert_eq!(error.kind(), ErrorKind::Usage, "{door}");
    assert_eq!(error.detail().message(), message, "{door}");
    assert_eq!(error.stopped().at(), Some(2), "{door}");
    assert_eq!(listener.count(), 0, "{door}");
}

fn assert_whole_seed_refusals(
    engine: &Engine,
    listener: &Listener,
    rank_set: &thinkthen::RankSet,
    relate: &thinkthen::Relate,
    find: &Question,
    seeds: &[thinkthen::RecognitionSeedSpan],
) {
    for (function, definition, doors) in [
        (
            thinkthen::RequestFunction::Rank,
            thinkthen::RequestDefinition::from(rank_set.clone()),
            [
                "rank_set_records_complete_with",
                "try_rank_set_records_complete_with",
            ],
        ),
        (
            thinkthen::RequestFunction::Find,
            find.clone().into(),
            [
                "find_records_complete_with",
                "try_find_records_complete_with",
            ],
        ),
        (
            thinkthen::RequestFunction::Relate,
            relate.clone().into(),
            [
                "relate_records_complete_with",
                "try_relate_records_complete_with",
            ],
        ),
    ] {
        for door in doors {
            let outcome = crate::api::whole(
                engine,
                function,
                definition.clone(),
                seed_records(seeds).into_iter().map(Ok),
                CallOptions::new(),
            );
            assert!(outcome.is_err(), "record seed controls must refuse");
            if let Err(error) = outcome {
                assert_seed_refusal(
                    error,
                    listener,
                    door,
                    "item recognition controls apply only to recognize",
                );
            }
        }
    }
}
#[test]
fn eager_nonrecognize_records_refuse_later_seed_controls_before_sending() {
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
    let rank = Question::rank("Best?").unwrap();
    let find = Question::find("Where?").unwrap();
    let definition = r#"{"version":1,"questions":{"a":{"decide":"Fits?"}}}"#;
    let annotate = thinkthen::QuestionSet::from_json(definition).unwrap();
    let rank_set = thinkthen::RankSet::from_json(definition).unwrap();
    let relate = thinkthen::Relate::from_records_json(r#"{"version":1,"relate":{"relations":[{"name":"follows","source":"*","target":"*","reads":"follows"}]}}"#).unwrap();
    for seeds in [
        vec![],
        vec![thinkthen::RecognitionSeedSpan {
            start: 0,
            end: 6,
            kind: None,
        }],
    ] {
        let records = || seed_records(&seeds);
        for (function, definition, door) in [
            (
                thinkthen::RequestFunction::Decide,
                decide.clone().into(),
                "decide_records_complete_with",
            ),
            (
                thinkthen::RequestFunction::Filter,
                decide.clone().into(),
                "filter_records_complete_with",
            ),
            (
                thinkthen::RequestFunction::Choose,
                choose.clone().into(),
                "choose_records_complete_with",
            ),
            (
                thinkthen::RequestFunction::Tag,
                tag.clone().into(),
                "tag_records_complete_with",
            ),
            (
                thinkthen::RequestFunction::Score,
                score.clone().into(),
                "score_records_complete_with",
            ),
            (
                thinkthen::RequestFunction::Annotate,
                annotate.clone().into(),
                "annotate_records_complete_with",
            ),
        ] {
            let error = crate::api::whole(
                &engine,
                function,
                definition,
                records().into_iter().map(Ok),
                CallOptions::new(),
            )
            .unwrap_err();
            assert_seed_refusal(
                error,
                &listener,
                door,
                "item recognition controls apply only to recognize",
            );
        }
        for door in [
            "rank_records_complete_with",
            "try_rank_records_complete_with",
        ] {
            let error =
                crate::api::eager_rank(&engine, &rank, records(), CallOptions::new()).unwrap_err();
            assert_seed_refusal(
                error,
                &listener,
                door,
                "item recognition controls apply only to recognize",
            );
        }
        assert_whole_seed_refusals(&engine, &listener, &rank_set, &relate, &find, &seeds);
    }
}

#[test]
fn eager_atomic_annotation_and_set_rank_plan_later_contexts_before_any_send() {
    let listener = Listener::answering(|_| Canned::ok("unused")).unwrap();
    let engine = crate::api::fixed(listener.base(), "complete-private")
        .no_cache()
        .max_request_bytes(1024)
        .unwrap()
        .build()
        .unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let definition =
        r#"{"version":1,"questions":{"first":{"decide":"Refund?"},"second":{"decide":"Urgent?"}}}"#;
    let annotation = thinkthen::QuestionSet::from_json(definition).unwrap();
    let ranking = thinkthen::RankSet::from_json(definition).unwrap();
    let records = || {
        [None, Some("x".repeat(2048).into())].map(|context| RecordInput {
            context,

            ..crate::api::record("Plain.")
        })
    };
    for error in [
        crate::api::eager_decisions(&engine, &question, records(), CallOptions::new())
            .map(|_| ())
            .unwrap_err(),
        crate::api::whole(
            &engine,
            thinkthen::RequestFunction::Annotate,
            annotation.into(),
            records().into_iter().map(Ok),
            CallOptions::new(),
        )
        .map(|_| ())
        .unwrap_err(),
        crate::api::whole(
            &engine,
            thinkthen::RequestFunction::Rank,
            ranking.into(),
            records().into_iter().map(Ok),
            CallOptions::new(),
        )
        .map(|_| ())
        .unwrap_err(),
    ] {
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert_eq!(
            listener.count(),
            0,
            "a valid earlier plan sent before the later refusal"
        );
    }
}
