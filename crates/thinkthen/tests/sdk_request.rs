#![cfg(test)]
//! Reusable typed questions keep settings and cancellation in their calls.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

use loopback::{KEY, Loopback};
use serde_json::json;
use thinkthen::{
    Annotate, Batch, Cancel, Decide, Key, OffsetUnit, Rank, Record, Request, Settings,
};
#[test]
fn questions_reuse_engine_settings_and_cancel_only_the_selected_call() {
    let server = Loopback::noul(0.9).unwrap();
    let engine = Settings::default()
        .url(server.base())
        .api_key(Key::new(KEY).unwrap())
        .batch(Batch::try_from(1_u64).unwrap())
        .model("client-model")
        .engine(|_| None)
        .unwrap();
    let question = Decide::new("Ready?")
        .unwrap()
        .evidence("/body".parse().unwrap())
        .threshold("0.95".parse().unwrap());
    let cancel = Cancel::default();
    cancel.cancel();
    let stopped = engine
        .clone()
        .cancel(cancel)
        .rows(&question, [Record::text("evidence")])
        .unwrap_err();
    assert_eq!(stopped.code().as_str(), "local.cancelled");
    assert_eq!(server.count(), 0);
    let row = engine
        .run(
            &question,
            Record::json(json!({"id":"original","body":"evidence"})),
        )
        .unwrap();
    assert_eq!(row.id().unwrap().as_str(), Some("original"));
    assert_eq!(row.into_result().unwrap(), Some(false));
    assert_eq!(
        server.requests()[0].json().unwrap()["model"],
        "client-model"
    );
    let annotate = Annotate::new(std::collections::BTreeMap::from([(
        "ready".into(),
        Decide::new("Ready?").unwrap().into(),
    )]))
    .unwrap()
    .evidence("/body".parse().unwrap());
    let row = engine
        .run(&annotate, Record::json(json!({"body":"evidence"})))
        .unwrap();
    assert_eq!(
        serde_json::to_value(row).unwrap()["answer"]["ready"]["answer"],
        true
    );
    assert_eq!(server.count(), 2);
}
#[test]
fn rank_levels_and_top_retain_original_positions() {
    let server = Loopback::answering(|question| {
        assert_eq!(question["type"], "score");
        assert_eq!(question["criteria"], json!(["low", "high"]));
        let p = usize::from(
            question["instructions"]
                .as_str()
                .unwrap()
                .contains("higher"),
        );
        json!({"type":"score","probabilities":{"0":1-p,"1":p}})
    })
    .unwrap();
    let engine = Settings::default()
        .url(server.base())
        .api_key(Key::new(KEY).unwrap())
        .batch(Batch::default())
        .engine(|_| None)
        .unwrap();
    let question = Rank::new("How urgent?", ["low", "high"])
        .unwrap()
        .top(std::num::NonZeroUsize::new(1).unwrap())
        .unwrap();
    let rows = engine
        .rows(&question, [Record::text("lower"), Record::text("higher")])
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].position(), 2);
    assert_eq!(serde_json::to_value(&rows[0]).unwrap()["answer"], 1);
    assert_eq!(server.count(), 1);
}
#[test]
fn initial_request_records_precede_pushed_records() {
    let engine = Settings::from_json(r#"{"backend":"stub"}"#)
        .unwrap()
        .engine(|_| None)
        .unwrap();
    let request =
        Request::new(Decide::new("Ready?").unwrap()).with_records([json!({"id":"initial"})]);
    let call = engine
        .open_request(&request, OffsetUnit::CodePoints)
        .unwrap();
    let sender = call.sender();
    sender
        .send(Record::text("next").with_id("pushed".into()))
        .unwrap();
    sender.close();
    let rows = call.rows().collect::<Result<Vec<_>, _>>().unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(
        rows.iter()
            .map(|row| row.id().unwrap().as_str().unwrap())
            .collect::<Vec<_>>(),
        ["initial", "pushed"]
    );
    assert_eq!(
        rows.iter()
            .map(thinkthen::AnyRow::position)
            .collect::<Vec<_>>(),
        [1, 2]
    );
}
