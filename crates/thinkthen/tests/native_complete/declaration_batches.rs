use super::*;
use std::cell::Cell;
use std::num::NonZeroUsize;
use thinkthen::{BatchSetting, RawRecord, RecordInput, RecordReading};

#[cfg(test)]
fn input(text: &str) -> RecordInput<thinkthen::RecordEvidence> {
    RecordReading::new(&[""], None, None)
        .unwrap()
        .compose(RawRecord::json(text).unwrap())
        .unwrap()
}
#[cfg(test)]
fn two() -> CallOptions<'static> {
    CallOptions::new()
        .batch(BatchSetting::Records(NonZeroUsize::new(2).unwrap()))
        .attempts(true)
}

#[test]
fn invalid_second_streamed_item_prevents_the_entire_staged_batch_from_sending() {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.7}}}"#)
    })
    .unwrap();
    let engine = engine(&listener);
    let question =
        Question::from_json(r#"{"decide":"Refund?","item_schema":{"type":"string"}}"#).unwrap();
    let pulled = Cell::new(0);
    let records = [r#""Valid.""#, "12", r#""Unread.""#]
        .into_iter()
        .inspect(|_| pulled.set(pulled.get() + 1))
        .map(|text| Ok(input(text)));
    let mut batch = engine.try_decide_records_complete_with(&question, records, two());
    let error = batch.next().unwrap().unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(
        error.detail().message(),
        "the item does not match item_schema"
    );
    assert_eq!(error.stopped().at(), Some(2));
    assert_eq!(error.facts().unwrap().records(), 0);
    assert_eq!(error.facts().unwrap().requests_sent(), 0);
    assert_eq!(pulled.get(), 2);
    assert!(batch.next().is_none());
    assert_eq!(listener.count(), 0);
}

#[test]
fn invalid_later_batch_keeps_only_the_dispatched_prefix_and_original_error_position() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.7},"q2":{"type":"noul","noul":0.2}},"usage":{"input_tokens":887}}"#)).unwrap();
    let engine = engine(&listener);
    let question =
        Question::from_json(r#"{"decide":"Refund?","item_schema":{"type":"string"}}"#).unwrap();
    let pulled = Cell::new(0);
    let records = [r#""A.""#, r#""B.""#, r#""C.""#, "false", r#""Unread.""#]
        .into_iter()
        .inspect(|_| pulled.set(pulled.get() + 1))
        .map(|text| Ok(input(text)));
    let mut batch = engine.try_decide_records_complete_with(&question, records, two());
    for (at, value) in [Answer::Yes, Answer::No].into_iter().enumerate() {
        let row = batch.next().unwrap().unwrap();
        assert_eq!(row.ordinal(), at);
        assert_eq!(row.result().value(), value);
        assert_eq!(
            row.result().identity().question_sources()[0]
                .batch_size()
                .unwrap(),
            2
        );
    }
    let error = batch.next().unwrap().unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(error.stopped().at(), Some(4));
    assert_eq!(error.facts().unwrap().records(), 2);
    assert_eq!(error.facts().unwrap().requests_sent(), 1);
    assert_eq!(error.facts().unwrap().input_tokens(), Some(887));
    assert_eq!(error.facts().unwrap().output_tokens(), None);
    assert_eq!(pulled.get(), 4);
    assert!(batch.next().is_none());
    assert_eq!(listener.count(), 1);
    assert_eq!(
        serde_json::from_slice::<Value>(&listener.requests()[0].body).unwrap(),
        json!({"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"A.\". Refund?"},"q2":{"type":"noul","instructions":"The text is \"B.\". Refund?"}}})
    );
}

#[test]
fn an_invalid_replay_batch_is_admitted_before_looking_up_its_missing_first_question() {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.7}}}"#)
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
    let question =
        Question::from_json(r#"{"decide":"Refund?","item_schema":{"type":"string"}}"#).unwrap();
    engine
        .decide_complete_with(&question, "Primed.", CallOptions::new())
        .unwrap();
    let replay = build().replay(&folder).unwrap().build().unwrap();
    let mut batch = replay.try_decide_records_complete_with(
        &question,
        [Ok(input(r#""Unstored.""#)), Ok(input("null"))],
        two(),
    );
    let error = batch.next().unwrap().unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(error.stopped().at(), Some(2));
    assert_eq!(error.facts().unwrap().cache_answers(), 0);
    assert_eq!(error.facts().unwrap().requests_sent(), 0);
    assert!(batch.next().is_none());
    assert_eq!(listener.count(), 1);
    drop(batch);
    drop(replay);
    drop(engine);
    std::fs::remove_dir_all(folder).unwrap();
}

#[test]
fn annotation_member_declarations_admit_the_whole_stream_batch_before_any_member_sends() {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.7}}}"#)
    })
    .unwrap();
    let engine = engine(&listener);
    let set = thinkthen::QuestionSet::from_json(r#"{"version":1,"questions":{"first":{"decide":"Refund?","item_schema":{"type":"string"}},"second":{"decide":"Urgent?","item_schema":{"type":"string"}}}}"#).unwrap();
    let mut batch = engine.try_annotate_records_complete_with(
        &set,
        [Ok(input(r#""Valid.""#)), Ok(input("null"))],
        two(),
    );
    let error = batch.next().unwrap().unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(error.stopped().at(), Some(2));
    assert_eq!(error.facts().unwrap().records(), 0);
    assert_eq!(listener.count(), 0);
    assert!(batch.next().is_none());
}
