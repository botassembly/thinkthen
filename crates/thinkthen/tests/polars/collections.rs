//! Public whole-column consumers with independent saved replies and send counts.
use super::common;
use conformance_backend::{Backend, Canned, Listener};
use thinkthen::polars::prelude::{DataFrame, IntoLazy, NamedFrom, Series};
use thinkthen::{BatchSetting, CallOptions, ErrorKind, PolarsEngine, Question, Recognize, Relate};

fn singleton() -> CallOptions<'static> {
    CallOptions::new().batch(BatchSetting::Records(std::num::NonZeroUsize::MIN))
}

#[test]
fn filter_and_rank_keep_nullable_duplicate_row_positions() {
    let listener = collection_listener();
    let engine = common::builder(listener.base())
        .no_cache()
        .build()
        .expect("engine");
    let texts = Series::new(
        "body".into(),
        [Some("strong"), None, Some("weak"), Some("strong")],
    );
    let decide = Question::decide("Relevant?").expect("question").cut();
    let filtered = engine
        .filter_series(&decide, &texts, singleton())
        .expect("filter");
    assert_eq!(filtered.value().name().as_str(), "body");
    assert_eq!(
        filtered
            .value()
            .str()
            .expect("text")
            .iter()
            .collect::<Vec<_>>(),
        [Some("strong"), Some("strong")]
    );
    assert_eq!(filtered.facts().requests_sent(), 3);
    let rank = Question::rank("Relevant?").expect("rank question");
    let ranked = engine
        .rank_series(&rank, &texts, singleton())
        .expect("rank");
    assert_eq!(
        ranked
            .value()
            .column("index")
            .expect("index")
            .u64()
            .expect("places")
            .iter()
            .collect::<Vec<_>>(),
        [Some(0), Some(3), Some(2)]
    );
    assert_eq!(
        ranked
            .value()
            .column("probability")
            .expect("probability")
            .f64()
            .expect("probabilities")
            .iter()
            .collect::<Vec<_>>(),
        [Some(0.9), Some(0.9), Some(0.2)]
    );
    assert_eq!(listener.count(), 6);
}

#[test]
fn find_and_lazy_find_see_all_candidates_in_the_complete_collection() {
    let listener = collection_listener();
    let engine = common::builder(listener.base())
        .no_cache()
        .build()
        .expect("engine");
    let texts = Series::new(
        "body".into(),
        [Some("strong"), None, Some("weak"), Some("strong")],
    );
    let find = Question::find("Which?")
        .expect("find question")
        .offering_none()
        .expect("none");
    let found = engine
        .find_series(&find, &texts, singleton())
        .expect("find");
    assert_eq!(
        found
            .value()
            .column("index")
            .expect("index")
            .u64()
            .expect("places")
            .iter()
            .collect::<Vec<_>>(),
        [Some(0), Some(2), Some(3), None]
    );
    assert_eq!(
        found
            .value()
            .column("probability")
            .expect("probability")
            .f64()
            .expect("probabilities")
            .iter()
            .collect::<Vec<_>>(),
        [Some(0.1), Some(0.8), Some(0.05), Some(0.05)]
    );
    assert_eq!(
        found
            .value()
            .column("selected")
            .expect("selected")
            .bool()
            .expect("booleans")
            .iter()
            .collect::<Vec<_>>(),
        [Some(false), Some(true), Some(false), Some(false)]
    );
    let lazy = DataFrame::new(4, vec![texts.into()]).expect("frame").lazy();
    let materialized = engine
        .find_lazy(&find, lazy, "body", singleton())
        .expect("whole logical set");
    assert_eq!(materialized.value(), found.value());
    assert_eq!(listener.count(), 2);
    let bodies = listener.requests();
    let find_body: serde_json::Value = serde_json::from_slice(&bodies[0].body).expect("body");
    let units: serde_json::Value =
        serde_json::from_str(find_body["state"].as_str().expect("whole-set state")).expect("units");
    assert_eq!(units.as_array().expect("all candidates").len(), 3);
    assert_eq!(units[0]["evidence"], "strong");
    assert_eq!(units[1]["evidence"], "weak");
    assert_eq!(units[2]["evidence"], "strong");
}

