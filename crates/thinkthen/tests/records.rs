#![cfg(test)]
//! `decide` over a stream of records through the library: ids, pointers,
//! batching, order and failures.

#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

use std::sync::{Arc, Condvar, Mutex, mpsc};
use std::thread;

use loopback::{KEY, Loopback, Reply, Request};
use serde_json::{Value, json};
use thinkthen::{Batch, Decide, Engine, Key, Pointer, Record, Settings};

const QUESTION: &str = "Is it a refund request?";

fn engine(server: &Loopback, batch: Option<usize>) -> Engine {
    let mut settings = Settings::default()
        .url(server.base())
        .api_key(Key::new(KEY).unwrap());
    if let Some(batch) = batch {
        settings = settings.batch(batch.to_string().parse().unwrap());
    }
    settings.engine(|_| None).unwrap()
}

fn run(engine: &Engine, records: impl IntoIterator<Item = Record>) -> Vec<Value> {
    engine
        .rows(
            &Decide::new(QUESTION).unwrap(),
            records.into_iter().collect::<Vec<_>>(),
        )
        .unwrap()
        .map(|row| serde_json::to_value(row.unwrap()).unwrap())
        .collect()
}

fn texts(count: usize) -> Vec<Record> {
    (1..=count)
        .map(|n| Record::text(format!("message {n}")))
        .collect()
}

/// Every wire question of `request`, in wire order.
fn questions(request: &Request) -> Vec<Value> {
    let body = request.json().unwrap();
    let questions = body["questions"].as_object().unwrap();
    (1..=questions.len())
        .map(|n| questions[&format!("q{n}")].clone())
        .collect()
}

fn instructions(evidence: &str) -> Value {
    json!(format!(
        "The text is {}. {QUESTION}",
        serde_json::to_string(evidence).unwrap()
    ))
}

fn ids(rows: &[Value]) -> Vec<Value> {
    rows.iter().map(|row| row["position"].clone()).collect()
}

fn positions(count: usize) -> Vec<Value> {
    (1..=count).map(|n| json!(n)).collect()
}

#[test]
fn records_go_out_in_the_requests_the_batch_size_predicts() {
    let cases = [
        (None, 6),
        (Some(1), 300),
        (Some(7), 43),
        (Some(50), 6),
        (Some(300), 1),
    ];
    for (batch, requests) in cases {
        let server = Loopback::noul(0.9).unwrap();
        let rows = run(&engine(&server, batch), texts(300));
        assert_eq!(ids(&rows), positions(300), "batch {batch:?}");
        assert!(rows.iter().all(|row| row["answer"] == true));
        assert_eq!(server.count(), requests, "batch {batch:?}");
        let most = batch.unwrap_or(50);
        for request in server.requests() {
            assert!(questions(&request).len() <= most, "batch {batch:?}");
        }
    }
}

#[test]
fn the_batch_never_changes_the_wire_questions() {
    let sent = |batch| {
        let server = Loopback::refunds().unwrap();
        let records = (1..=300).map(|n| {
            let topic = if n % 3 == 0 { "refund" } else { "parcel" };
            Record::text(format!("{topic} {n}"))
        });
        let rows = run(&engine(&server, Some(batch)), records);
        let mut wire: Vec<String> = server
            .requests()
            .iter()
            .flat_map(questions)
            .map(|question| question.to_string())
            .collect();
        wire.sort();
        (rows, wire)
    };
    let (alone, alone_wire) = sent(1);
    let (packed, packed_wire) = sent(50);
    assert_eq!(alone_wire.len(), 300);
    assert_eq!(alone_wire, packed_wire);
    assert_eq!(alone, packed);
}

