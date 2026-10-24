#![cfg(test)]
//! Independent labels through the public Rust API.

use loopback::{KEY, Loopback, Reply};
use serde_json::json;
use thinkthen::{Backend, Batch, Engine, Pointer, Record, Tag, Threshold};

fn engine(server: &Loopback) -> Engine {
    let backend = Backend::resolve(Some(server.base()), None, |name| {
        (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned())
    })
    .unwrap();
    Engine::new(backend)
}

#[test]
fn independent_labels_keep_their_probability_and_input_order() {
    for (probabilities, expected) in [
        ([0.1, 0.69, 0.0], vec![]),
        ([0.7, 0.69, 0.9], vec!["billing", "shipping"]),
        ([0.7, 0.8, 1.0], vec!["billing", "refund", "shipping"]),
    ] {
        let server = Loopback::answering(move |question| {
            assert_eq!(question["type"], "noul");
            let instructions = question["instructions"].as_str().unwrap();
            assert!(instructions.starts_with("The text is \"a \\\"quoted\\\" message\"."));
            let position = ["billing", "refund", "shipping"]
                .iter()
                .position(|label| {
                    instructions.ends_with(&format!("Does the label \"{label}\" apply?"))
                })
                .unwrap();
            json!({"type": "noul", "noul": probabilities[position]})
        })
        .unwrap();
        let question = Tag::new("What applies?", ["billing", "refund", "shipping"])
            .unwrap()
            .threshold("0.7".parse().unwrap())
            .unwrap();
        let records = (0..3).map(|i| {
            Record::json(
                &json!({"id": format!("r{i}"), "body": "a \"quoted\" message"}).to_string(),
                &"/body".parse::<Pointer>().unwrap(),
            )
        });
        let mut rows = Vec::new();
        engine(&server)
            .batch("2".parse::<Batch>().unwrap())
            .call(&question, records, |row| {
                rows.push(serde_json::to_value(row).unwrap())
            })
            .unwrap();
        assert_eq!(rows.len(), 3);
        for (i, row) in rows.iter().enumerate() {
            assert_eq!(
                &loopback::without_source(row.clone()),
                &json!({"id": format!("r{i}"), "status": "answered", "answer": expected,
                "detail": {"billing": probabilities[0], "refund": probabilities[1], "shipping": probabilities[2]}})
            );
        }
        assert_eq!(server.count(), 2);
        for request in server.requests() {
            assert_eq!(
                request.json().unwrap()["questions"]
                    .as_object()
                    .unwrap()
                    .len(),
                3
            );
        }
    }
}

#[test]
fn invalid_questions_and_threshold_bands_are_refused_at_construction() {
    for (text, labels) in [
        (" ", vec!["billing"]),
        ("What?", vec![]),
        ("What?", vec![" "]),
        ("What?", vec!["billing", "billing"]),
    ] {
        assert_eq!(
            Tag::new(text, labels).unwrap_err().kind().code(),
            "usage.question"
        );
    }
    let error = Tag::new("What?", ["billing"])
        .unwrap()
        .threshold("0.2:0.8".parse::<Threshold>().unwrap())
        .unwrap_err();
    assert_eq!(error.kind().code(), "usage.threshold");
}

#[test]
fn one_missing_label_or_failed_record_fails_the_whole_row() {
    let server = Loopback::start(|_| {
        Reply::json(
            200,
            &json!({"answers": {"q1": {"type": "noul", "noul": 0.9}}}),
        )
    })
    .unwrap();
    let question = Tag::new("What?", ["billing", "shipping"]).unwrap();
    let records = [
        Record::text("text"),
        Record::json("bad json", &Pointer::default()),
        Record::text("x".repeat(96_000)),
    ];
    let mut rows = Vec::new();
    engine(&server)
        .call(&question, records, |row| {
            rows.push(serde_json::to_value(row).unwrap())
        })
        .unwrap_err();
    assert_eq!(rows.len(), 3);
    for (row, code) in rows
        .iter()
        .zip(["backend.reply", "usage.record", "backend.limit"])
    {
        assert_eq!(row["status"], "failed");
        assert_eq!(row["error"]["code"], code);
        assert!(row.get("answer").is_none());
    }
    assert_eq!(server.count(), 1);
}

#[test]
fn multi_ask_records_span_requests_at_the_byte_limit() {
    let question = Tag::new("What?", ["shipping", "billing"]).unwrap();
    // Measure the wire overhead rather than copying the request encoding.
    let server = Loopback::noul(0.9).unwrap();
    engine(&server)
        .call(&question, [Record::text("x".repeat(100))], |_| {})
        .unwrap();
    let overhead = server.requests()[0].body.len() - 200;
    let fitting = (96_000 - overhead) / 2;
    for (sizes, expected, requests) in [
        (vec![fitting], vec!["answered"], 1),
        (vec![fitting + 1], vec!["answered"], 2),
        (vec![23_000, 23_001], vec!["answered", "answered"], 1),
        (vec![24_000, 24_001], vec!["answered", "answered"], 2),
    ] {
        let server = Loopback::noul(0.9).unwrap();
        let mut rows = Vec::new();
        engine(&server)
            .call(
                &question,
                sizes
                    .iter()
                    .enumerate()
                    .map(|(i, size)| Record::text(format!("{i}{}", "x".repeat(size - 1)))),
                |row| rows.push(serde_json::to_value(row).unwrap()),
            )
            .unwrap();
        let statuses: Vec<_> = rows
            .iter()
            .map(|row| row["status"].as_str().unwrap())
            .collect();
        assert_eq!(statuses, expected, "{sizes:?}");
        assert_eq!(server.count(), requests, "{sizes:?}");
        assert!(
            server
                .requests()
                .iter()
                .all(|request| request.body.len() <= 96_000)
        );
        for row in rows {
            assert_eq!(row["answer"], json!(["shipping", "billing"]));
        }
    }
}
