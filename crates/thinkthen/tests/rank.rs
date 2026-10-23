#![cfg(test)]
//! Ranking through the public Rust API.

use loopback::{KEY, Loopback};
use serde_json::{Value, json};
use thinkthen::{Backend, Decide, Engine, Rank, RankLimit, Record, Score};

#[test]
fn rank_orders_both_question_kinds_stably_with_failures_last_and_top_limits() {
    for scale in [false, true] {
        for top in [None, Some(1), Some(2), Some(20)] {
            let server = Loopback::answering(move |q| {
                let text = q["instructions"].as_str().unwrap();
                if text.contains("broken") {
                    return json!({});
                }
                let p = if text.contains("low") {
                    0.1
                } else if text.contains("middle") {
                    0.5
                } else {
                    0.9
                };
                if scale {
                    json!({"type":"score", "probabilities":{"0": 1.0-p, "1":p}})
                } else {
                    json!({"type":"noul", "noul":p})
                }
            })
            .unwrap();
            let backend = Backend::resolve(Some(server.base()), None, |name| {
                (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned())
            })
            .unwrap();
            let question = if scale {
                Rank::Score(Score::new("Priority?", ["low", "high"]).unwrap())
            } else {
                Rank::Decide(
                    Decide::new("Priority?")
                        .unwrap()
                        .threshold("0.2:0.8".parse().unwrap()),
                )
            };
            let records = [
                "broken",
                "low",
                "high-a",
                "middle",
                "high-b",
                "broken-again",
            ]
            .map(|text| {
                (
                    Record::json(
                        &json!({"id":text,"body":text}).to_string(),
                        &"/body".parse().unwrap(),
                    ),
                    text,
                )
            });
            let mut rows = Vec::new();
            let failure = Engine::new(backend).batch("1".parse().unwrap()).rank(
                &question,
                records,
                top.map(RankLimit::new).transpose().unwrap(),
                |original, row| {
                    rows.push((original, serde_json::to_value(row).unwrap()));
                },
            );
            assert_eq!(failure.unwrap_err().kind().code(), "backend.reply");
            let expected = [
                "high-a",
                "high-b",
                "middle",
                "low",
                "broken",
                "broken-again",
            ];
            let length = top.unwrap_or(expected.len()).min(expected.len());
            assert_eq!(
                rows.iter()
                    .map(|(original, _)| *original)
                    .collect::<Vec<_>>(),
                expected[..length]
            );
            for (index, (original, row)) in rows.iter().enumerate() {
                assert_eq!(row["id"], *original);
                if index < 4 {
                    assert_eq!(row["answer"], index + 1);
                    assert_eq!(row["status"], "answered");
                    assert!(row.get("p").is_none());
                } else {
                    assert_eq!(row["status"], "failed");
                    assert_eq!(row["error"]["code"], "backend.reply");
                }
            }
            assert_eq!(server.count(), 6);
        }
    }
}

#[test]
fn empty_rank_sends_nothing() {
    let server = Loopback::noul(0.9).unwrap();
    let backend = Backend::resolve(Some(server.base()), None, |_| Some(KEY.to_owned())).unwrap();
    let failure = Engine::new(backend).rank(
        &Rank::Decide(Decide::new("Priority?").unwrap()),
        Vec::<(Record, Value)>::new(),
        None,
        |_, _| panic!("no records"),
    );
    failure.unwrap();
    assert_eq!(server.count(), 0);
}

