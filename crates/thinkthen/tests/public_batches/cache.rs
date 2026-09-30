//! The question store and the one pipeline through the public Rust API, by
//! ADR 0111, counted on loopback listeners.

use super::*;
use serde_json::{Map, Value, json};

/// Each quoted record a request asks about, in wire order.
fn quoted(body: &[u8]) -> Vec<String> {
    let request: Value = serde_json::from_slice(body).expect("request");
    let questions = request
        .get("questions")
        .and_then(Value::as_object)
        .expect("questions");
    (1..=questions.len())
        .map(|place| {
            let asked = questions
                .get(&format!("q{place}"))
                .and_then(|question| question.get("instructions"))
                .and_then(Value::as_str)
                .expect("text instructions");
            let (head, _) = asked.split_once("\". ").expect("a quoted record");
            head.trim_start_matches("The text is \"").to_owned()
        })
        .collect()
}

/// A reply answering every question of the request yes, or each by `rule`.
fn every(body: &[u8], rule: impl Fn(&str) -> f64) -> Canned {
    let request: Value = serde_json::from_slice(body).expect("request");
    let records = quoted(body);
    let answers: Map<String, Value> = request
        .get("questions")
        .and_then(Value::as_object)
        .expect("questions")
        .keys()
        .map(|name| {
            let place: usize = name[1..].parse().expect("qN");
            let noul = rule(records.get(place - 1).expect("a quoted record"));
            (name.clone(), json!({"type": "noul", "noul": noul}))
        })
        .collect();
    Canned::ok(&json!({"model": "jev-1.13.0", "answers": answers}).to_string())
}

fn texts(count: usize) -> Vec<String> {
    (1..=count).map(|at| format!("record {at}")).collect()
}

/// ADR 0111's headline through the public API: after 100 records, a batch
/// of 120 that includes them sends one request holding exactly the 20 new
/// questions, and reports 100 cache answers.
#[test]
fn a_longer_batch_sends_only_the_new_records_questions() {
    let _serial = serial();
    let listener = Listener::answering(|body| every(body, |_| 0.9)).expect("listener");
    let folder = std::path::Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("public-question-cache-{}", std::process::id()));
    let _gone = std::fs::remove_dir_all(&folder);
    let engine = Engine::builder()
        .base_url(listener.base())
        .and_then(|builder| builder.api_key("sk-public-batches"))
        .and_then(|builder| builder.model("jev-1.13.0"))
        .and_then(|builder| builder.throttle(THROTTLE))
        .and_then(|builder| builder.cache_at(&folder))
        .and_then(EngineBuilder::build)
        .expect("engine");
    let asked = question();
    let facts = |count: usize| {
        let mut rows = engine.decide_many(&asked, texts(count));
        let answered = rows.by_ref().filter(Result::is_ok).count();
        let facts = rows.facts().expect("final facts");
        (
            answered,
            facts.records(),
            facts.requests_sent(),
            facts.cache_answers(),
        )
    };
    assert_eq!(facts(100), (100, 100, 1, 0));
    assert_eq!(listener.questions(), 100);
    let _first = listener.requests();
    assert_eq!(facts(120), (120, 120, 1, 100));
    let second = listener.requests();
    assert_eq!(second.len(), 1);
    assert_eq!(quoted(&second[0].body), texts(120)[100..]);
    assert_eq!(
        listener.questions(),
        120,
        "20 questions after the first 100"
    );
    let _gone = std::fs::remove_dir_all(&folder);
}

/// Requests of two records go out two at a time. The requests for records
/// 1 and 2 and for 5 and 6 answer slowly, so the fast ones behind them finish
/// first, and the rows still come back in input order with their own answers.
#[test]
fn rows_keep_input_order_when_replies_finish_out_of_order() {
    let _serial = serial();
    let number = |record: &str| -> usize { record["record ".len()..].parse().expect("a number") };
    let odd = move |record: &str| number(record) % 2 == 1;
    let listener = Listener::answering(move |body| {
        let slow = quoted(body).iter().any(|record| number(record) % 4 == 1);
        let reply = every(body, |record| if odd(record) { 0.9 } else { 0.1 });
        if slow { reply.after(120) } else { reply }
    })
    .expect("listener");
    let engine = engine(listener.base());
    let asked = question();
    let rows = engine
        .decide_many_with(
            &asked,
            texts(8),
            CallOptions::new().batch(BatchSetting::Records(
                std::num::NonZeroUsize::new(2).expect("two"),
            )),
        )
        .map(|row| row.map(Row::into_parts))
        .collect::<Result<Vec<_>, _>>()
        .expect("rows");
    let expected: Vec<(String, Answer)> = texts(8)
        .into_iter()
        .enumerate()
        .map(|(at, text)| (text, if at % 2 == 0 { Answer::Yes } else { Answer::No }))
        .collect();
    assert_eq!(rows, expected);
    assert_eq!(listener.count(), 4);
    assert_eq!(listener.peak(), usize::from(THROTTLE));
}
