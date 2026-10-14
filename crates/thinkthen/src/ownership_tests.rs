//! Stays: the private rank_set_complete_with, execute_relate_records_complete_with, try_find_complete_with, decide_input_complete_with and try_choose_dynamic_records_complete_with primitives preserve opaque caller ownership that public carriers cannot show.
//! Complete ownership contracts that require opaque caller originals.
use crate::{
    CallOptions, Engine, ErrorKind, InputEvidence, Question, QuestionInput, RankSet, RawRecord,
    RecordEvidence, RecordInput, RecordObservation, RecordReading,
};
use conformance_backend::{Canned, Listener};
use serde_json::{Value, json};
#[path = "../tests/support/source_relations.rs"]
mod fixture;
use fixture::{ask, item};
pub(crate) fn engine(listener: &Listener) -> Engine {
    crate::api::loopback(listener.base(), "complete-private")
}
#[path = "../tests/support/set_rank.rs"]
mod set_fixture;
use set_fixture::SET;
const SIX: &str = r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.7},"q2":{"type":"noul","noul":1.0},"q3":{"type":"noul","noul":0.6},"q4":{"type":"noul","noul":0.98},"q5":{"type":"noul","noul":0.5},"q6":{"type":"noul","noul":0.99}},"usage":{"input_tokens":12}}"#;
#[cfg(test)]
fn assert_actual_metadata(result: &thinkthen::CompleteSetRank) {
    let identity = result.result().identity();
    assert_eq!(identity.observations().len(), 2);
    assert!(
        identity
            .question_sources()
            .iter()
            .all(|source| source.batch_size() == Some(6))
    );
    assert_eq!(
        result.result().reported_usage().unwrap().input_tokens(),
        Some(4)
    );
    assert_eq!(
        result.result().reported_usage().unwrap().output_tokens(),
        None
    );
    assert_eq!(result.result().meta().attempts().unwrap().len(), 1);
    assert!(!format!("{result:?}").contains(result.question_name()));
}
#[cfg(test)]
fn assert_serialized_members(result: &thinkthen::CompleteSetRank, ordinal: usize) {
    let document: Value = serde_json::from_str(&result.to_json().unwrap()).unwrap();
    assert!(document.get("input").is_none());
    assert!(document.get("index").is_none());
    assert_eq!(document["members"].as_array().unwrap().len(), 2);
    for ((member, child), probability) in result
        .members()
        .iter()
        .zip(document["members"].as_array().unwrap())
        .zip([[0.7, 1.0], [0.6, 0.98], [0.5, 0.99]][ordinal])
    {
        assert_eq!(child["name"], member.name());
        assert_eq!(
            child["result"],
            serde_json::to_value(member.result()).unwrap()
        );
        assert_eq!(child["result"]["value"], member.result().value());
        assert_eq!(child["result"]["answer"]["probability"], probability);
        assert_eq!(
            child["result"]["answer_id"],
            member.result().answer_id().as_str()
        );
        assert_eq!(child["result"]["meta"]["usage"], json!({"input_tokens":2}));
        assert_eq!(
            child["result"]["meta"]["question_sources"][0]["batch_size"],
            6
        );
        assert_eq!(child["result"]["meta"]["model"], "fixed");
        assert_eq!(
            child["result"]["meta"]["context_sha256"],
            document["meta"]["context_sha256"]
        );
    }
}
#[test]
fn complete_set_rank_keeps_all_members_originals_partial_usage_and_independent_turns_order() {
    // An original has no Clone, Serialize or Send requirement.
    struct Original {
        id: usize,
        text: &'static str,
        local: std::rc::Rc<()>,
    }
    impl InputEvidence for Original {
        fn question_input(&self) -> QuestionInput {
            QuestionInput::Text(self.text.to_owned())
        }
    }
    let listener = Listener::answering(|_| Canned::ok(SIX)).unwrap();
    let engine = engine(&listener);
    let set = RankSet::from_json(SET).unwrap();
    let observed = std::sync::Mutex::new(Vec::new());
    let observer = |event: RecordObservation<'_>| observed.lock().unwrap().push(event.to_owned());
    let call = engine
        .rank_set_complete_with(
            &set,
            ["a", "b", "c"]
                .into_iter()
                .enumerate()
                .map(|(id, text)| Original {
                    id,
                    text,
                    local: std::rc::Rc::new(()),
                }),
            CallOptions::new().attempts(true).observe(&observer),
        )
        .unwrap();
    assert_eq!(call.facts().records(), 3);
    assert_eq!(call.facts().requests_sent(), 1);
    assert_eq!(call.facts().input_tokens(), Some(12));
    assert_eq!(call.facts().output_tokens(), None);
    for (row, (ordinal, winner, first, second)) in
        call.value()
            .iter()
            .zip([(0, "first", 1, 1), (1, "first", 2, 3), (2, "second", 3, 2)])
    {
        assert_eq!(row.ordinal(), ordinal);
        assert_eq!(row.original().id, ordinal);
        assert_eq!(std::rc::Rc::strong_count(&row.original().local), 1);
        let result = row.result();
        assert_eq!(result.question_name(), winner);
        assert_eq!(
            result
                .members()
                .iter()
                .map(|member| (member.name(), member.result().value()))
                .collect::<Vec<_>>(),
            vec![("first", first), ("second", second)]
        );
        assert_actual_metadata(result);
        assert_serialized_members(result, ordinal);
    }
    assert_eq!(
        call.value()
            .iter()
            .map(|row| row.result().value())
            .collect::<Vec<_>>(),
        vec![1, 2, 3]
    );
    assert_eq!(observed.lock().unwrap().len(), 9);
    assert_eq!(listener.count(), 1);
    assert_eq!(
        serde_json::from_slice::<Value>(&listener.requests()[0].body).unwrap(),
        json!({"state":"Each question quotes the text it asks about.","model":"fixed","questions":{
            "q1":{"type":"noul","instructions":"The text is \"a\". First?"},"q2":{"type":"noul","instructions":"The text is \"a\". Second?"},
            "q3":{"type":"noul","instructions":"The text is \"b\". First?"},"q4":{"type":"noul","instructions":"The text is \"b\". Second?"},
            "q5":{"type":"noul","instructions":"The text is \"c\". First?"},"q6":{"type":"noul","instructions":"The text is \"c\". Second?"}
        }})
    );
    let json: Value = serde_json::from_str(&call.value()[2].result().to_json().unwrap()).unwrap();
    assert_eq!(json["schema"], "thinkthen.result/2");
    assert_eq!(json["value"], 3);
    assert_eq!(json["question_name"], "second");
    assert_eq!(json["answer"]["probability"], 0.99);
    assert_eq!(json["meta"]["usage"], json!({"input_tokens":4}));
}
#[test]
fn source_relations_keep_nonserializable_nonclone_originals_and_refuse_mixed_sources_before_sending()
 {
    struct Original {
        value: RecordEvidence,
        marker: std::rc::Rc<()>,
    }
    impl InputEvidence for Original {
        fn question_input(&self) -> thinkthen::QuestionInput {
            self.value.question_input()
        }
    }
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.1}}}"#)).unwrap();
    let engine = engine(&listener);
    let make = |name, at| RecordInput {
        examples: None,
        seed_spans: None,
        original: Original {
            value: item(name, at, "source.txt", "false").original,
            marker: std::rc::Rc::new(()),
        },
        context: None,
        options: None,
    };
    let call = engine
        .execute_relate_records_complete_with(
            &ask(),
            [make("Ada", 1), make("Grace", 2)].into_iter().map(Ok),
            CallOptions::new(),
        )
        .unwrap();
    assert_eq!(
        std::rc::Rc::strong_count(&call.value().original()[0].marker),
        1
    );
    assert_eq!(
        call.value().result().source_edges().unwrap()[0]
            .source()
            .record()
            .content()
            .unwrap()
            .to_json()
            .unwrap(),
        r#"{"name":"Ada","kind":"person","private":false}"#
    );
    let bare = RecordReading::new(&[], None, None)
        .unwrap()
        .compose(RawRecord::json(r#"{"name":"Grace","kind":"person"}"#).unwrap())
        .unwrap();
    let error = engine
        .execute_relate_records_complete_with(
            &ask(),
            [item("Ada", 1, "source.txt", "false"), bare]
                .into_iter()
                .map(Ok),
            CallOptions::new(),
        )
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert!(error.facts().is_none());
    assert_eq!(listener.count(), 1);
}

#[path = "../tests/support/original.rs"]
mod original;
use original::Original;

#[test]
fn complete_find_serialization_retains_the_original_payload_selected_by_native_execution() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"choice","probabilities":{"u001":0.2,"u002":0.8}}}}"#)).unwrap();
    let engine = engine(&listener);
    let question = Question::find("Where?").unwrap();
    let call = engine
        .try_find_complete_with(
            &question,
            [
                Original {
                    z: false,
                    body: "First.",
                    a: vec![1],
                },
                Original {
                    z: true,
                    body: "Second.",
                    a: vec![3, 2],
                },
            ]
            .into_iter()
            .map(Ok),
            CallOptions::new(),
        )
        .unwrap();
    assert_eq!(call.value().selected().unwrap().body, "Second.");
    let doc: Value = serde_json::from_str(&call.value().to_json().unwrap()).unwrap();
    assert_eq!(doc["value"], json!({"z":true,"body":"Second.","a":[3,2]}));
    assert_eq!(
        serde_json::to_value(&call).unwrap()["value"]["value"],
        doc["value"]
    );
    assert_eq!(listener.count(), 1);
}

