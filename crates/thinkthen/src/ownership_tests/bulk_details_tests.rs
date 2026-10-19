//! Stays: private bulk primitives retain original values and packed request ownership that public carriers cannot show.
//! Exact Details ownership and packed serialization on the shipped frame primitive.
use crate::{BatchSetting, CallOptions, Engine, Question, QuestionSet};
use conformance_backend::{Canned, Listener};
use std::sync::Mutex;
const DECIDED: &str = r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":3,"output_tokens":1}}"#;
fn engine(base: &str) -> Engine {
    crate::api::builder(base, "sk-public-batches")
        .throttle(2)
        .expect("throttle")
        .no_cache()
        .build()
        .expect("engine")
}
fn question() -> Question {
    Question::decide("Does this ask for a refund?")
        .expect("question")
        .cut()
}
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
fn dynamic_details_keep_original_input_and_one_request() {
    let reply = r#"{"model":"jev-latest","answers":{"q1":{"type":"choice","choice":"first","probabilities":{"first":0.9,"second":0.1}},"q2":{"type":"choice","choice":"second","probabilities":{"first":0.2,"second":0.8}}},"usage":{"input_tokens":5,"output_tokens":3}}"#;
    let listener = Listener::answering(move |_| Canned::ok(reply)).expect("listener");
    let engine = crate::api::builder(listener.base(), "sk-public-batches")
        .prices_usd_per_million("0.25", "0.25")
        .expect("prices")
        .no_cache()
        .build()
        .expect("engine");
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
    let mut batch = engine.try_details_input_many_with(
        &asked,
        records.into_iter().map(Ok),
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
        assert!(
            json["meta"].get("batch").is_none(),
            "ADR 0111 drops meta.batch"
        );
        assert_eq!(json["meta"]["batch_setting"], 2, "ticket 0349");
    }
    assert_eq!(
        batch
            .facts()
            .map(|facts| (facts.records(), facts.requests_sent())),
        Some((2, 1))
    );
    assert_eq!(
        batch
            .facts()
            .and_then(|facts| facts.estimated_cost_usd().map(str::to_owned)),
        Some("0.000002".to_owned())
    );
}

#[test]
fn split_record_details_keep_one_question_key_each() {
    let listener = Listener::serving(vec![
        Canned::status(413, "too large"),
        Canned::ok(DECIDED),
        Canned::ok(DECIDED),
    ])
    .expect("listener");
    let engine = engine(listener.base());
    let asked = question();
    let mut batch = engine.try_details_input_many_with(
        &asked,
        ["alpha", "beta"].map(Ok),
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
        assert!(json["meta"].get("batch").is_none());
        assert_eq!(
            json["meta"]["requests_sent"],
            if position == 0 { 2 } else { 1 }
        );
        assert_eq!(json["meta"]["requests"].as_array().map(Vec::len), Some(1));
        assert_eq!(row.value().requests().len(), 1);
    }
}

#[test]
fn fallible_input_keeps_packed_detail_and_grouped_annotation_request_bytes() {
    let captured = std::sync::Arc::new(Mutex::new(Vec::new()));
    let observed = std::sync::Arc::clone(&captured);
    let listener = Listener::answering(move |body| {
        observed
            .lock()
            .expect("capture")
            .push(String::from_utf8(body.to_vec()).expect("body"));
        let request: serde_json::Value = serde_json::from_slice(body).expect("request");
        let answers: serde_json::Map<String, serde_json::Value> = request["questions"]
            .as_object()
            .expect("questions")
            .keys()
            .map(|key| (key.clone(), serde_json::json!({"type":"noul","noul":0.9})))
            .collect();
        Canned::ok(&serde_json::json!({"model":"jev-latest","answers":answers}).to_string())
    })
    .expect("listener");
    let engine = engine(listener.base());
    let asked = question();
    let set = QuestionSet::from_json(
        r#"{"version":1,"questions":{"first":{"decide":"First?"},"second":{"decide":"Second?"}}}"#,
    )
    .expect("set");
    let records = ["alpha", "beta", "gamma", "delta"];
    let options = CallOptions::new().batch(BatchSetting::Records(
        std::num::NonZeroUsize::new(2).expect("two"),
    ));
    for annotation in [false, true] {
        let original_len = if annotation {
            engine
                .annotate_with(&set, records, options)
                .expect("annotation")
                .value()
                .len()
        } else {
            engine
                .try_details_input_many_with(&asked, records.map(Ok), options)
                .collect::<Result<Vec<_>, _>>()
                .expect("old detail")
                .len()
        };
        assert_eq!(original_len, 4);
        let mut original = std::mem::take(&mut *captured.lock().expect("capture"));
        assert_eq!(original.len(), 2);
        if annotation {
            let rows = crate::api::rows(
                &engine,
                thinkthen::RequestFunction::Annotate,
                set.clone().into(),
                crate::api::plain_records(records),
                options,
            )
            .expect("annotation rows");
            assert_eq!(
                crate::api::annotation_rows(rows)
                    .expect("annotations")
                    .collect::<Result<Vec<_>, _>>()
                    .expect("fallible annotation")
                    .len(),
                4
            );
        } else {
            assert_eq!(
                crate::api::decision_rows(
                    crate::api::rows(
                        &engine,
                        thinkthen::RequestFunction::Decide,
                        asked.clone().into(),
                        records.map(|original| Ok(crate::api::record(original))),
                        options,
                    )
                    .expect("request rows")
                )
                .expect("decisions")
                .collect::<Result<Vec<_>, _>>()
                .expect("fallible detail")
                .len(),
                4
            );
        }
        let mut fallible = std::mem::take(&mut *captured.lock().expect("capture"));
        original.sort();
        fallible.sort();
        assert_eq!(original, fallible);
    }
    assert_eq!(listener.count(), 8);
}
