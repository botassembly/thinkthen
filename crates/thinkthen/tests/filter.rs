#![cfg(test)]
//! Filter callers retain originals and every independent row outcome.
use loopback::{KEY, Loopback};
use serde_json::json;
use thinkthen::{Filter, Key, Record, Settings, Threshold};

#[test]
fn filter_pairs_every_original_with_its_row_and_keeps_only_answered_true() {
    let server = Loopback::answering(|question| {
        let text = question["instructions"].as_str().unwrap();
        let p = if text.contains("yes") {
            0.9
        } else if text.contains("maybe") {
            0.5
        } else if text.contains("broken") {
            2.0
        } else {
            0.1
        };
        json!({"type":"noul","noul":p})
    })
    .unwrap();
    let engine = Settings::default()
        .url(server.base())
        .api_key(Key::new(KEY).unwrap())
        .batch("2".parse().unwrap())
        .engine(|_| None)
        .unwrap();
    let question = Filter::new("Keep?")
        .unwrap()
        .threshold(Threshold::band(0.2, 0.8).unwrap());
    let original = ["yes first", "no", "maybe", "broken", "yes last", "invalid"];
    let rows = engine
        .filter_items(&question, original, |text| {
            if *text == "invalid" {
                Record::from_json("invalid")
            } else {
                Record::text(*text)
            }
        })
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(rows.len(), 6);
    assert_eq!(
        rows.iter().map(|(text, _)| *text).collect::<Vec<_>>(),
        original
    );
    assert_eq!(
        rows.iter()
            .filter(|(_, row)| row.kept())
            .map(|(text, _)| *text)
            .collect::<Vec<_>>(),
        ["yes first", "yes last"]
    );
    assert_eq!(rows[2].1.status(), thinkthen::Status::Unsure);
    assert_eq!(rows[3].1.error().unwrap().code().as_str(), "backend.reply");
    assert_eq!(rows[5].1.error().unwrap().code().as_str(), "record.json");
    assert_eq!(
        rows.iter()
            .filter(|(_, row)| row.kept())
            .map(|(_, row)| row.position())
            .collect::<Vec<_>>(),
        [1, 5]
    );
}