#[test]
fn top_rank_releases_discarded_originals_while_reading_input() {
    use std::cell::Cell;
    use std::rc::Rc;
    struct Original(Rc<Cell<usize>>);
    impl Drop for Original {
        fn drop(&mut self) {
            self.0.set(self.0.get() - 1);
        }
    }
    let alive = Rc::new(Cell::new(0));
    let peak = Cell::new(0);
    let server = Loopback::noul(0.9).unwrap();
    let backend = Backend::resolve(Some(server.base()), None, |_| Some(KEY.to_owned())).unwrap();
    let records = (0..64).map(|_| {
        alive.set(alive.get() + 1);
        peak.set(peak.get().max(alive.get()));
        (Record::text("record"), Original(alive.clone()))
    });
    let failure = Engine::new(backend).batch("1".parse().unwrap()).rank(
        &Rank::Decide(Decide::new("Priority?").unwrap()),
        records,
        Some(RankLimit::new(2).unwrap()),
        |_, _| {},
    );
    failure.unwrap();
    assert!(
        peak.get() < 20,
        "top retains discarded originals: {}",
        peak.get()
    );
    assert_eq!(alive.get(), 0);
}

#[test]
fn cancelled_rank_does_not_read_input() {
    let server = Loopback::noul(0.9).unwrap();
    let backend = Backend::resolve(Some(server.base()), None, |_| Some(KEY.to_owned())).unwrap();
    let cancel = thinkthen::Cancel::default();
    cancel.cancel();
    let read = std::cell::Cell::new(0);
    let records = (0..3).map(|_| {
        read.set(read.get() + 1);
        (Record::text("record"), ())
    });
    let failure = Engine::new(backend).cancel(cancel).rank(
        &Rank::Decide(Decide::new("Priority?").unwrap()),
        records,
        None,
        |_, _| panic!("cancelled"),
    );
    assert_eq!(failure.unwrap_err().kind().code(), "local.cancelled");
    assert_eq!(read.get(), 0);
    assert_eq!(server.count(), 0);
}

#[test]
fn rank_keeps_signed_zero_probabilities_tied_below_positive_answers() {
    for top in [None, Some(2)] {
        let server = Loopback::answering(|q| {
            let text = q["instructions"].as_str().unwrap();
            let p = if text.contains("negative-zero") {
                -0.0
            } else if text.contains("positive-zero") {
                0.0
            } else {
                0.1
            };
            json!({"type":"noul", "noul":p})
        })
        .unwrap();
        let backend =
            Backend::resolve(Some(server.base()), None, |_| Some(KEY.to_owned())).unwrap();
        let records =
            ["negative-zero", "positive-zero", "positive"].map(|text| (Record::text(text), text));
        let mut ordered = Vec::new();
        let failure = Engine::new(backend).rank(
            &Rank::Decide(Decide::new("Priority?").unwrap()),
            records,
            top.map(RankLimit::new).transpose().unwrap(),
            |original, _| ordered.push(original),
        );
        failure.unwrap();
        let expected = ["positive", "negative-zero", "positive-zero"];
        assert_eq!(ordered, expected[..top.unwrap_or(3)]);
    }
}

#[test]
fn zero_top_is_refused_at_rust_and_json_admission_without_sending() {
    let server = Loopback::noul(0.9).unwrap();
    let backend = Backend::resolve(Some(server.base()), None, |_| Some(KEY.to_owned())).unwrap();
    let engine = Engine::new(backend);
    let rust = RankLimit::new(0)
        .and_then(|top| {
            engine.rank(
                &Rank::Decide(Decide::new("Priority?").unwrap()),
                [(Record::text("evidence"), ())],
                Some(top),
                |_, _| panic!("invalid limit produced a row"),
            )
        })
        .unwrap_err();
    let client = thinkthen::Client::from_json(&json!({"url":server.base()}).to_string(), |name| {
        (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned())
    })
    .unwrap();
    let json = thinkthen::SdkRequest::from_json(
        r#"{"function":"rank","question":{"type":"inline","value":"Priority?"},"options":{"top":0},"records":["evidence"]}"#,
    )
    .and_then(|request| {
        request.execute(&client, thinkthen::Cancel::default(), |_| {
            panic!("invalid limit produced a row")
        })
    })
    .unwrap_err();
    assert_eq!(rust, json);
    assert_eq!(rust.kind().code(), "usage.question");
    assert_eq!(server.count(), 0);
}