#[test]
fn rows_keep_input_order_and_at_most_four_requests_wait_on_a_slow_first() {
    let (arrived, arrivals) = mpsc::channel();
    let release = Arc::new((Mutex::new(false), Condvar::new()));
    let held = Arc::clone(&release);
    let server = Loopback::start(move |request| {
        arrived.send(()).unwrap();
        let body = request.json().unwrap();
        let questions = body["questions"].as_object().unwrap();
        if questions.values().any(|question| {
            question["instructions"]
                .as_str()
                .unwrap()
                .contains("parcel first")
        }) {
            let (ready, wake) = &*held;
            drop(
                wake.wait_while(ready.lock().unwrap(), |ready| !*ready)
                    .unwrap(),
            );
        }
        let answers: serde_json::Map<String, Value> = questions
            .iter()
            .map(|(name, question)| {
                let yes = question["instructions"]
                    .as_str()
                    .unwrap()
                    .contains("text is \"refund");
                (
                    name.clone(),
                    json!({"type":"noul", "noul":if yes { 0.9 } else { 0.1 }}),
                )
            })
            .collect();
        Reply::json(200, &json!({"answers":answers}))
    })
    .unwrap();
    let records = (1..=400).map(|n| match n {
        1 => Record::text("parcel first"),
        n if n % 2 == 0 => Record::text(format!("refund {n}")),
        n => Record::text(format!("parcel {n}")),
    });
    let engine = engine(&server, Some(50));
    let rows = thread::scope(|scope| {
        let running = scope.spawn(|| run(&engine, records));
        for _ in 0..4 {
            arrivals.recv().unwrap();
        }
        assert_eq!(server.count(), 4);
        let (ready, wake) = &*release;
        *ready.lock().unwrap() = true;
        wake.notify_all();
        running.join().unwrap()
    });
    assert_eq!(server.count(), 8);
    assert_eq!(ids(&rows), positions(400));
    for (place, row) in rows.iter().enumerate() {
        assert_eq!(row["answer"], place % 2 == 1, "row {place}");
    }
}

#[test]
fn a_failure_fails_only_the_rows_it_reaches() {
    // Ten records at batch 5. The third mentions poison, so the reply to the
    // first request goes wrong in the way each case names.
    let cases: [(&str, &[u64], &str); 4] = [
        ("status", &[1, 2, 3, 4, 5], "backend.status"),
        ("garbled", &[1, 2, 3, 4, 5], "backend.reply"),
        ("missing", &[3], "backend.reply"),
        ("range", &[3], "backend.reply"),
    ];
    for (case, failed, code) in cases {
        let server = Loopback::start(move |request| {
            let body = request.json().unwrap();
            let questions = body["questions"].as_object().unwrap();
            let poisoned = |question: &Value| {
                question["instructions"]
                    .as_str()
                    .unwrap()
                    .contains("poison")
            };
            let hit = questions.values().any(poisoned);
            match case {
                "status" if hit => return Reply::json(503, &json!({})),
                "garbled" if hit => return Reply::json(200, &json!("not a reply")),
                _ => {}
            }
            let answers: serde_json::Map<String, Value> = questions
                .iter()
                .filter(|(_, question)| case != "missing" || !poisoned(question))
                .map(|(name, question)| {
                    let p = if case == "range" && poisoned(question) {
                        1.5
                    } else {
                        0.9
                    };
                    (name.clone(), json!({"type": "noul", "noul": p}))
                })
                .collect();
            Reply::json(200, &json!({"answers": answers}))
        })
        .unwrap();
        let records = (1..=10).map(|n| {
            Record::text(if n == 3 {
                "poison".to_owned()
            } else {
                format!("message {n}")
            })
        });
        let rows = run(&engine(&server, Some(5)), records);
        assert_eq!(ids(&rows), positions(10), "{case}");
        for (row, id) in rows.iter().zip(1..) {
            if failed.contains(&id) {
                assert_eq!(row["status"], "failed", "{case} row {id}");
                assert_eq!(row["error"]["code"], code, "{case} row {id}");
            } else {
                assert_eq!(row["status"], "answered", "{case} row {id}");
            }
        }
    }
}

/// Records whose texts have exactly `sizes` bytes, each distinct.
fn sized(sizes: &[usize]) -> impl Iterator<Item = Record> + '_ {
    (1..).zip(sizes).map(|(n, size)| {
        let mut text = format!("{n} ");
        text.push_str(&"x".repeat(size - text.len()));
        Record::text(text)
    })
}