#[test]
fn recognition_keeps_complete_spans_and_relations_and_relate_keeps_the_entity_set() {
    let corpus: serde_json::Value =
        serde_json::from_str(include_str!("../../../../conformance/cases.json")).expect("corpus");
    let case = corpus["cases"]
        .as_array()
        .expect("cases")
        .iter()
        .find(|case| case["id"] == "42-recognize-C01-relations")
        .expect("saved case");
    let ask = Recognize::from_json(&case["question"].to_string()).expect("recognize specification");
    let replies = case["exchanges"].as_array().expect("saved exchanges");
    let responses = (0..2)
        .flat_map(|_| {
            replies
                .iter()
                .map(|reply| Canned::ok(&reply["response"].to_string()))
        })
        .collect();
    let saved = Listener::serving(responses).expect("saved reply listener");
    let engine = common::builder(saved.base())
        .no_cache()
        .model("jev-1.13.0")
        .expect("model")
        .build()
        .expect("engine");
    let texts = Series::new(
        "body".into(),
        [case["text"].as_str(), None, case["text"].as_str()],
    );
    let found = engine
        .recognize_series(&ask, &texts, CallOptions::new())
        .expect("recognition");
    assert!(found.value()[1].is_none());
    let first = found.value()[0].as_ref().expect("first");
    assert_eq!(
        first
            .entities()
            .iter()
            .map(|entity| (entity.text(), entity.start(), entity.end(), entity.kind()))
            .collect::<Vec<_>>(),
        [
            ("Maria Chen", 0, 10, "person"),
            ("Northwind Freight", 18, 35, "organization"),
            ("Chicago", 39, 46, "place")
        ]
    );
    assert_eq!(
        first
            .relations()
            .expect("relations")
            .iter()
            .map(|relation| relation.relation())
            .collect::<Vec<_>>(),
        ["works_for"]
    );
    assert_eq!(found.value()[0], found.value()[2]);
    assert_eq!(found.facts().records(), 2);
    assert_eq!(found.facts().requests_sent(), 6);
    assert_eq!(saved.requests().len(), 6);
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.8}}}"#)).expect("relations listener");
    let engine = common::engine(listener.base());
    let ask = Relate::from_json(r#"{"version":1,"relate":{"relations":[{"name":"knows","source":"person","target":"person"}]}}"#).expect("relation spec");
    let frame = DataFrame::new(
        3,
        vec![
            Series::new("name".into(), [Some("Ada"), None, Some("Bo")]).into(),
            Series::new("kind".into(), [Some("person"), None, Some("person")]).into(),
        ],
    )
    .expect("entities frame");
    let related = engine
        .relate_frame(&ask, &frame, "name", "kind", CallOptions::new())
        .expect("relate");
    assert_eq!(
        related
            .value()
            .iter()
            .map(|edge| (edge.source().name(), edge.target().name()))
            .collect::<Vec<_>>(),
        [("Ada", "Bo"), ("Bo", "Ada")]
    );
    assert_eq!(listener.count(), 1);
}

#[test]
fn empty_invalid_and_cancelled_collections_do_not_send() {
    let backend = Backend::start().expect("backend");
    let engine = common::engine(&format!("{}/generic/v1", backend.origin()));
    let texts = Series::new("body".into(), Vec::<String>::new());
    let rank = Question::rank("Relevant?").expect("rank");
    let ranked = engine
        .rank_series(&rank, &texts, CallOptions::new())
        .expect("empty rank");
    assert_eq!(ranked.value().height(), 0);
    assert_eq!(ranked.facts().requests_sent(), 0);
    let find = Question::find("Which?").expect("find");
    assert_eq!(
        engine
            .find_series(&find, &texts, CallOptions::new())
            .expect_err("empty find")
            .kind(),
        ErrorKind::Usage
    );
    let token = thinkthen::CancelToken::new();
    token.cancel();
    assert_eq!(
        engine
            .rank_series(
                &rank,
                &common::column(&["one", "two"]),
                CallOptions::new().cancel(&token)
            )
            .expect_err("cancel")
            .kind(),
        ErrorKind::Cancelled
    );
    assert_eq!(backend.count(), 0);
}

#[expect(
    clippy::expect_used,
    reason = "a failed loopback fixture stops the consumer"
)]
fn collection_listener() -> Listener {
    Listener::answering(|body| {
        if String::from_utf8_lossy(body).contains("u001") {
            Canned::ok(r#"{"model":"jev-latest","answers":{"q1":{"type":"choice","probabilities":{"u001":0.1,"u002":0.8,"u003":0.05,"none":0.05}}},"usage":{"input_tokens":10,"output_tokens":2}}"#)
        } else {
            let probability = if String::from_utf8_lossy(body).contains("weak") { 0.2 } else { 0.9 };
            Canned::ok(&format!(r#"{{"model":"jev-latest","answers":{{"q1":{{"type":"noul","noul":{probability}}}}},"usage":{{"input_tokens":10,"output_tokens":2}}}}"#))
        }
    }).expect("listener")
}
