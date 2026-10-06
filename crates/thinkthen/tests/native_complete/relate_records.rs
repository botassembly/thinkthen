use super::*;
use std::sync::Mutex;
use thinkthen::{
    InputFileReader, InputReaderOptions, RawRecord, RecordInput, RecordObservation, RecordReading,
    Relate,
};

#[test]
fn located_relations_project_saved_fields_once_and_replay_actual_original_set() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"actual-model","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.1}},"usage":{"input_tokens":887}}"#)).unwrap();
    let folder = folder();
    let path = folder.join("relate.json");
    std::fs::write(&path, r#"{"version":1,"relate":{"fields":{"name":"/person/name","kind":"/person/kind"},"relations":[{"name":"follows","source":"person","target":"person","reads":"follows"}]},"model":"saved-model"}"#).unwrap();
    let ask = Relate::load_records(&path).unwrap();
    assert_eq!(Relate::load(&path).unwrap_err().kind(), ErrorKind::Local);
    let original = b"{\"private\":false,\"person\":{\"name\":\"Ada\",\"kind\":\"person\"}}\n{\"private\":null,\"person\":{\"name\":\"Grace\",\"kind\":\"person\"}}\n";
    let records = || {
        let reading = RecordReading::new(&[""], None, None).unwrap();
        InputFileReader::new(
            "people.jsonl",
            std::io::Cursor::new(original),
            InputReaderOptions::default(),
        )
        .unwrap()
        .map(move |source| source.and_then(|source| reading.compose_source(source)))
    };
    let build = || {
        Engine::builder()
            .base_url(listener.base())
            .unwrap()
            .model("engine-model")
            .unwrap()
            .api_key("complete-private")
            .unwrap()
            .max_retries(0)
    };
    let engine = build().cache_at(&folder).unwrap().build().unwrap();
    let held = Mutex::new(Vec::new());
    let observer = |event: RecordObservation<'_>| held.lock().unwrap().push(event.to_owned());
    let call = engine
        .try_relate_records_complete_with(
            &ask,
            records(),
            CallOptions::new()
                .context("Separate.")
                .attempts(true)
                .observe(&observer),
        )
        .unwrap();
    assert_relation_call(&call, &listener);
    let replay = build().replay(&folder).unwrap().build().unwrap();
    let again = replay
        .try_relate_records_complete_with(
            &ask,
            records(),
            CallOptions::new().context("Separate.").attempts(true),
        )
        .unwrap();
    assert_eq!(
        again.value().result().answer_id(),
        call.value().result().answer_id()
    );
    assert_eq!(
        again.value().result().identity().origin(),
        Some(Origin::Replay)
    );
    let thinkthen::QuestionInput::Record(second) = &again.value().original()[1] else {
        panic!("located original")
    };
    assert_eq!(second.location().unwrap().first_line(), Some(2));
    assert_eq!(again.facts().requests_sent(), 0);
    assert_eq!(listener.count(), 1);
    assert_eq!(listener.questions(), 2);
    drop(replay);
    drop(engine);
    assert_owned_originals(&held.lock().unwrap());
    std::fs::remove_dir_all(folder).unwrap();
}
#[cfg(test)]
fn assert_owned_originals(events: &[thinkthen::OwnedRecordObservation]) {
    for event in events {
        let thinkthen::OwnedRecordObservation::Question { detail, .. } = event else {
            continue;
        };
        let detail = detail.detail();
        assert!(detail.answer_id().is_some());
        let inputs = detail.inputs().collect::<Vec<_>>();
        assert_eq!(inputs.len(), 2);
        for (at, input) in inputs.iter().enumerate() {
            let thinkthen::QuestionInput::Record(record) = input else {
                panic!("original")
            };
            assert_eq!(record.location().unwrap().first_line(), Some(at + 1));
            assert_eq!(record.location().unwrap().file(), "people.jsonl");
        }
    }
}

