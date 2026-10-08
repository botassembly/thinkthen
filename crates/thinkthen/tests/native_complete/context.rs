use super::*;
use thinkthen::{ObjectContext, RawRecord, RecordContext, RecordInput, RecordReading};

const DECLARED: &str = r#"{"decide":"Refund?","context_schema":{"type":"object","properties":{"ready":{"type":"boolean"},"guide":{"type":"string"}},"required":["ready"]}}"#;
const CONTEXT: &str = r#"{"guide":"Private guide.","ready":false,"extra":[null,12]}"#;

#[cfg(test)]
fn context() -> RecordContext {
    RecordContext::Object(ObjectContext::new(&RawRecord::json(CONTEXT).unwrap()).unwrap())
}
fn input(context: Option<RecordContext>) -> RecordInput<&'static str> {
    RecordInput {
        examples: None,
        original: "Refund me.",
        context,
        options: None,
    }
}

#[test]
fn object_context_is_actual_ordered_state_and_replay_preserves_its_observation() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.7}},"usage":{"input_tokens":887}}"#)).unwrap();
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
    let thinkthen::LoadedQuestion::Question(question) = Question::from_json(DECLARED).unwrap()
    else {
        unreachable!()
    };
    let live = engine
        .decide_records_complete_with(
            &question,
            [input(Some(context()))],
            CallOptions::new().context("Unsupplied fallback."),
        )
        .unwrap();
    assert_eq!(listener.count(), 1);
    assert_eq!(
        String::from_utf8(listener.requests()[0].body.clone()).unwrap(),
        r#"{"state":{"guide":"Private guide.","ready":false,"extra":[null,12]},"model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"Refund me.\". Refund?"}}}"#
    );
    let replay = build().replay(&folder).unwrap().build().unwrap();
    let held = replay
        .decide_records_complete_with(&question, [input(Some(context()))], CallOptions::new())
        .unwrap();
    assert_eq!(
        live.value()[0].result().answer_id(),
        held.value()[0].result().answer_id()
    );
    assert_eq!(
        held.value()[0].result().identity().origin(),
        Some(Origin::Replay)
    );
    assert_eq!(
        held.value()[0]
            .result()
            .reported_usage()
            .unwrap()
            .input_tokens(),
        Some(887)
    );
    assert_eq!(listener.count(), 1);
    assert!(!format!("{:?}", context()).contains("Private guide"));
    let RecordContext::Object(object) = context() else {
        unreachable!()
    };
    assert_eq!(object.content().to_json().unwrap(), CONTEXT);
    assert!(!format!("{object:?}").contains("Private guide"));
    drop(replay);
    drop(engine);
    std::fs::remove_dir_all(folder).unwrap();
}

#[test]
fn finite_context_admission_refuses_the_whole_call_without_coercion_or_cache_lookup() {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.7}}}"#)
    })
    .unwrap();
    let engine = engine(&listener);
    let thinkthen::LoadedQuestion::Question(question) = Question::from_json(DECLARED).unwrap()
    else {
        unreachable!()
    };
    for invalid in [
        RecordContext::Text(CONTEXT.into()),
        RecordContext::Text(String::new()),
        RecordContext::Object(
            ObjectContext::new(&RawRecord::json(r#"{"ready":null}"#).unwrap()).unwrap(),
        ),
    ] {
        let error = engine
            .decide_records_complete_with(
                &question,
                [input(Some(context())), input(Some(invalid))],
                CallOptions::new(),
            )
            .unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert_eq!(
            error.detail().message(),
            "the per-item context does not match context_schema"
        );
        assert_eq!(error.stopped().at(), Some(2));
        assert!(error.facts().is_none());
        assert!(!format!("{error:?}").contains("Private guide"));
    }
    for original in [
        RawRecord::text(CONTEXT).unwrap(),
        RawRecord::json("null").unwrap(),
        RawRecord::json("[]").unwrap(),
    ] {
        assert!(ObjectContext::new(&original).is_err());
    }
    let undeclared = Question::decide("Refund?").unwrap().cut();
    assert!(
        engine
            .decide_records_complete_with(&undeclared, [input(Some(context()))], CallOptions::new())
            .is_err()
    );
    assert_eq!(listener.count(), 0);
    engine
        .decide_records_complete_with(
            &question,
            [input(None)],
            CallOptions::new().context("Plain shared guide."),
        )
        .unwrap();
    assert_eq!(listener.count(), 1);
    assert_eq!(
        serde_json::from_slice::<Value>(&listener.requests()[0].body).unwrap()["state"],
        "Plain shared guide."
    );
}

#[test]
fn native_context_projection_keeps_originals_and_annotation_wraps_the_typed_state_separately() {
    let thinkthen::LoadedQuestion::Question(question) = Question::from_json(DECLARED).unwrap()
    else {
        unreachable!()
    };
    let reading = RecordReading::new(&["/message"], Some("/context"), None)
        .unwrap()
        .with_context_schema(question.context_schema().unwrap().clone());
    let original = r#"{"message":"Refund me.","context":{"guide":"Private guide.","ready":false,"extra":[null,12]},"private":"unsent"}"#;
    let record = reading.compose(RawRecord::json(original).unwrap()).unwrap();
    assert_eq!(
        record
            .original
            .original()
            .content()
            .unwrap()
            .to_json()
            .unwrap(),
        original
    );
    assert_eq!(
        serde_json::to_string(record.context.as_ref().unwrap()).unwrap(),
        CONTEXT
    );
    for invalid in [
        r#"{"message":"Refund me.","context":null}"#,
        r#"{"message":"Refund me.","context":"{}"}"#,
    ] {
        let error = reading
            .compose(RawRecord::json(invalid).unwrap())
            .unwrap_err();
        assert_eq!(
            error.detail().message(),
            "the per-item context does not match context_schema"
        );
    }
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.7}}}"#)
    })
    .unwrap();
    let engine = engine(&listener);
    let set = thinkthen::QuestionSet::builder()
        .question("refund", question)
        .unwrap()
        .build()
        .unwrap();
    let rows = engine
        .annotate_records_complete_with(&set, [record], CallOptions::new())
        .unwrap();
    assert_eq!(
        rows.value()[0]
            .original()
            .original()
            .content()
            .unwrap()
            .to_json()
            .unwrap(),
        original
    );
    assert_eq!(listener.count(), 1);
    assert_eq!(
        String::from_utf8(listener.requests()[0].body.clone()).unwrap(),
        r#"{"state":{"context":{"guide":"Private guide.","ready":false,"extra":[null,12]},"evidence":"Each question quotes the text it asks about."},"model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"Refund me.\". Refund?"}}}"#
    );
}
