//! The public bulk call must retain the literal portable Max boundaries.

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
fn public_bulk_keeps_portable_max_bodies_and_row_identities() {
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
        Some((5, 3))
    );
    let expected: Vec<&[u8]> = BODIES
        .iter()
        .map(|body| body.strip_suffix('\n').expect("fixture newline").as_bytes())
        .collect();
    // Two requests may be in flight, so they can arrive in either order.
    let requests = listener.requests();
    let mut arrived: Vec<&[u8]> = requests.iter().map(|sent| sent.body.as_slice()).collect();
    arrived.sort_unstable();
    let mut sorted = expected.clone();
    sorted.sort_unstable();
    assert_eq!(arrived, sorted);
    let hashes: Vec<_> = expected
        .iter()
        .map(|body| super::identity::request_digest(listener.url(), body))
        .collect();
    assert_eq!(
        *seen.lock().expect("observations"),
        [
            (0, vec![hashes[0].clone()]),
            (1, vec![hashes[0].clone()]),
            (2, vec![hashes[1].clone()]),
            (3, vec![hashes[1].clone()]),
            (4, vec![hashes[2].clone()]),
        ]
    );
    assert_eq!(listener.count(), 3);
}
