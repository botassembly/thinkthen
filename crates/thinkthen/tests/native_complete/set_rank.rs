use super::*;
#[path = "../../src/test_deadline/child.rs"]
mod child;
use thinkthen::{
    BatchSetting, InputEvidence, QuestionInput, RankSet, RecordInput, RecordObservation,
};
const SET: &str =
    r#"{"version":1,"questions":{"first":{"decide":"First?"},"second":{"decide":"Second?"}}}"#;
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
fn complete_set_rank_duplicate_occurrences_keep_observations_and_replay_ids_across_batch_sizes() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.7},"q2":{"type":"noul","noul":0.9},"q3":{"type":"noul","noul":0.4},"q4":{"type":"noul","noul":0.8}}}"#)).unwrap();
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
    let set = RankSet::from_json(SET).unwrap();
    let call = engine
        .rank_set_complete_with(&set, ["a", "a", "b"], CallOptions::new())
        .unwrap();
    assert_eq!(listener.count(), 1);
    let rows = call.value();
    assert_eq!(
        rows.iter().map(|row| row.ordinal()).collect::<Vec<_>>(),
        vec![0, 1, 2]
    );
    assert_eq!(
        rows[0].result().result().identity().observations(),
        rows[1].result().result().identity().observations()
    );
    assert_ne!(
        rows[0].result().result().answer_id(),
        rows[1].result().result().answer_id()
    );
    assert_ne!(
        rows[0].result().members()[0].result().answer_id(),
        rows[0].result().members()[1].result().answer_id()
    );
    assert!(
        rows[0]
            .result()
            .result()
            .identity()
            .question_sources()
            .iter()
            .all(|source| source.batch_size() == Some(4))
    );
    let replay = build().replay(&folder).unwrap().build().unwrap();
    let held = replay
        .rank_set_complete_with(
            &set,
            ["a", "a", "b"],
            CallOptions::new().batch(BatchSetting::Records(
                std::num::NonZeroUsize::new(1).unwrap(),
            )),
        )
        .unwrap();
    assert_ne!(call.facts().call_id(), held.facts().call_id());
    for (one, other) in rows.iter().zip(held.value()) {
        assert_eq!(
            one.result().result().answer_id(),
            other.result().result().answer_id()
        );
        for (a, b) in one.result().members().iter().zip(other.result().members()) {
            assert_eq!(a.name(), b.name());
            assert_eq!(a.result().answer_id(), b.result().answer_id());
            assert_eq!(a.result().value(), b.result().value());
        }
        assert_eq!(
            other.result().result().identity().origin(),
            Some(Origin::Replay)
        );
        assert!(
            other
                .result()
                .result()
                .identity()
                .question_sources()
                .iter()
                .all(|source| source.batch_size() == Some(4))
        );
    }
    assert_eq!(held.facts().requests_sent(), 0);
    assert_eq!(listener.count(), 1);
    drop(replay);
    drop(engine);
    std::fs::remove_dir_all(folder).unwrap();
}
#[test]
fn complete_set_rank_admits_every_member_and_empty_input_without_inventing_observations() {
    let listener = Listener::answering(|_| Canned::ok("{}")).unwrap();
    let engine = engine(&listener);
    let set = RankSet::from_json(r#"{"version":1,"questions":{"first":{"decide":"First?","context_schema":{"type":"string"}},"second":{"decide":"Second?","name":"authored_second","item_schema":{"type":"object","properties":{}}}}}"#).unwrap();
    let error = engine
        .rank_set_complete_with(&set, ["a", "b"], CallOptions::new())
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(
        error.detail().message(),
        "the item does not match item_schema"
    );
    assert!(error.facts().is_none());
    let reader_error = engine
        .try_rank_set_records_complete_with(
            &set,
            [
                Ok(RecordInput {
                    original: "a",
                    context: None,
                    options: None,
                }),
                Err(thinkthen::Error::new(
                    ErrorKind::Local,
                    "the input could not be read",
                )),
            ],
            CallOptions::new(),
        )
        .unwrap_err();
    assert_eq!(reader_error.kind(), ErrorKind::Local);
    assert!(reader_error.facts().is_none());
    let empty = engine
        .rank_set_complete_with(&set, Vec::<&str>::new(), CallOptions::new())
        .unwrap();
    assert!(empty.value().is_empty());
    assert_eq!(empty.facts().records(), 0);
    assert_eq!(empty.facts().requests_sent(), 0);
    assert_eq!(empty.facts().model(), None);
    assert_eq!(listener.count(), 0);
}
#[test]
fn complete_set_rank_started_member_failure_retains_actual_observer_identity_and_call_facts() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","error":"private-backend-error"},"q2":{"type":"noul","noul":0.9}},"usage":{"input_tokens":887}}"#)).unwrap();
    let engine = engine(&listener);
    let set = RankSet::from_json(SET).unwrap();
    let observed = std::sync::Mutex::new(Vec::new());
    let observer = |event: RecordObservation<'_>| observed.lock().unwrap().push(event.to_owned());
    let error = engine
        .rank_set_complete_with(
            &set,
            ["private-input"],
            CallOptions::new().attempts(true).observe(&observer),
        )
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Backend);
    assert_eq!(error.facts().unwrap().requests_sent(), 1);
    assert_eq!(error.facts().unwrap().input_tokens(), Some(887));
    assert_eq!(error.facts().unwrap().output_tokens(), None);
    assert_eq!(error.facts().unwrap().attempts().unwrap().len(), 1);
    assert!(!format!("{error:?}").contains("private"));
    assert_eq!(observed.lock().unwrap().len(), 2);
    for event in observed.lock().unwrap().iter() {
        let thinkthen::OwnedRecordObservation::Question {
            index,
            member,
            detail,
            ..
        } = event
        else {
            panic!("question")
        };
        assert_eq!(*index, 0);
        assert!(matches!(member.as_deref(), Some("first" | "second")));
        assert!((detail.detail().answer_id().is_some() || detail.detail().failure_id().is_some()));
        assert_eq!(detail.detail().question_sources()[0].batch_size(), Some(2));
    }
    assert_eq!(listener.count(), 1);
}

