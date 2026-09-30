//! Actual request packing and completed facts at the Polars trait boundary.

use super::*;
use conformance_backend::{Canned, Listener, Recorded};
use serde_json::{Map, Value, json};
use sha2::{Digest as _, Sha256};
use std::sync::Mutex;
use thinkthen::{BatchSetting, RecordObservation};

type Seen = Mutex<Vec<(usize, Vec<String>)>>;

fn listener() -> Listener {
    Listener::answering(|body| {
        let request: Value = serde_json::from_slice(body).expect("request JSON");
        let answers: Map<String, Value> = request
            .get("questions")
            .and_then(Value::as_object)
            .expect("question map")
            .keys()
            .map(|key| (key.clone(), json!({"type":"noul","noul":0.9})))
            .collect();
        Canned::ok(&json!({"model":"jev-latest","answers":answers}).to_string())
    })
    .expect("listener")
}

fn assert_decided(call: &thinkthen::Call<Series>, sends: u64) {
    assert_eq!(
        call.value()
            .bool()
            .expect("Boolean")
            .iter()
            .collect::<Vec<_>>(),
        [Some(true); 3]
    );
    assert_eq!(
        (call.facts().records(), call.facts().requests_sent()),
        (3, sends)
    );
}

fn question_count(request: &Recorded) -> usize {
    serde_json::from_slice::<Value>(&request.body)
        .expect("request JSON")
        .get("questions")
        .and_then(Value::as_object)
        .expect("question map")
        .len()
}

fn digest(url: &str, body: &[u8]) -> String {
    let mut hash = Sha256::new();
    hash.update(b"systemone\n");
    hash.update(url.as_bytes());
    hash.update(b"\n");
    hash.update(body);
    hash.finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn assert_portable_exchange(listener: &Listener, expected: [&str; 3], observed: &Seen) {
    let literal: Vec<_> = expected
        .iter()
        .map(|body| body.strip_suffix('\n').expect("fixture newline").as_bytes())
        .collect();
    assert_eq!(
        listener
            .requests()
            .iter()
            .map(|request| request.body.as_slice())
            .collect::<Vec<_>>(),
        literal
    );
    let [first, second, third] = expected.map(|body| {
        digest(
            listener.url(),
            body.strip_suffix('\n').expect("fixture newline").as_bytes(),
        )
    });
    assert_eq!(
        *observed.lock().expect("observations"),
        [
            (0, vec![first.clone()]),
            (1, vec![first]),
            (2, vec![second.clone()]),
            (3, vec![second]),
            (4, vec![third]),
        ]
    );
}

#[test]
fn portable_max_cuts_cross_public_series_and_frame_calls() {
    let corpus: Value = serde_json::from_str(include_str!(
        "../../../../specification/fixtures/batching/portable-records.json"
    ))
    .expect("literal corpus");
    let texts: Vec<&str> = corpus["texts"]
        .as_array()
        .expect("five texts")
        .iter()
        .map(|value| value.as_str().expect("text"))
        .collect();
    let source = common::column(&texts);
    let listener = listener();
    let engine = common::builder(listener.base())
        .model("jev-1.13.0")
        .expect("model")
        .throttle(1)
        .expect("one in flight")
        .no_cache()
        .build()
        .expect("engine");
    let question = Question::decide(corpus["question"].as_str().expect("question"))
        .expect("question")
        .cut();
    let set = QuestionSet::from_json(&format!(
        r#"{{"version":1,"questions":{{"answer":{{"decide":{}}}}}}}"#,
        corpus["question"]
    ))
    .expect("one-member set");
    let bodies = [
        include_str!("../../../../specification/fixtures/batching/portable-1.request.json"),
        include_str!("../../../../specification/fixtures/batching/portable-2.request.json"),
        include_str!("../../../../specification/fixtures/batching/portable-3.request.json"),
    ];
    // A series and a frame quote each record the same way, so both send the same bodies.
    for through_frame in [false, true] {
        let observed = Mutex::new(Vec::new());
        let observe = |event: RecordObservation<'_>| {
            if let RecordObservation::Question { index, detail, .. } = event {
                observed
                    .lock()
                    .expect("observations")
                    .push((index, detail.requests().to_vec()));
            }
        };
        let options = CallOptions::new().observe(&observe);
        let call = if through_frame {
            let answered = engine
                .annotate_frame(&set, &frame(vec![source.clone()]), "body", options)
                .expect("public frame");
            assert_eq!(
                answered
                    .value()
                    .column("answer")
                    .expect("answer")
                    .bool()
                    .expect("Boolean")
                    .iter()
                    .collect::<Vec<_>>(),
                [Some(true); 5]
            );
            (answered.facts().records(), answered.facts().requests_sent())
        } else {
            let answered = engine
                .decide_series(&question, &source, options)
                .expect("public series");
            assert_eq!(
                answered
                    .value()
                    .bool()
                    .expect("Boolean")
                    .iter()
                    .collect::<Vec<_>>(),
                [Some(true); 5]
            );
            (answered.facts().records(), answered.facts().requests_sent())
        };
        assert_eq!(call, (5, 3));
        assert_portable_exchange(&listener, bodies, &observed);
    }
    assert_eq!(listener.count(), 6);
}

