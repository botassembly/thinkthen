#![cfg(test)]
//! Result presentation preserves numeric probabilities and precise calculations.

use loopback::{KEY, Loopback};
use serde_json::json;
use thinkthen::{Annotate, Backend, Decide, Engine, Outcome, Record, Toolbox};

#[test]
fn serialized_rows_round_every_probability_without_changing_answers() {
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
    let backend = Backend::resolve(Some(server.base()), None, |name| {
        (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned())
    })
    .unwrap();
    let engine = Engine::new(backend);
    let decide = Decide::new("Ready?")
        .unwrap()
        .threshold("0.99".parse().unwrap());
    engine
        .call(&decide, [Record::text("Ada")], |row| {
            let Outcome::Answered { p: Some(p), .. } = row.outcome() else {
                panic!("expected a decision");
            };
            assert_eq!(p.get(), raw);
            let row = serde_json::to_value(row).unwrap();
            assert_eq!(row["p"], 0.99);
            assert_eq!(row["answer"], false);
        })
        .unwrap();
    let toolbox = Toolbox::from_bytes(
        br#"{"questions":{
            "decision":{"decide":"Ready?","threshold":"0.99"},
            "names":{"recognize":[],"on":"/name"},
            "relations":{"relate":[{"name":"knows","source":"person","target":"person"}],"on":"/pair"},
            "score":{"score":"How much?","levels":["low","high"]},
            "tags":{"tag":"Which labels?","labels":["ready"]}
        }}"#,
    )
    .unwrap();
    let mut rows = Vec::new();
    engine
        .annotate(
            &Annotate::new(&toolbox),
            [Record::json(
                r#"{"id":42,"name":"Ada","pair":"Ada meets Bob"}"#,
                &Default::default(),
            )],
            |row| rows.push(serde_json::to_value(row).unwrap()),
        )
        .unwrap();
    let row = &rows[0];
    assert_eq!(row["id"], 42);
    let entries = &row["answer"];
    assert_eq!(entries["decision"]["p"], 0.99);
    assert_eq!(entries["decision"]["answer"], false);
    assert_eq!(entries["names"]["answer"][0]["p"], 0.94);
    assert_eq!(entries["names"]["answer"][0]["start"], 0);
    assert_eq!(entries["names"]["answer"][0]["end"], 3);
    assert_eq!(
        entries["names"]["detail"]["pieces"][0]["tags"]["BEGIN"],
        0.99
    );
    assert_eq!(
        entries["names"]["detail"]["pieces"][0]["tags"]["SINGLE"],
        0.94
    );
    assert_eq!(entries["relations"]["answer"][0]["p"], 0.99);
    assert_eq!(entries["relations"]["detail"][0]["p"], 0.99);
    assert_eq!(entries["score"]["answer"], raw);
    assert_eq!(entries["score"]["detail"], json!([0.01, 0.99]));
    assert_eq!(entries["tags"]["detail"], json!({"ready":0.99}));
    assert!(
        entries
            .as_object()
            .unwrap()
            .values()
            .all(|entry| entry["status"] == "answered")
    );
}