/// The body bytes one request spends beyond the texts of `sizes`.
fn overhead(sizes: &[usize]) -> usize {
    let server = Loopback::noul(0.9).unwrap();
    run(&engine(&server, None), sized(sizes));
    server.requests()[0].body.len() - sizes.iter().sum::<usize>()
}

#[test]
fn the_byte_limit_closes_requests_and_fails_a_record_too_large_alone() {
    const LIMIT: usize = 96_000;
    let one = LIMIT - overhead(&[100]);
    let two = LIMIT - overhead(&[100, 100]);
    let (half, rest) = (two / 2, two - two / 2);
    // (text sizes, requests, rows too large, whether one body is exactly at the limit)
    let cases: [(Vec<usize>, usize, &[u64], bool); 7] = [
        (vec![40_000; 5], 3, &[], false),
        (vec![one], 1, &[], true),
        (vec![one + 1], 0, &[1], false),
        (vec![half, rest], 1, &[], true),
        (vec![half, rest + 1], 2, &[], false),
        (vec![10, 100_000, 10], 1, &[2], false),
        (vec![60_000, 60_000, 100_000, 10], 2, &[3], false),
    ];
    for (sizes, requests, too_large, exact) in cases {
        let server = Loopback::noul(0.9).unwrap();
        let rows = run(&engine(&server, None), sized(&sizes));
        assert_eq!(ids(&rows), positions(sizes.len()), "{sizes:?}");
        assert_eq!(server.count(), requests, "{sizes:?}");
        let bodies: Vec<usize> = server.requests().iter().map(|r| r.body.len()).collect();
        assert!(bodies.iter().all(|len| *len <= LIMIT), "{sizes:?}");
        assert_eq!(bodies.contains(&LIMIT), exact, "{sizes:?}");
        for (row, id) in rows.iter().zip(1..) {
            if too_large.contains(&id) {
                assert_eq!(
                    row["error"]["code"], "record.too-large",
                    "{sizes:?} row {id}"
                );
            } else {
                assert_eq!(row["status"], "answered", "{sizes:?} row {id}");
            }
        }
    }
}

#[test]
fn a_question_asked_twice_in_one_request_is_sent_once() {
    let cases = [(50, vec![2]), (1, vec![1, 1, 1])];
    for (batch, sent) in cases {
        let server = Loopback::refunds().unwrap();
        let records = ["refund", "parcel", "refund"].map(Record::text);
        let rows = run(&engine(&server, Some(batch)), records);
        let answers: Vec<&Value> = rows.iter().map(|row| &row["answer"]).collect();
        assert_eq!(answers, [&json!(true), &json!(false), &json!(true)]);
        let counts: Vec<usize> = server
            .requests()
            .iter()
            .map(|r| questions(r).len())
            .collect();
        assert_eq!(counts, sent, "batch {batch}");
    }
}