#[path = "../tests/support/image_fixture.rs"]
mod image_fixture;
use crate::{InputFileReader, InputReaderOptions, ReaderMedia, ReaderOptions, SourceUnit};
use image_fixture::{LIQUID_DECIDE, RED, question};
use std::io::Cursor;

#[test]
fn moved_file_names_do_not_change_existing_identity_and_source_has_no_lines() {
    let listener = Listener::answering(|_| Canned::ok(LIQUID_DECIDE)).unwrap();
    let place = image_fixture::folder();
    let engine = Engine::builder()
        .backend("liquid")
        .unwrap()
        .base_url(listener.base())
        .unwrap()
        .model("d1")
        .unwrap()
        .api_key("fake-key")
        .unwrap()
        .cache_at(&place)
        .unwrap()
        .max_retries(0)
        .build()
        .unwrap();
    let options = InputReaderOptions {
        media: ReaderMedia::Image,
        reading: ReaderOptions {
            unit: SourceUnit::File,
            window: None,
        },
    };
    let source = |name| {
        InputFileReader::new(name, Cursor::new(RED), options)
            .unwrap()
            .next()
            .unwrap()
            .unwrap()
    };
    let old = source("original.png");
    let moved = source("elsewhere/image.dat");
    let first = engine
        .details_input_with(
            &question(),
            &old.question_input(),
            thinkthen::CallOptions::new(),
        )
        .unwrap();
    let second = engine
        .details_input_with(
            &question(),
            &moved.question_input(),
            thinkthen::CallOptions::new(),
        )
        .unwrap();
    assert_eq!(first.value().requests(), second.value().requests());
    let first_complete = engine
        .decide_input_complete_with(
            &question(),
            &old.question_input(),
            thinkthen::CallOptions::new(),
        )
        .unwrap();
    let second_complete = engine
        .decide_input_complete_with(
            &question(),
            &moved.question_input(),
            thinkthen::CallOptions::new(),
        )
        .unwrap();
    assert_eq!(
        first_complete.value().answer_id(),
        second_complete.value().answer_id()
    );
    assert_eq!(first_complete.facts().requests_sent(), 0);
    assert_eq!(second_complete.facts().requests_sent(), 0);
    let first_doc = serde_json::to_value(&first_complete).unwrap();
    assert_eq!(
        first_doc["value"]["source"],
        serde_json::json!({"file":"original.png"})
    );
    let complete = serde_json::to_value(&second_complete).unwrap();
    assert_eq!(
        complete["value"]["source"],
        serde_json::json!({"file":"elsewhere/image.dat"})
    );
    assert_eq!(listener.count(), 1);
    let located = serde_json::to_value(moved).unwrap();
    assert_eq!(located["file"], "elsewhere/image.dat");
    assert!(located.get("first_line").is_none());
    assert!(located.get("last_line").is_none());
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    let body = std::str::from_utf8(&requests[0].body).unwrap();
    assert!(!body.contains("original.png"));
    assert!(!body.contains("elsewhere/image.dat"));
    drop(engine);
    std::fs::remove_dir_all(place).unwrap();
}

