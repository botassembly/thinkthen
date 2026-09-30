//! The public bulk call keeps the portable fixture's question bytes. The
//! content cut is gone, by ADR 0111, so every record rides one request, and
//! each row's key is the key of its fixture question.

use super::*;
use serde_json::{Map, Value, json};

const CORPUS: &str =
    include_str!("../../../../specification/fixtures/batching/portable-records.json");
const BODIES: [&str; 3] = [
    include_str!("../../../../specification/fixtures/batching/portable-1.request.json"),
    include_str!("../../../../specification/fixtures/batching/portable-2.request.json"),
    include_str!("../../../../specification/fixtures/batching/portable-3.request.json"),
];

#[test]
fn public_bulk_keeps_portable_question_bytes_and_keys_in_one_request() {
    let _serial = serial();
    let corpus: Value = serde_json::from_str(CORPUS).expect("literal corpus");
    let texts: Vec<&str> = corpus["texts"]
        .as_array()
        .expect("five texts")
        .iter()
        .map(|value| value.as_str().expect("text"))
        .collect();
    let listener = Listener::answering(|body| {
        let sent: Value = serde_json::from_slice(body).expect("request");
        let answers: Map<String, Value> = sent["questions"]
            .as_object()
            .expect("questions")
            .keys()
            .map(|name| (name.clone(), json!({"type":"noul","noul":0.9})))
            .collect();
        Canned::ok(&json!({"model":"jev-1.13.0","answers":answers}).to_string())
    })
    .expect("listener");
    let engine = Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .api_key("sk-public-batches")
        .expect("key")
        .model("jev-1.13.0")
        .expect("model")
        .throttle(THROTTLE)
        .expect("the process throttle")
        .no_cache()
        .build()
        .expect("engine");
    let question = Question::decide(corpus["question"].as_str().expect("question"))
        .expect("decide question")
        .cut();
    let seen = Mutex::new(Vec::new());
    let observe = |event: RecordObservation<'_>| {
        if let RecordObservation::Question { index, detail, .. } = event {
            seen.lock()
                .expect("observations")
                .push((index, detail.requests().to_vec()));
        }
    };
    let mut call = engine.decide_many_with(&question, texts, CallOptions::new().observe(&observe));
    let answers = call
        .by_ref()
        .collect::<Result<Vec<_>, _>>()
        .expect("five rows");
    assert_eq!(answers.len(), 5);
    assert!(answers.iter().all(|row| *row.value() == Answer::Yes));
    assert_eq!(
        call.facts()
            .map(|facts| (facts.records(), facts.requests_sent())),
        Some((5, 1))
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    let sent: Value = serde_json::from_slice(&requests[0].body).expect("request");
    let fixture: Vec<Value> = BODIES
        .iter()
        .flat_map(|body| {
            let body: Value = serde_json::from_str(body).expect("fixture body");
            let mut questions: Vec<(usize, Value)> = body["questions"]
                .as_object()
                .expect("questions")
                .iter()
                .map(|(name, question)| (name[1..].parse().expect("qN"), question.clone()))
                .collect();
            questions.sort_by_key(|(place, _)| *place);
            questions.into_iter().map(|(_, question)| question)
        })
        .collect();
    let questions: Vec<Value> = (1..=5)
        .map(|place| sent["questions"][format!("q{place}")].clone())
        .collect();
    assert_eq!(questions, fixture, "each question keeps the fixture bytes");
    let keys = super::identity::question_keys(listener.url(), &requests[0].body);
    assert_eq!(
        *seen.lock().expect("observations"),
        keys.into_iter()
            .enumerate()
            .map(|(index, key)| (index, vec![key]))
            .collect::<Vec<_>>()
    );
    assert_eq!(listener.count(), 1);
}