#[test]
fn complete_set_rank_preserves_selected_objects_typed_context_and_actual_source_coordinates() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.7},"q2":{"type":"noul","noul":0.4}}}"#)).unwrap();
    let engine = engine(&listener);
    let set = RankSet::from_json(r#"{"version":1,"questions":{"first":{"decide":"First?","name":"authored_first","wording_version":4,"item_schema":{"type":"object","properties":{"body":{"type":"string"}},"required":["body"]},"context_schema":{"type":"object","properties":{"ready":{"type":"boolean"}},"required":["ready"]}},"second":{"decide":"Second?","name":"authored_second","item_schema":{"type":"object","properties":{}},"context_schema":{"type":"object","properties":{}}}}}"#).unwrap();
    let reading = thinkthen::RecordReading::new(&["/item"], Some("/ctx"), None)
        .unwrap()
        .with_context_schema(thinkthen::InputDeclaration::Object(
            thinkthen::ObjectDeclaration::new(Vec::new(), Vec::new()).unwrap(),
        ));
    let record = reading
        .compose_source(thinkthen::SourceItem::Text(thinkthen::SourceRecord {
            record: r#"{"private":null,"item":{"body":"a","extra":false},"ctx":{"ready":false}}"#
                .to_owned(),
            file: "private-source.jsonl".into(),
            first_line: 4,
            last_line: 4,
        }))
        .unwrap();
    let second = reading
        .compose_source(thinkthen::SourceItem::Text(thinkthen::SourceRecord {
            record: r#"{"private":null,"item":{"body":"b","extra":false},"ctx":{"ready":true}}"#
                .to_owned(),
            file: "other-source.jsonl".into(),
            first_line: 8,
            last_line: 8,
        }))
        .unwrap();
    let events = std::sync::Mutex::new(Vec::new());
    let observer = |event: RecordObservation<'_>| events.lock().unwrap().push(event.to_owned());
    let call = engine
        .rank_set_records_complete_with(
            &set,
            [record, second],
            CallOptions::new()
                .context("Unused shared text.")
                .observe(&observer),
        )
        .unwrap();
    assert_eq!(call.value()[0].result().value(), 1);
    assert_eq!(call.value()[0].result().members().len(), 2);
    let document = serde_json::to_value(call.complete().unwrap()).unwrap();
    assert_located_documents(&document);
    assert_rank_schema(&document);

    for event in events.lock().unwrap().iter() {
        let thinkthen::OwnedRecordObservation::Question { index, detail, .. } = event else {
            continue;
        };
        let detail = detail.detail();
        let QuestionInput::Record(original) = detail.input().unwrap() else {
            panic!("record")
        };
        assert_eq!(
            original.location().unwrap().file(),
            if *index == 0 {
                "private-source.jsonl"
            } else {
                "other-source.jsonl"
            }
        );
        assert_eq!(
            original.location().unwrap().first_line(),
            Some(if *index == 0 { 4 } else { 8 })
        );
        assert_eq!(
            original.selected().to_json().unwrap(),
            if *index == 0 {
                r#"{"body":"a","extra":false}"#
            } else {
                r#"{"body":"b","extra":false}"#
            }
        );
    }
    assert_eq!(listener.count(), 2);
    let mut bodies = listener
        .requests()
        .iter()
        .map(|request| String::from_utf8(request.body.clone()).unwrap())
        .collect::<Vec<_>>();
    bodies.sort();
    assert_eq!(bodies, vec![
        r#"{"state":{"ready":false},"model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is {\"body\":\"a\",\"extra\":false}. First?"},"q2":{"type":"noul","instructions":"The text is {\"body\":\"a\",\"extra\":false}. Second?"}}}"#.to_owned(),
        r#"{"state":{"ready":true},"model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is {\"body\":\"b\",\"extra\":false}. First?"},"q2":{"type":"noul","instructions":"The text is {\"body\":\"b\",\"extra\":false}. Second?"}}}"#.to_owned(),
    ]);
}