use crate::test_deadline::child;
#[path = "../tests/support/complete_schema.rs"]
mod schema;
mod scalar_decision {
    use super::*;
    use crate::{Answer, LoadedQuestion};
    #[test]
    fn complete_decision_schema_admits_authored_content_while_accessors_stay_boolean() {
        let listener = Listener::answering(|_| {
            Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#)
        })
        .unwrap();
        let engine = engine(&listener);
        for meaning in [
            json!({"action":"review","ready":false}),
            json!(["review",false,null,{"priority":2}]),
            Value::Null,
        ] {
            let LoadedQuestion::Question(question) =
                Question::from_json(&json!({"decide":"Refund?","true":meaning}).to_string())
                    .unwrap()
            else {
                panic!("decision")
            };
            let call = engine
                .decide_input_complete_with(
                    &question,
                    &thinkthen::QuestionInput::Text("Evidence".into()),
                    CallOptions::new(),
                )
                .unwrap();
            assert_eq!(call.value().value(), Answer::Yes);
            assert_eq!(
                call.value().question().yes().unwrap().is_null(),
                meaning.is_null()
            );
            let document = serde_json::to_value(&call).unwrap();
            assert_eq!(document["value"]["value"], meaning);
            super::schema::check(&document, "completeDecide");
        }
    }
}

