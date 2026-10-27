#![cfg(test)]
//! Result presentation preserves numeric probabilities and precise calculations.

#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

use loopback::{KEY, Loopback};
use serde_json::json;
use thinkthen::{Batch, Decide, Key, OffsetUnit, Record, Request, Settings};

#[test]
fn serialized_rows_preserve_every_probability_without_changing_answers() {
    let raw = 0.98999901;
    let span = 0.9421819849031113;
    let server = Loopback::answering(move |q| {
        let criteria = &q["criteria"];
        if criteria.get("BEGIN").is_some() && q["instructions"].as_str().unwrap().contains("[[meets]]") {
            json!({"type":"choice","probabilities":{"BEGIN":0.0,"INSIDE":0.0,"END":0.0,"SINGLE":0.0,"OUT":1.0}})
        } else if criteria.get("BEGIN").is_some() {
            json!({"type":"choice","probabilities":{
                "BEGIN":raw,"INSIDE":0.0,"END":0.0,"SINGLE":span,"OUT":1.0-span
            }})
        } else if criteria.get("person").is_some() {
            json!({"type":"choice","probabilities":{"person":raw,"none of these":1.0-raw}})
        } else if q["type"] == "score" {
            json!({"type":"score","probabilities":{"0":1.0-raw,"1":raw}})
        } else {
            json!({"type":"noul","noul":raw})
        }
    })
    .unwrap();
    let engine = Settings::default()
        .url(server.base())
        .api_key(Key::new(KEY).unwrap())
        .batch(Batch::default())
        .engine(|_| None)
        .unwrap();
    let decide = Decide::new("Ready?")
        .unwrap()
        .threshold("0.99".parse().unwrap());
    let row = engine.run(&decide, Record::text("Ada")).unwrap();
    assert_eq!(row.probability().unwrap().get(), raw);
    let row = serde_json::to_value(row).unwrap();
    assert_eq!(row["probability"], raw);
    assert_eq!(row["answer"], false);
    let request = Request::from_json(&json!({"question":{"function":"annotate","questions":{
        "decision":{"function":"decide","question":"Ready?","threshold":0.99},
        "names":{"function":"recognize","kinds":["person"],"evidence":"/name"},
        "relations":{"function":"relate","rules":[{"relation":"knows","source":"person","target":"person"}],"evidence":"/pair"},
        "score":{"function":"score","question":"How much?","levels":["low","high"]},
        "tags":{"function":"tag","question":"Which labels?","labels":["ready"]}
    }},"records":[{"id":42,"name":"Ada","pair":"Ada meets Bob"}]}).to_string()).unwrap();
    let call = engine
        .open_request(&request, OffsetUnit::CodePoints)
        .unwrap();
    call.sender().close();
    let rows = call.rows().collect::<Result<Vec<_>, _>>().unwrap();
    assert_eq!(rows.len(), 1);
    let row = serde_json::to_value(&rows[0]).unwrap();
    assert_eq!(row["id"], 42);
    let entries = &row["answer"];
    assert_eq!(entries["decision"]["probability"], raw);
    assert_eq!(entries["decision"]["answer"], false);
    assert_eq!(entries["names"]["answer"][0]["probability"], span * raw);
    assert_eq!(entries["names"]["answer"][0]["start"], 0);
    assert_eq!(entries["names"]["answer"][0]["end"], 3);
    assert_eq!(
        entries["names"]["detail"]["pieces"][0]["tags"]["BEGIN"],
        raw
    );
    assert_eq!(
        entries["names"]["detail"]["pieces"][0]["tags"]["SINGLE"],
        span
    );
    assert_eq!(entries["relations"]["answer"][0]["probability"], raw);
    assert_eq!(
        entries["relations"]["detail"]["pairs"][0]["probability"],
        raw
    );
    assert_eq!(entries["score"]["answer"], raw);
    assert_eq!(
        entries["score"]["detail"]["levels"],
        json!([1.0 - raw, raw])
    );
    assert_eq!(entries["tags"]["detail"]["labels"], json!({"ready":raw}));
    assert!(
        entries
            .as_object()
            .unwrap()
            .values()
            .all(|entry| entry["status"] == "answered")
    );
}