#[test]
fn complete_relation_records_keep_literal_mode_and_refuse_bad_whole_sets_before_sends() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.1}}}"#)).unwrap();
    let engine = engine(&listener);
    let ask = Relate::builder()
        .relation(thinkthen::RelationRule::one_way("follows", "*", "*").unwrap())
        .unwrap()
        .build()
        .unwrap();
    let lines = [
        RecordInput {
            original: r#"{"name":"literal"}"#,
            context: None,
            options: None,
        },
        RecordInput {
            original: "Other.",
            context: None,
            options: None,
        },
    ];
    let call = engine
        .relate_records_complete_with(&ask, lines, CallOptions::new())
        .unwrap();
    assert_eq!(call.value().result().question().fields(), None);
    assert_eq!(call.value().original()[0], r#"{"name":"literal"}"#);
    assert_eq!(
        call.value().result().value()[0].source().name(),
        r#"{"name":"literal"}"#
    );
    let reading = RecordReading::new(&[], None, None).unwrap();
    let record = || {
        reading
            .compose(
                RawRecord::json(r#"{"text":"Recognized.","kind":"*","private":false}"#).unwrap(),
            )
            .unwrap()
    };
    assert_eq!(
        engine
            .relate_records_complete_with(&ask, [record(), record()], CallOptions::new())
            .unwrap_err()
            .kind(),
        ErrorKind::Usage
    );
    let invalid = reading
        .compose(RawRecord::json(r#"{"name":false,"kind":"*"}"#).unwrap())
        .unwrap();
    assert_eq!(
        engine
            .relate_records_complete_with(&ask, [record(), invalid], CallOptions::new())
            .unwrap_err()
            .stopped()
            .at(),
        Some(2)
    );
    let error = engine
        .try_relate_records_complete_with(
            &ask,
            [
                Ok(record()),
                Err(thinkthen::Error::new(ErrorKind::Local, "reader stopped")),
            ],
            CallOptions::new(),
        )
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Local);
    assert_eq!(error.stopped().at(), Some(2));
    assert!(error.facts().is_none());
    let empty = engine
        .relate_records_complete_with(
            &ask,
            Vec::<RecordInput<&str>>::new(),
            CallOptions::new().attempts(true),
        )
        .unwrap();
    assert_eq!(empty.value().original().len(), 0);
    assert_eq!(empty.facts().records(), 0);
    assert_eq!(empty.value().result().identity().origin(), None);
    assert_eq!(empty.value().result().meta().attempts(), Some(&[][..]));
    assert_eq!(listener.count(), 1);
}

#[cfg(test)]
type RelatedRecords = thinkthen::Call<
    thinkthen::CompleteRecord<Vec<thinkthen::QuestionInput>, thinkthen::CompleteRelated>,
>;
#[cfg(test)]
fn assert_relation_call(call: &RelatedRecords, listener: &Listener) {
    assert_eq!(call.value().original().len(), 2);
    assert_eq!(
        call.value().result().question().fields(),
        Some(("/person/name", "/person/kind"))
    );
    assert_eq!(call.value().result().value()[0].source().name(), "Ada");
    assert_eq!(call.value().result().value()[0].target().name(), "Grace");
    assert_eq!(
        call.value()
            .result()
            .members()
            .map(|member| member.accepted())
            .collect::<Vec<_>>(),
        [Some(true), Some(false)]
    );
    assert_eq!(
        call.value().result().identity().answered_by(),
        Some("actual-model")
    );
    assert_eq!(call.facts().input_tokens(), Some(887));
    assert_eq!(call.facts().output_tokens(), None);
    let document = serde_json::to_value(call.value()).unwrap();
    assert_eq!(document["schema"], "thinkthen.result/2");
    assert_eq!(
        document["input"],
        json!([{"private":false,"person":{"name":"Ada","kind":"person"}}, {"private":null,"person":{"name":"Grace","kind":"person"}}])
    );
    assert_eq!(document["meta"]["usage"], json!({"input_tokens":887}));
    let body: Value = serde_json::from_slice(&listener.requests()[0].body).unwrap();
    assert_eq!(body["model"], "saved-model");
    assert_eq!(
        body["state"],
        json!({"context":"Separate.","evidence":{"entities":[{"id":"i1","name":"Ada","kind":"person"},{"id":"i2","name":"Grace","kind":"person"}]}})
    );
    assert_eq!(
        body["questions"],
        json!({"q1":{"type":"noul","instructions":"Is it true that i1 follows i2?"}, "q2":{"type":"noul","instructions":"Is it true that i2 follows i1?"}})
    );
}