mod dynamic_choose {
    use super::*;
    use crate::CancelToken;
    use crate::api::{dynamic_candidates as candidates, dynamic_one as one};
    use std::cell::Cell;
    #[test]
    fn dynamic_choose_cancellation_stops_before_polling_and_after_a_delivered_row() {
        let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"choice","probabilities":{"b":0.8,"a":0.2}}}}"#)).unwrap();
        let engine = engine(&listener);
        let question = Question::choose_records("Which?").unwrap();
        for pre_cancelled in [true, false] {
            let token = CancelToken::new();
            if pre_cancelled {
                token.cancel();
            }
            let pulled = Cell::new(0);
            let records = (0..2).map(|_| {
                pulled.set(pulled.get() + 1);
                Ok(RecordInput {
                    options: Some(candidates(r#"["b","a"]"#)),

                    ..crate::api::record("Same.")
                })
            });
            let mut batch = engine.try_choose_dynamic_records_complete_with(
                &question,
                records,
                one().cancel(&token),
            );
            if !pre_cancelled {
                assert_eq!(batch.next().unwrap().unwrap().ordinal(), 0);
                assert_eq!(pulled.get(), 1);
                token.cancel();
            }
            let error = batch.next().unwrap().unwrap_err();
            assert_eq!(error.kind(), ErrorKind::Cancelled);
            assert!(batch.next().is_none());
            if !pre_cancelled {
                assert_eq!(error.facts().unwrap().records(), 1);
                assert_eq!(error.facts().unwrap().requests_sent(), 1);
                assert_eq!(
                    error.facts().unwrap().call_id(),
                    batch.facts().unwrap().call_id()
                );
            } else {
                assert_eq!(pulled.get(), 0);
                assert!(error.facts().is_none());
            }
        }
        assert_eq!(listener.count(), 1);
    }
}

mod bulk_details_tests;
