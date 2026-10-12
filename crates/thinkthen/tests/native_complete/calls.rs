use super::*;
use thinkthen::{BatchSetting, CancelToken, StopCause};

#[path = "../support/original.rs"]
mod original;
use original::Original;

#[test]
fn complete_envelopes_keep_owned_originals_and_actual_facts_without_changing_legacy_call() {
    let listener = Listener::answering(|body| {
        let body: Value = serde_json::from_slice(body).unwrap();
        let answers = body["questions"]
            .as_object()
            .unwrap()
            .keys()
            .map(|name| (name.clone(), json!({"type":"noul","noul":0.9})))
            .collect::<serde_json::Map<_, _>>();
        Canned::ok(&json!({"model":"fixed","answers":answers}).to_string())
    })
    .unwrap();
    let engine = engine(&listener);
    let question = Question::decide("Good?").unwrap().cut();
    let rows = crate::api::rows(
        &engine,
        thinkthen::RequestFunction::Decide,
        question.clone().into(),
        crate::api::plain_records([Original {
            z: false,
            body: "One.",
            a: vec![2, 1],
        }]),
        CallOptions::new().attempts(true),
    )
    .unwrap();
    let thinkthen::RequestRows::Decisions(batch) = rows else {
        panic!("decision rows");
    };
    let call = batch.into_call().unwrap();
    assert_eq!(call.value()[0].original().a, [2, 1]);
    let complete = call.complete().unwrap();
    let encoded = serde_json::to_string(&complete).unwrap();
    assert!(encoded.contains(r#""input":{"z":false,"body":"One.","a":[2,1]}"#));
    let doc: Value = serde_json::from_str(&encoded).unwrap();
    assert_eq!(doc["value"][0]["schema"], "thinkthen.result/2");
    assert_eq!(doc["facts"]["records"], 1);
    assert_eq!(doc["facts"]["requests_sent"], 1);
    assert_eq!(doc["facts"]["attempts"].as_array().unwrap().len(), 1);
    assert_eq!(
        doc["facts"]["call_id"],
        call.facts().call_id().unwrap().as_str()
    );
    assert!(doc.get("error").is_none());
    let legacy: Value = serde_json::to_value(call.facts()).unwrap();
    assert!(legacy.get("call_id").is_none());
    assert!(legacy.get("attempts").is_none());
    assert!(!format!("{complete:?}").contains("One."));
    assert_eq!(listener.count(), 1);
}

#[test]
fn a_started_record_failure_has_original_stop_position_status_and_joined_complete_facts() {
    let listener = Listener::answering(|body| {
        let body: Value = serde_json::from_slice(body).unwrap();
        if body["questions"]["q1"]["instructions"]
            .as_str()
            .unwrap()
            .contains("One.")
        {
            Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#)
        } else {
            Canned::status(503, "PRIVATE_RESPONSE")
        }
    })
    .unwrap();
    let engine = engine(&listener);
    let question = Question::decide("Good?").unwrap().cut();
    let error = crate::api::whole(
        &engine,
        thinkthen::RequestFunction::Decide,
        question.clone().into(),
        crate::api::plain_records(["One.", "Two."]),
        CallOptions::new()
            .batch(BatchSetting::Records(std::num::NonZeroUsize::MIN))
            .attempts(true),
    )
    .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Backend);
    assert_eq!(error.stopped().at(), Some(2));
    assert_eq!(error.stopped().cause(), StopCause::Status);
    assert_eq!(error.stopped().status(), Some(503));
    assert!(error.stopped().retryable());
    schema::check(&serde_json::to_value(error.complete()).unwrap(), "");
    let complete = error.complete();
    let doc = serde_json::to_value(&complete).unwrap();
    assert_eq!(
        doc["error"]["stopped"],
        json!({"at":2,"cause":"status","status":503,"retryable":true})
    );
    assert_eq!(doc["facts"]["records"], 1);
    assert_eq!(doc["facts"]["requests_sent"], 2);
    assert_eq!(doc["facts"]["attempts"].as_array().unwrap().len(), 2);
    assert_eq!(
        doc["facts"]["call_id"],
        error.facts().unwrap().call_id().unwrap().as_str()
    );
    assert!(doc.get("value").is_none());
    assert!(doc.get("answer_id").is_none());
    assert!(
        !serde_json::to_string(&complete)
            .unwrap()
            .contains("PRIVATE_RESPONSE")
    );
    assert_eq!(listener.count(), 2);
}

#[test]
fn admission_and_cancellation_keep_distinct_complete_failure_facts_and_zero_sends() {
    let listener = Listener::answering(|_| Canned::ok("{}")).unwrap();
    let engine = engine(&listener);
    let question = Question::decide("Good?").unwrap().cut();
    let invalid = crate::api::whole(
        &engine,
        thinkthen::RequestFunction::Decide,
        question.clone().into(),
        crate::api::plain_records(["valid", ""]),
        CallOptions::new().attempts(true),
    )
    .unwrap_err();
    let refused = serde_json::to_value(invalid.complete()).unwrap();
    assert_eq!(refused["error"]["kind"], "usage");
    assert_eq!(refused["error"]["stopped"]["cause"], "usage");
    schema::check(&refused, "");
    assert!(refused.get("facts").is_none());
    let token = CancelToken::new();
    token.cancel();
    let cancelled = crate::api::decide(
        &engine,
        &question,
        "valid",
        CallOptions::new().cancel(&token).attempts(true),
    )
    .unwrap_err();
    assert_eq!(cancelled.kind(), ErrorKind::Cancelled);
    assert_eq!(cancelled.stopped().cause(), StopCause::Cancelled);
    assert_eq!(cancelled.stopped().at(), None);
    let cancelled = serde_json::to_value(cancelled.complete()).unwrap();
    assert!(cancelled.get("facts").is_none());
    let expired = crate::api::decide(
        &engine,
        &question,
        "valid",
        CallOptions::new().deadline_ms(0).unwrap().attempts(true),
    )
    .unwrap_err();
    assert_eq!(expired.kind(), ErrorKind::Deadline);
    assert_eq!(expired.stopped().cause(), StopCause::Deadline);
    assert_eq!(expired.stopped().at(), None);
    let stopped = serde_json::to_value(expired.complete()).unwrap();
    assert_eq!(stopped["facts"]["requests_sent"], 0);
    assert!(stopped["facts"]["call_id"].is_string());
    assert_eq!(stopped["facts"]["attempts"], json!([]));
    assert!(stopped.get("value").is_none());
    assert_eq!(listener.count(), 0);
}
