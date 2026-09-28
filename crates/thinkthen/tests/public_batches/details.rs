//! Dynamic questions keep full input and bounded record details without an observer.

use super::*;

#[derive(serde::Serialize)]
struct Original<'a> {
    id: &'a str,
    body: &'a str,
}

impl thinkthen::Evidence for Original<'_> {
    fn evidence(&self) -> &str {
        self.body
    }
}

#[test]
fn dynamic_details_keep_original_input_and_one_batch_receipt() {
    let _serial = serial();
    let reply = r#"{"model":"jev-latest","answers":{"q1":{"type":"choice","choice":"first","probabilities":{"first":0.9,"second":0.1}},"q2":{"type":"choice","choice":"second","probabilities":{"first":0.2,"second":0.8}}},"usage":{"input_tokens":5,"output_tokens":3}}"#;
    let listener = Listener::answering(move |_| Canned::ok(reply)).expect("listener");
    let engine = engine(listener.base());
    let thinkthen::LoadedQuestion::Question(asked) =
        Question::from_json(r#"{"choose":"Which label?","options":["first","second"]}"#)
            .expect("dynamic question")
    else {
        panic!("choice question");
    };
    let records = [
        Original {
            id: "one",
            body: "alpha",
        },
        Original {
            id: "two",
            body: "beta",
        },
    ];
    let mut batch = engine.details_many_with(
        &asked,
        records,
        CallOptions::new().batch(BatchSetting::Records(
            std::num::NonZeroUsize::new(2).expect("two"),
        )),
    );
    let rows = batch.by_ref().collect::<Result<Vec<_>, _>>().expect("rows");
    assert_eq!(listener.count(), 1);
    assert_eq!(rows.len(), 2);
    for (position, row) in rows.iter().enumerate() {
        let json: serde_json::Value =
            serde_json::from_str(&row.value().to_json()).expect("detail JSON");
        assert_eq!(
            json["input"]["id"],
            if position == 0 { "one" } else { "two" }
        );
        assert_eq!(
            json["input"]["body"],
            if position == 0 { "alpha" } else { "beta" }
        );
        assert_eq!(json["meta"]["batch"]["records"], 2);
        assert_eq!(json["meta"]["batch"]["position"], position + 1);
        assert_eq!(
            json["meta"]["requests_sent"],
            if position == 0 { 1 } else { 0 }
        );
    }
    assert_eq!(
        batch
            .facts()
            .map(|facts| (facts.records(), facts.requests_sent())),
        Some((2, 1))
    );
}

#[test]
fn split_record_details_name_the_refused_parent_and_the_answering_half() {
    let _serial = serial();
    let listener = Listener::serving(vec![
        Canned::status(413, "too large"),
        Canned::ok(DECIDED),
        Canned::ok(DECIDED),
    ])
    .expect("listener");
    let engine = engine(listener.base());
    let asked = question();
    let mut batch = engine.details_many_with(
        &asked,
        ["alpha", "beta"],
        CallOptions::new().batch(BatchSetting::Records(
            std::num::NonZeroUsize::new(2).expect("two"),
        )),
    );
    let rows = batch
        .by_ref()
        .collect::<Result<Vec<_>, _>>()
        .expect("split rows");
    assert_eq!(listener.connections(), 3);
    assert_eq!(batch.facts().map(|facts| facts.requests_sent()), Some(3));
    for (position, row) in rows.iter().enumerate() {
        let json: serde_json::Value =
            serde_json::from_str(&row.value().to_json()).expect("detail JSON");
        assert_eq!(json["meta"]["batch"]["split"], true);
        assert_eq!(json["meta"]["batch"]["records"], 1);
        assert_eq!(
            json["meta"]["requests_sent"],
            if position == 0 { 2 } else { 1 }
        );
        assert_eq!(json["meta"]["requests"].as_array().map(Vec::len), Some(2));
        assert_eq!(row.value().requests().len(), 2);
    }
}
