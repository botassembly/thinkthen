//! Pure second-route behavior through existing public calls and owned loopback responses.
use conformance_backend::{Canned, Listener};
use serde_json::{Value, json};
use std::sync::atomic::{AtomicUsize, Ordering};
use thinkthen::{Answer, BatchSetting, CallOptions, Engine, ErrorKind, Question, QuestionSet};

#[cfg(test)]
fn folder() -> std::path::PathBuf {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let folder = std::env::temp_dir().join(format!(
        "thinkthen-decisions-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir(&folder).unwrap();
    folder
}
#[cfg(test)]
fn builder(listener: &Listener) -> thinkthen::EngineBuilder {
    Engine::builder()
        .backend("openai")
        .unwrap()
        .base_url(listener.base())
        .unwrap()
        .model("fixed")
        .unwrap()
        .api_key("decisions-test-private")
        .unwrap()
        .max_retries(0)
}
#[cfg(test)]
fn response(body: &[u8]) -> Canned {
    let request: Value = serde_json::from_slice(body).unwrap();
    assert!(request.get("state").is_none());
    assert!(request["input"].is_string());
    let answers = request["questions"].as_array().unwrap().iter().map(|question| {
        let name = &question["name"];
        match question["type"].as_str().unwrap() {
            "predicate" => json!({"type":"predicate","name":name,"probability":0.9}),
            "choice" => {
                let choices = question["choices"].as_array().unwrap();
                let picked = picked(choices,question["instructions"].as_str().unwrap());
                let probabilities = choices.iter().map(|choice| json!({"value":choice["value"],"probability":u8::from(choice["value"] == picked)})).collect::<Vec<_>>();
                json!({"type":"choice","name":name,"choice":picked,"probabilities":probabilities,"confidence":0.7})
            }
            "score" => {
                let probabilities = question["levels"].as_array().unwrap().iter().enumerate().map(|(index,level)| json!({"value":index,"label":level["label"],"probability":u8::from(index==1)})).collect::<Vec<_>>();
                json!({"type":"score","name":name,"score":1.0,"probabilities":probabilities,"confidence":0.8})
            }
            _ => panic!("unexpected type"),
        }
    }).collect::<Vec<_>>();
    Canned::ok(
        &json!({"model":"fixed","answers":answers,"usage":{"input_tokens":17,"output_tokens":0}})
            .to_string(),
    )
}
#[cfg(test)]
fn picked(choices: &[Value], instructions: &str) -> Value {
    let has = |value| choices.iter().any(|choice| choice["value"] == value);
    if has("SINGLE") {
        return json!(
            if instructions.contains("[[Ada]]") || instructions.contains("[[Acme]]") {
                "SINGLE"
            } else {
                "OUT"
            }
        );
    }
    if has("none of these") {
        return json!(if instructions.contains("[[Acme]]") {
            "organization"
        } else {
            "person"
        });
    }
    choices[0]["value"].clone()
}
#[test]
fn primitive_text_functions_keep_distributions_usage_and_secrecy() {
    let listener = Listener::answering(response).unwrap();
    let engine = builder(&listener).no_cache().build().unwrap();
    let decide = Question::decide("Good?").unwrap().cut();
    let choose = Question::choose_labels("Team?")
        .unwrap()
        .label("billing", None)
        .unwrap()
        .label("shipping", None)
        .unwrap()
        .build()
        .unwrap();
    let tag = Question::tag_labels("Applies?")
        .unwrap()
        .label("refund", None)
        .unwrap()
        .label("urgent", None)
        .unwrap()
        .build()
        .unwrap();
    let score = Question::score("Severity?")
        .unwrap()
        .level("low", None)
        .unwrap()
        .level("high", None)
        .unwrap()
        .build()
        .unwrap();
    let decision = engine
        .decide_complete_with(&decide, "Input.", CallOptions::new().attempts(true))
        .unwrap();
    assert_eq!(decision.value().value(), Answer::Yes);
    assert_eq!(decision.facts().model(), Some("fixed"));
    assert_eq!(decision.facts().input_tokens(), Some(17));
    assert_eq!(decision.facts().output_tokens(), Some(0));
    engine
        .choose_complete_with(&choose, "Input.", CallOptions::new())
        .unwrap();
    engine
        .tag_complete_with(&tag, "Input.", CallOptions::new())
        .unwrap();
    engine
        .score_complete_with(&score, "Input.", CallOptions::new())
        .unwrap();
    assert!(!format!("{decision:?} {engine:?}").contains("decisions-test-private"));
    assert_routes(&listener);
}
#[test]
fn composed_text_functions_share_the_decisions_route_and_existing_planners() {
    use thinkthen::{Entity, Kind, Recognize, Relate, RelationRule};
    let listener = Listener::answering(response).unwrap();
    let engine = builder(&listener).no_cache().build().unwrap();
    let decide = Question::decide("Good?").unwrap().cut();
    engine
        .filter_complete_with(
            &decide,
            ["One.", "Two."],
            CallOptions::new().batch(BatchSetting::Max),
        )
        .unwrap();
    engine
        .rank_complete_with(
            &Question::rank("Good?").unwrap(),
            ["One.", "Two."],
            CallOptions::new(),
        )
        .unwrap();
    engine
        .find_complete_with(
            &Question::find("Where?").unwrap(),
            ["One.", "Two."],
            CallOptions::new(),
        )
        .unwrap();
    let set = QuestionSet::from_json(r#"{"version":1,"questions":{"first":{"decide":"Good?"},"second":{"choose":"Team?","options":["billing","shipping"]}}}"#).unwrap();
    engine
        .annotate_complete_with(&set, ["One."], CallOptions::new())
        .unwrap();
    let recognize = Recognize::builder()
        .kind(Kind::new("person", None).unwrap())
        .unwrap()
        .kind(Kind::new("organization", None).unwrap())
        .unwrap()
        .build()
        .unwrap();
    let found = engine
        .recognize_complete_with(&recognize, "Ada met Acme.", CallOptions::new())
        .unwrap();
    assert_eq!(found.value().value().entities().len(), 2);
    let relate = Relate::builder()
        .relation(RelationRule::one_way("follows", "person", "person").unwrap())
        .unwrap()
        .build()
        .unwrap();
    engine
        .relate_complete_with(
            &relate,
            [
                Entity::new("Ada", "person").unwrap(),
                Entity::new("Grace", "person").unwrap(),
            ],
            CallOptions::new(),
        )
        .unwrap();
    assert_routes(&listener);
}
#[cfg(test)]
fn assert_routes(listener: &Listener) {
    for request in listener.requests() {
        assert_eq!(request.line, "POST /v1/decisions HTTP/1.1");
        assert_eq!(
            request.header("authorization"),
            Some("Bearer decisions-test-private")
        );
        assert!(
            !String::from_utf8(request.body)
                .unwrap()
                .contains("decisions-test-private")
        );
    }
}
#[test]
fn unchanged_second_question_reuses_observation_after_repacking_and_raw_recording_keeps_names() {
    let listener = Listener::answering(response).unwrap();
    let place = folder();
    let set = QuestionSet::from_json(
        r#"{"version":1,"questions":{"first":{"decide":"A?"},"second":{"decide":"B?"}}}"#,
    )
    .unwrap();
    let only =
        QuestionSet::from_json(r#"{"version":1,"questions":{"second":{"decide":"B?"}}}"#).unwrap();
    let engine = builder(&listener).record(&place).unwrap().build().unwrap();
    let first = engine
        .annotate_complete_with(&set, ["Input."], CallOptions::new())
        .unwrap();
    let cached = builder(&listener)
        .cache_at(&place)
        .unwrap()
        .build()
        .unwrap();
    let second = cached
        .annotate_complete_with(&only, ["Input."], CallOptions::new())
        .unwrap();
    let a = first.value()[0].result().members().nth(1).unwrap();
    let b = second.value()[0].result().members().next().unwrap();
    // Removing a named member changes logical position, while retaining its observation.
    assert_eq!(a.observations(), b.observations());
    assert_eq!(listener.count(), 1);
    let changed = QuestionSet::from_json(
        r#"{"version":1,"questions":{"first":{"decide":"Different A?"},"second":{"decide":"B?"}}}"#,
    )
    .unwrap();
    let repacked = cached
        .annotate_complete_with(&changed, ["Input."], CallOptions::new())
        .unwrap();
    let c = repacked.value()[0].result().members().nth(1).unwrap();
    assert_eq!(a.answer_id(), c.answer_id());
    assert_eq!(a.observations(), c.observations());
    assert_eq!(listener.count(), 2);
    let sent = listener.requests();
    let remainder: Value = serde_json::from_slice(&sent[1].body).unwrap();
    assert_eq!(remainder["questions"].as_array().unwrap().len(), 1);
    assert!(
        remainder["questions"][0]["instructions"]
            .as_str()
            .unwrap()
            .contains("Different A?")
    );
    let db = rusqlite::Connection::open(place.join("thinkthen.sqlite")).unwrap();
    let (question, answer, adapter): (String, String, String) = db
        .query_row(
            "SELECT question,answer,adapter FROM answers WHERE question LIKE '%B?%'",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .unwrap();
    assert_eq!(adapter, "openai-decisions");
    assert!(!question.contains("\"name\""));
    assert!(!answer.contains("\"name\""));
    let (request, response): (Vec<u8>, Vec<u8>) = db
        .query_row("SELECT request,response FROM exchanges", [], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })
        .unwrap();
    assert!(String::from_utf8(request).unwrap().contains("\"q2\""));
    assert!(String::from_utf8(response).unwrap().contains("\"q2\""));
    drop(db);
    #[cfg(feature = "cli")]
    {
        let converted = std::process::Command::new(env!("CARGO_BIN_EXE_thinkthen"))
            .args(["cache", "convert"])
            .arg(&place)
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .env("HOME", &place)
            .env("LANG", "C.UTF-8")
            .output()
            .unwrap();
        assert!(
            converted.status.success(),
            "{}",
            String::from_utf8_lossy(&converted.stderr)
        );
        assert!(place.join("thinkthen.jsonl").exists());
    }
    let replay = builder(&listener).replay(&place).unwrap().build().unwrap();
    replay
        .annotate_complete_with(&only, ["Input."], CallOptions::new())
        .unwrap();
    assert_eq!(listener.count(), 2);
    drop(replay);
    drop(cached);
    drop(engine);
    std::fs::remove_dir_all(place).unwrap();
}
#[test]
fn failures_refusals_and_invalid_input_never_become_false_or_trigger_response_retries() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":[{"type":"refusal","name":"q1","reason":"private"}],"usage":{"input_tokens":17,"output_tokens":0}}"#)).unwrap();
    let engine = builder(&listener)
        .no_cache()
        .max_retries(3)
        .build()
        .unwrap();
    let question = Question::decide("Good?").unwrap().cut();
    let error = engine.decide(&question, "Input.").unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Backend);
    assert_eq!(error.facts().unwrap().requests_sent(), 1);
    assert_eq!(error.facts().unwrap().model(), Some("fixed"));
    assert_eq!(error.facts().unwrap().input_tokens(), Some(17));
    assert_eq!(error.facts().unwrap().output_tokens(), Some(0));
    assert!(!format!("{error:?} {error}").contains("private"));
    assert_eq!(listener.count(), 1);
    assert_eq!(
        engine.decide(&question, "").unwrap_err().kind(),
        ErrorKind::Usage
    );
    assert_eq!(listener.count(), 1);
}

#[test]
fn same_endpoint_and_model_keep_adapter_answers_separate_and_wrong_replay_sends_nothing() {
    let listener = Listener::answering(|body| {
        let request: Value = serde_json::from_slice(body).unwrap();
        if request["questions"].is_array() {
            response(body)
        } else {
            Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.1}}}"#)
        }
    })
    .unwrap();
    let place = folder();
    let question = Question::decide("Good?").unwrap().cut();
    let first = builder(&listener)
        .cache_at(&place)
        .unwrap()
        .build()
        .unwrap();
    assert_eq!(
        *first.decide(&question, "Input.").unwrap().value(),
        Answer::Yes
    );
    let other = builder(&listener)
        .backend("perplexity")
        .unwrap()
        .cache_at(&place)
        .unwrap()
        .build()
        .unwrap();
    assert_eq!(
        *other.decide(&question, "Input.").unwrap().value(),
        Answer::No
    );
    assert_eq!(
        *first.decide(&question, "Input.").unwrap().value(),
        Answer::Yes
    );
    assert_eq!(listener.count(), 2);
    let db = rusqlite::Connection::open(place.join("thinkthen.sqlite")).unwrap();
    let variants: u32 = db
        .query_row("SELECT count(DISTINCT adapter) FROM answers", [], |row| {
            row.get(0)
        })
        .unwrap();
    assert_eq!(variants, 2);
    drop(db);
    drop(first);
    drop(other);
    std::fs::remove_dir_all(place).unwrap();
    let empty = folder();
    let replay = builder(&listener).replay(&empty).unwrap().build().unwrap();
    assert_eq!(
        replay.decide(&question, "Input.").unwrap_err().kind(),
        ErrorKind::Local
    );
    assert_eq!(listener.count(), 2);
    drop(replay);
    std::fs::remove_dir_all(empty).unwrap();
}