#[test]
fn a_json_record_gives_its_id_and_the_evidence_its_pointer_names() {
    // (record, pointer, id, the evidence sent, or the failure code)
    let cases: [(&str, &str, Value, Result<&str, &str>); 15] = [
        (
            r#"{"id":"a1","body":"refund"}"#,
            "/body",
            json!("a1"),
            Ok("refund"),
        ),
        (r#"{"body":"plain"}"#, "/body", json!(2), Ok("plain")),
        (r#"{"id":7,"body":"seven"}"#, "/body", json!(7), Ok("seven")),
        (
            r#"{"id":"o","body":{"k": 1}}"#,
            "/body",
            json!("o"),
            Ok(r#"{"k": 1}"#),
        ),
        (r#"{"a":{"b/c":["x","y"]}}"#, "/a/b~1c/1", json!(5), Ok("y")),
        (r#"{"a~b":"tilde"}"#, "/a~0b", json!(6), Ok("tilde")),
        (r#"{"body":12}"#, "/body", json!(7), Ok("12")),
        (r#"{"whole":true}"#, "", json!(8), Ok(r#"{"whole":true}"#)),
        (r#""just text""#, "", json!(9), Ok("just text")),
        (
            r#"{"body":"x"}"#,
            "/missing",
            json!(10),
            Err("usage.missing"),
        ),
        (
            r#"{"id":"m","body":"x"}"#,
            "/nope",
            json!("m"),
            Err("usage.missing"),
        ),
        (
            r#"{"a":["x","y"]}"#,
            "/a/01",
            json!(12),
            Err("usage.missing"),
        ),
        (r#"{"a":["x"]}"#, "/a/1", json!(13), Err("usage.missing")),
        ("not json", "/body", json!(14), Err("usage.record")),
        (
            r#"{"id":null,"body":"nameless"}"#,
            "/body",
            json!(15),
            Ok("nameless"),
        ),
    ];
    for (line, pointer, old_id, expected) in cases {
        let server = Loopback::noul(0.9).unwrap();
        let question = Decide::new(QUESTION)
            .unwrap()
            .evidence(pointer.parse().unwrap());
        let row = engine(&server, None)
            .run(&question, Record::from_json(line))
            .unwrap();
        let value = serde_json::to_value(&row).unwrap();
        assert_eq!(row.position(), 1);
        let input = serde_json::from_str::<Value>(line).ok();
        let supplied_id = input.as_ref().and_then(|value| value.get("id"));
        let expected_id = supplied_id.filter(|id| !id.is_null());
        assert_eq!(
            value.get("id"),
            expected_id,
            "{line} at {pointer}, former id {old_id}"
        );
        if supplied_id == Some(&Value::Null) {
            assert_eq!(row.error().unwrap().code().as_str(), "record.invalid");
            assert_eq!(server.count(), 0);
            continue;
        }
        match expected {
            Ok(evidence) => {
                assert_eq!(value["status"], "answered", "{line} at {pointer}");
                assert_eq!(server.count(), 1);
                assert_eq!(
                    questions(&server.requests()[0])[0]["instructions"],
                    instructions(evidence),
                    "{line}"
                );
            }
            Err(code) => {
                let code = match code {
                    "usage.missing" => "record.missing",
                    "usage.record" => "record.json",
                    other => other,
                };
                assert_eq!(value["error"]["code"], code, "{line} at {pointer}");
                assert_eq!(server.count(), 0);
            }
        }
    }
}

#[test]
fn a_pointer_outside_rfc_6901_is_refused() {
    for text in ["", "/", "/body", "/a~0b~1c", "/0"] {
        assert!(text.parse::<Pointer>().is_ok(), "{text}");
    }
    for text in ["body", "$.body", "#/id", "/a~", "/a~2"] {
        let error = text.parse::<Pointer>().unwrap_err();
        assert_eq!(error.code().as_str(), "usage.pointer", "{text}");
    }
}

#[test]
fn the_batch_flag_beats_the_variable_which_beats_the_default() {
    // (flag, variable, the batch chosen, or None when refused)
    let cases: [(Option<&str>, Option<&str>, Option<&str>); 11] = [
        (Some("7"), Some("9"), Some("7")),
        (None, Some("9"), Some("9")),
        (None, None, Some("50")),
        (Some("3"), Some("x"), Some("3")),
        (Some("0"), None, None),
        (Some("x"), None, None),
        (Some("+5"), None, None),
        (Some("-1"), None, None),
        (Some(""), None, None),
        (Some("99999999999999999999999"), None, None),
        (None, Some("0"), None),
    ];
    for (flag, variable, expected) in cases {
        let got = Batch::resolve(flag, |name| {
            (name == "THINKTHEN_BATCH")
                .then(|| variable.map(str::to_owned))
                .flatten()
        });
        match expected {
            Some(size) => assert_eq!(
                got.unwrap(),
                size.parse::<Batch>().unwrap(),
                "{flag:?} {variable:?}"
            ),
            None => assert_eq!(got.unwrap_err().code().as_str(), "usage.batch", "{flag:?}"),
        }
    }
}
