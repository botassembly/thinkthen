//! Refused parent requests and their one eligible split at the public boundary.

use super::*;
use sha2::{Digest as _, Sha256};

fn digest(url: &str, body: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"systemone\n");
    hasher.update(url.as_bytes());
    hasher.update(b"\n");
    hasher.update(body);
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn packed() -> CallOptions<'static> {
    CallOptions::new().batch(BatchSetting::Records(
        std::num::NonZeroUsize::new(2).expect("two"),
    ))
}

#[test]
fn a_refused_parent_and_both_halves_keep_exact_request_shares() {
    let _serial = serial();
    let listener = Listener::serving(vec![
        Canned::status(413, "too large"),
        Canned::ok(DECIDED),
        Canned::ok(DECIDED),
    ])
    .expect("scripted listener");
    let engine = engine(listener.base());
    let seen = Mutex::new(Vec::new());
    let observe = |event: RecordObservation<'_>| {
        if let RecordObservation::Question { index, detail, .. } = event {
            seen.lock().expect("observations").push((
                index,
                detail.requests().to_vec(),
                detail.requests_sent(),
            ));
        }
    };
    let question = question();
    let mut rows =
        engine.decide_many_with(&question, ["alpha", "beta"], packed().observe(&observe));
    let values = rows
        .by_ref()
        .collect::<Result<Vec<_>, _>>()
        .expect("halves");
    assert_eq!(values.len(), 2);
    assert!(values.iter().all(|row| *row.value() == Answer::Yes));
    assert_eq!(
        rows.facts()
            .map(|facts| (facts.records(), facts.requests_sent())),
        Some((2, 3))
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 3);
    let sizes = requests
        .iter()
        .map(|request| {
            let body: serde_json::Value = serde_json::from_slice(&request.body).expect("body");
            body["questions"].as_object().expect("questions").len()
        })
        .collect::<Vec<_>>();
    assert_eq!(sizes, [2, 1, 1]);
    let digests = requests
        .iter()
        .map(|request| digest(listener.url(), &request.body))
        .collect::<Vec<_>>();
    let seen = seen.lock().expect("observations");
    assert_eq!(
        seen.as_slice(),
        [
            (0, vec![digests[0].clone(), digests[1].clone()], 2),
            (1, vec![digests[0].clone(), digests[2].clone()], 1),
        ]
    );
}

#[test]
fn a_failed_left_half_prevents_a_right_send_and_row() {
    let _serial = serial();
    let missing = r#"{"model":"jev-latest","answers":{}}"#;
    let listener = Listener::serving(vec![
        Canned::status(413, "too large"),
        Canned::ok(missing),
        Canned::ok(DECIDED),
    ])
    .expect("scripted listener");
    let engine = engine(listener.base());
    let seen = Mutex::new(Vec::new());
    let observe = |event: RecordObservation<'_>| {
        if let RecordObservation::Question { index, detail, .. } = event {
            seen.lock().expect("observations").push((
                index,
                detail.failure(),
                detail.requests().to_vec(),
                detail.requests_sent(),
            ));
        }
    };
    let question = question();
    let mut rows =
        engine.decide_many_with(&question, ["alpha", "beta"], packed().observe(&observe));
    let error = rows
        .next()
        .expect("terminal error")
        .expect_err("failed left");
    assert_eq!(error.kind(), ErrorKind::Backend);
    assert!(rows.next().is_none());
    assert_eq!(
        rows.facts()
            .map(|facts| (facts.records(), facts.requests_sent())),
        Some((0, 2))
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 2, "the right half was not sent");
    assert!(seen.lock().expect("observations").is_empty());
}
