use super::*;
use std::sync::Mutex;
use thinkthen::{BatchSetting, OwnedRecordObservation, StopCause};

#[cfg(test)]
fn builder(listener: &Listener) -> thinkthen::EngineBuilder {
    Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .model("fixed")
        .unwrap()
        .api_key("complete-private")
        .unwrap()
        .throttle(1)
        .unwrap()
        .max_retries(0)
}

#[cfg(test)]
fn listener() -> Listener {
    Listener::answering(|body| {
        let body: Value = serde_json::from_slice(body).unwrap();
        let instructions = body["questions"]["q1"]["instructions"].as_str().unwrap();
        if instructions.contains("Fail.") {
            return Canned::status(503, "PRIVATE_RESPONSE");
        }
        let model = if instructions.contains("Two.") {
            "other"
        } else {
            "fixed"
        };
        Canned::ok(&json!({"model":model,"answers":{"q1":{"type":"noul","noul":0.9}}}).to_string())
    })
    .unwrap()
}

#[cfg(test)]
fn engine_for(
    mode: &str,
    listener: &Listener,
    place: &std::path::Path,
    question: &Question,
) -> Engine {
    let builder = || builder(listener);
    match mode {
        "cache" => {
            let engine = builder().cache_at(place).unwrap().build().unwrap();
            let bare = engine.decide(question, "One.").unwrap();
            assert_eq!(*bare.value(), Answer::Yes);
            assert_eq!(bare.facts().model(), Some("fixed"));
            engine
        }
        "replay" => {
            let engine = builder().record(place).unwrap().build().unwrap();
            for (input, model) in [("One.", "fixed"), ("Two.", "other"), ("Three.", "fixed")] {
                let bare = engine.decide(question, input).unwrap();
                assert_eq!(*bare.value(), Answer::Yes);
                assert_eq!(bare.facts().model(), Some(model));
            }
            builder().replay(place).unwrap().build().unwrap()
        }
        _ => builder().no_cache().build().unwrap(),
    }
}

#[test]
fn separate_live_and_stored_replies_omit_mixed_model_facts_and_keep_actual_models() {
    for mode in ["live", "cache", "replay"] {
        let listener = listener();
        let place = folder();
        let question = Question::decide("Good?").unwrap().cut();
        let engine = engine_for(mode, &listener, &place, &question);
        let call = engine
            .decide_many_complete_with(
                &question,
                ["One.", "Two.", "Three."],
                CallOptions::new().batch(BatchSetting::Records(std::num::NonZeroUsize::MIN)),
            )
            .unwrap();
        assert_eq!(call.facts().model(), None, "{mode}");
        let facts = serde_json::to_value(call.facts().complete().unwrap()).unwrap();
        assert!(facts.get("model").is_none(), "{mode}: {facts}");
        assert_eq!(call.facts().records(), 3);
        let (sends, origins) = match mode {
            "cache" => (2, [Origin::Cache, Origin::Live, Origin::Live]),
            "replay" => (0, [Origin::Replay; 3]),
            _ => (3, [Origin::Live; 3]),
        };
        assert_eq!(call.facts().requests_sent(), sends);
        for ((row, actual), origin) in call
            .value()
            .iter()
            .zip(["fixed", "other", "fixed"])
            .zip(origins)
        {
            assert_eq!(row.result().value(), Answer::Yes);
            assert_eq!(
                row.result().identity().question_sources()[0].answered_by(),
                actual
            );
            assert_eq!(
                row.result().identity().question_sources()[0].origin(),
                origin
            );
            assert_eq!(row.result().meta().model(), actual);
        }
        assert_eq!(listener.count(), 3);
        for request in listener.requests() {
            let body: Value = serde_json::from_slice(&request.body).unwrap();
            assert_eq!(body["model"], "fixed");
            assert_eq!(body["questions"].as_object().unwrap().len(), 1);
        }
        drop(engine);
        std::fs::remove_dir_all(place).unwrap();
    }
}

#[test]
fn started_failure_retains_mixed_model_omission_after_a_later_matching_reply() {
    for cached in [false, true] {
        check_started_failure(cached);
    }
}

#[cfg(test)]
fn check_started_failure(cached: bool) {
    let listener = listener();
    let place = folder();
    let engine = if cached {
        builder(&listener).cache_at(&place).unwrap()
    } else {
        builder(&listener).no_cache()
    }
    .build()
    .unwrap();
    let events = Mutex::new(Vec::new());
    let capture =
        |event: thinkthen::RecordObservation<'_>| events.lock().unwrap().push(event.to_owned());
    let question = Question::decide("Good?").unwrap().cut();
    if cached {
        assert_eq!(
            *engine.decide(&question, "One.").unwrap().value(),
            Answer::Yes
        );
    }
    let error = engine
        .decide_many_complete_with(
            &question,
            ["One.", "Two.", "Three.", "Fail."],
            CallOptions::new()
                .batch(BatchSetting::Records(std::num::NonZeroUsize::MIN))
                .attempts(true)
                .observe(&capture),
        )
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Backend);
    assert_eq!(error.stopped().at(), Some(4));
    assert_eq!(error.stopped().cause(), StopCause::Status);
    assert_eq!(error.stopped().status(), Some(503));
    let facts = error.facts().unwrap();
    assert_eq!(facts.model(), None);
    assert_eq!(facts.records(), 3);
    let sends = if cached { 3 } else { 4 };
    assert_eq!(facts.requests_sent(), sends);
    assert_eq!(facts.cache_answers(), u64::from(cached));
    assert_eq!(facts.attempts().unwrap().len(), sends as usize);
    let doc = serde_json::to_value(error.complete()).unwrap();
    assert!(doc["facts"].get("model").is_none());
    assert!(doc["facts"]["call_id"].is_string());
    assert!(doc.get("value").is_none());
    assert!(!format!("{error:?} {doc}").contains("PRIVATE_RESPONSE"));
    let actual = reported_models(events.into_inner().unwrap());
    assert_eq!(actual, ["fixed", "other", "fixed"]);
    assert_eq!(listener.count(), 4);
    drop(engine);
    std::fs::remove_dir_all(place).unwrap();
}

#[cfg(test)]
fn reported_models(events: Vec<OwnedRecordObservation>) -> Vec<String> {
    events
        .into_iter()
        .filter_map(|event| {
            if let OwnedRecordObservation::Question { detail, .. } = event {
                detail
                    .detail()
                    .question_sources()
                    .first()
                    .map(|source| source.answered_by().to_owned())
            } else {
                None
            }
        })
        .collect()
}
