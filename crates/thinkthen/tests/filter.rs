#![cfg(test)]
//! Filtering retains successful records before one aggregate failure.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;
use loopback::{KEY, Loopback};
use serde_json::json;
use thinkthen::{Cut, Filter, Key, Record, Settings};

#[derive(Debug, PartialEq)]
struct Text(&'static str);
impl From<&Text> for Record {
    fn from(text: &Text) -> Self {
        if text.0 == "invalid" {
            Self::from_json("invalid")
        } else {
            Self::text(text.0)
        }
    }
}
#[test]
fn filter_keeps_successes_then_reports_every_failed_original() {
    let server = Loopback::answering(|question| {
        let text = question["instructions"].as_str().unwrap();
        let p = if text.contains("yes") {
            0.9
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
        .threshold(Cut::new(0.8).unwrap());
    let mut kept = engine
        .filter(
            &question,
            ["yes first", "no", "broken", "yes last", "invalid"].map(Text),
        )
        .unwrap();
    assert_eq!(kept.next().unwrap().unwrap().0, "yes first");
    assert_eq!(kept.next().unwrap().unwrap().0, "yes last");
    let error = kept.next().unwrap().unwrap_err();
    assert!(error.to_string().starts_with(
        "2 of 5 records failed; the first, at position 3, failed with backend.reply:"
    ));
    assert_eq!(
        error
            .rows()
            .iter()
            .map(|row| (
                row.item().0,
                row.position(),
                row.error().unwrap().code().as_str()
            ))
            .collect::<Vec<_>>(),
        [
            ("broken", 3, "backend.reply"),
            ("invalid", 5, "record.json")
        ]
    );
    assert!(kept.next().is_none());
    assert!(kept.next().is_none());
}

#[test]
fn a_saved_filter_band_is_refused_before_execution() {
    let error = serde_json::from_value::<Filter>(
        json!({"question":"Keep?", "threshold":{"low":0.2,"high":0.8}}),
    )
    .unwrap_err();
    assert!(error.to_string().contains("decide"));
}