#[cfg(test)]
fn assert_located_documents(document: &Value) {
    let row = &document["value"][0];
    assert_eq!(row["index"], 0);
    assert_eq!(
        row["source"],
        json!({"file":"private-source.jsonl","first_line":4,"last_line":4})
    );
    assert_eq!(
        row["input"],
        json!({"private":null,"item":{"body":"a","extra":false},"ctx":{"ready":false}})
    );
    for (member, authored) in row["members"]
        .as_array()
        .unwrap()
        .iter()
        .zip(["authored_first", "authored_second"])
    {
        assert_eq!(member["result"]["source"], row["source"]);
        assert_eq!(member["result"]["question"]["name"], authored);
        assert_eq!(
            member["result"]["meta"]["context_sha256"],
            row["meta"]["context_sha256"]
        );
        assert!(member["result"].get("input").is_none());
        assert!(member["result"].get("index").is_none());
    }
    assert_eq!(
        row["members"][0]["result"]["question"]["wording_version"],
        4
    );

    let other = &document["value"][1];
    assert_eq!(other["index"], 1);
    assert_eq!(other["input"]["item"]["body"], "b");
    assert_eq!(
        other["source"],
        json!({"file":"other-source.jsonl","first_line":8,"last_line":8})
    );
    assert_ne!(
        other["meta"]["context_sha256"],
        row["meta"]["context_sha256"]
    );
    for member in other["members"].as_array().unwrap() {
        assert_eq!(member["result"]["source"], other["source"]);
        assert_eq!(
            member["result"]["meta"]["context_sha256"],
            other["meta"]["context_sha256"]
        );
    }
}

#[cfg(test)]
fn assert_rank_schema(document: &Value) {
    schema::check(document, "completeRank");
    // Mutate actual native output at the reader boundary, never fabricate a result.
    let script = r#"
import copy, json, sys
from jsonschema import Draft202012Validator
case = json.load(sys.stdin)
validator = Draft202012Validator(case['schema'])
document = case['document']
for change in ['negative', 'unknown', 'nested', 'missing', 'null', 'empty', 'score']:
    bad = copy.deepcopy(document)
    row = bad['value'][0]
    member = row['members'][0]
    if change == 'negative': member['result']['value'] = -1
    elif change == 'unknown': member['extra'] = True
    elif change == 'nested': member['result']['members'] = row['members']
    elif change == 'missing': del member['result']
    elif change == 'null': row['members'] = None
    elif change == 'empty': row['members'] = []
    elif change == 'score': row['question']['verb'] = 'score'
    assert not validator.is_valid(bad), change
historical = copy.deepcopy(document)
del historical['value'][0]['members']
validator.validate(historical)
"#;
    use std::io::Write as _;
    let mut child = child::command("python3", &[])
        .args(["-c", script])
        .stdin(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(json!({"schema":serde_json::from_str::<Value>(thinkthen::complete_call_schema()).unwrap(),"document":document}).to_string().as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