fn assert_observed(listener: &Listener, requests: &[Recorded], observed: &Seen) {
    let actual = ["Come Together", "Because", "Help"].map(|record| {
        let quoted =
            format!("The text is \"{record}\". The text is the title of a song by the Beatles.");
        let request = requests
            .iter()
            .find(|request| {
                serde_json::from_slice::<Value>(&request.body).expect("request JSON")["questions"]
                    ["q1"]["instructions"]
                    .as_str()
                    == Some(quoted.as_str())
            })
            .expect("quoted request");
        digest(listener.url(), &request.body)
    });
    let observed = observed.lock().expect("observations");
    assert_eq!(observed.len(), 3);
    for ((index, requests), (wanted, digest)) in observed.iter().zip(actual.into_iter().enumerate())
    {
        assert_eq!(*index, wanted);
        assert_eq!(requests.as_slice(), [digest]);
    }
}

#[test]
fn polars_columns_keep_final_call_facts() {
    let listener = listener();
    let engine = common::builder(listener.base())
        .model("jev-latest")
        .expect("model")
        .no_cache()
        .build()
        .expect("engine");
    let question = Question::decide("The text is the title of a song by the Beatles.")
        .expect("question")
        .cut();
    let texts = common::column(&["Come Together", "Because", "Help"]);
    let packed = engine
        .decide_series(&question, &texts, CallOptions::new())
        .expect("packed call");
    assert_decided(&packed, 1);
    let first = listener.requests();
    assert_eq!(first.len(), 1);
    assert_eq!(question_count(&first[0]), 3);
    let packed_body = r#"{"state":"Each question quotes the text it asks about.","model":"jev-latest","questions":{"q1":{"type":"noul","instructions":"The text is \"Come Together\". The text is the title of a song by the Beatles."},"q2":{"type":"noul","instructions":"The text is \"Because\". The text is the title of a song by the Beatles."},"q3":{"type":"noul","instructions":"The text is \"Help\". The text is the title of a song by the Beatles."}}}"#;
    assert_eq!(first[0].body, packed_body.as_bytes());

    let observed = Mutex::new(Vec::new());
    let observe = |event: RecordObservation<'_>| {
        if let RecordObservation::Question { index, detail, .. } = event {
            observed
                .lock()
                .expect("observations")
                .push((index, detail.requests().to_vec()));
        }
    };
    let separate = engine
        .decide_series(
            &question,
            &texts,
            CallOptions::new()
                .batch(BatchSetting::Records(std::num::NonZeroUsize::MIN))
                .observe(&observe),
        )
        .expect("separate call");
    assert_decided(&separate, 3);
    let each = listener.requests();
    assert_eq!(each.len(), 3);
    assert!(each.iter().all(|request| question_count(request) == 1));
    let expected = r#"{"state":"Each question quotes the text it asks about.","model":"jev-latest","questions":{"q1":{"type":"noul","instructions":"The text is \"Come Together\". The text is the title of a song by the Beatles."}}}"#;
    assert!(
        each.iter()
            .any(|request| request.body == expected.as_bytes()),
        "singleton request identity drifted"
    );
    assert_observed(&listener, &each, &observed);

    let contextual = engine
        .decide_series(
            &question,
            &texts,
            CallOptions::new().context("review this claim"),
        )
        .expect("eligible context");
    assert_eq!(
        (
            contextual.facts().records(),
            contextual.facts().requests_sent()
        ),
        (3, 1)
    );
    let context_request = listener.requests();
    assert_eq!(context_request.len(), 1);
    assert_ne!(context_request[0].body, first[0].body);
    assert!(String::from_utf8_lossy(&context_request[0].body).contains("review this claim"));

    let set = QuestionSet::from_json(r#"{"version":1,"questions":{"answer":{"decide":"Ready?"}}}"#)
        .expect("set");
    let before = listener.count();
    let error = engine
        .annotate_frame(
            &set,
            &frame(vec![texts]),
            "body",
            CallOptions::new().context("forbidden"),
        )
        .expect_err("annotation context");
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(listener.count(), before);
}