#[test]
fn status_retries_keep_body_route_and_key_and_usage_absence_is_not_zero() {
    let attempts = std::sync::Arc::new(AtomicUsize::new(0));
    let seen = attempts.clone();
    let listener = Listener::answering(move |body| {
        if seen.fetch_add(1, Ordering::SeqCst) == 0 {
            Canned::status(429, "private").asking("retry-after-ms", "1")
        } else {
            response(body).asking("x-request-id", "observed-id")
        }
    })
    .unwrap();
    let engine = builder(&listener)
        .no_cache()
        .max_retries(1)
        .build()
        .unwrap();
    let question = Question::decide("Good?").unwrap().cut();
    let call = engine
        .decide_complete_with(&question, "Input.", CallOptions::new().attempts(true))
        .unwrap();
    assert_eq!(call.facts().requests_sent(), 2);
    let requests = listener.requests();
    assert_eq!(requests[0].body, requests[1].body);
    assert_eq!(requests[0].line, requests[1].line);
    assert_eq!(
        requests[0].header("authorization"),
        requests[1].header("authorization")
    );
    let document: Value = serde_json::from_str(&call.value().to_json().unwrap()).unwrap();
    assert_eq!(document["meta"]["attempts"][1]["request_id"], "observed-id");
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"reported","answers":[{"type":"predicate","name":"q1","probability":0.9}]}"#)).unwrap();
    let engine = builder(&listener).no_cache().build().unwrap();
    let call = engine.decide(&question, "Input.").unwrap();
    assert_eq!(call.facts().model(), Some("reported"));
    assert_eq!(call.facts().input_tokens(), None);
    assert_eq!(call.facts().output_tokens(), None);
    let token = thinkthen::CancelToken::new();
    token.cancel();
    let error = engine
        .decide_complete_with(&question, "Input.", CallOptions::new().cancel(&token))
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Cancelled);
    assert_eq!(listener.count(), 1);
}

#[test]
fn images_refuse_before_key_lookup_cache_access_or_transport_on_the_text_route() {
    let listener = Listener::answering(|_| Canned::ok("{}")).unwrap();
    let place = folder();
    let engine = builder(&listener)
        .cache_at(&place)
        .unwrap()
        .build()
        .unwrap();
    let image = thinkthen::ImageInput::new(
        thinkthen::ImageMedia::Png,
        include_bytes!("../../../specification/fixtures/images/red.png").to_vec(),
    )
    .unwrap();
    let input =
        thinkthen::QuestionInput::Images(thinkthen::ImageEvidence::new(None, vec![image]).unwrap());
    let question = Question::decide("Red?").unwrap().cut();
    let error = engine.decide_input(&question, &input).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(listener.count(), 0);
    assert!(!place.join("thinkthen.sqlite").exists());
    drop(engine);
    std::fs::remove_dir_all(place).unwrap();
}

#[path = "openai_decisions/partial.rs"]
mod partial;
