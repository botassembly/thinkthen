#![cfg(test)]
//! Finding lines through the public SDK.

use loopback::{KEY, Loopback};
use serde_json::{Value, json};
use thinkthen::{Backend, Engine, Find, Record, Toolbox};

fn engine(server: &Loopback) -> Engine {
    Engine::new(
        Backend::resolve(Some(server.base()), None, |name| {
            (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned())
        })
        .unwrap(),
    )
}

#[test]
fn finds_one_nonblank_line_with_scalar_offsets_and_settled_ties() {
    for (text, probabilities, threshold, answer, status, p) in [
        (
            "first\nsecond",
            json!({"u1":0.1,"u2":0.8,"none":0.1}),
            "0.5",
            json!({"text":"second","start":6,"end":12}),
            "answered",
            0.8,
        ),
        (
            "first\nsecond",
            json!({"u1":0.1,"u2":0.2,"none":0.7}),
            "0.5",
            Value::Null,
            "answered",
            0.7,
        ),
        (
            "first\nsecond",
            json!({"u1":0.4,"u2":0.3,"none":0.3}),
            "0.5",
            Value::Null,
            "unsure",
            0.4,
        ),
        (
            "first\nsecond",
            json!({"u1":0.5,"u2":0.5,"none":0.0}),
            "0.5",
            json!({"text":"first","start":0,"end":5}),
            "answered",
            0.5,
        ),
        (
            "first\nsecond",
            json!({"u1":0.5,"u2":0.0,"none":0.5}),
            "0.5",
            Value::Null,
            "answered",
            0.5,
        ),
        (
            "alone",
            json!({"u1":0.9,"none":0.1}),
            "0.5",
            json!({"text":"alone","start":0,"end":5}),
            "answered",
            0.9,
        ),
        (
            "é🦀\r\n \n 答🙂 \r\n",
            json!({"u1":0.1,"u2":0.9,"none":0.0}),
            "0.5",
            json!({"text":" 答🙂 ","start":6,"end":10}),
            "answered",
            0.9,
        ),
    ] {
        let expected_probabilities = probabilities.clone();
        let server =
            Loopback::answering(move |_| json!({"type":"choice","probabilities":probabilities}))
                .unwrap();
        let question = Find::new("Which line?")
            .unwrap()
            .threshold(threshold.parse().unwrap())
            .unwrap();
        let mut rows = Vec::new();
        engine(&server)
            .call(&question, [Record::text(text)], |row| {
                rows.push(serde_json::to_value(row).unwrap())
            })
            .unwrap();
        assert_eq!(rows[0]["status"], status);
        assert_eq!(rows[0]["answer"], answer);
        assert_eq!(rows[0]["p"], p);
        assert_eq!(rows[0]["detail"]["probabilities"], expected_probabilities);
        let request = server.requests()[0].json().unwrap();
        assert_eq!(
            request["questions"]["q1"]["criteria"]["none"],
            "No line answers the question."
        );
        assert_eq!(request["questions"].as_object().unwrap().len(), 1);
    }
}

#[test]
fn empty_and_oversized_texts_need_no_request_and_preserve_record_order() {
    let server = Loopback::noul(0.9).unwrap();
    let question = Find::new("Which?").unwrap();
    let mut rows = Vec::new();
    engine(&server)
        .call(
            &question,
            [
                Record::text(" \r\n\t\n"),
                Record::text("line\n".repeat(256)),
            ],
            |row| rows.push(serde_json::to_value(row).unwrap()),
        )
        .unwrap_err();
    assert_eq!(
        loopback::without_source(rows[0].clone()),
        json!({"id":1,"status":"answered","answer":null})
    );
    assert_eq!(rows[1]["error"]["code"], "backend.limit");
    assert_eq!(server.count(), 0);
}

#[test]
fn toolbox_find_retains_context_and_rejects_bands() {
    let server =
        Loopback::answering(|_| json!({"type":"choice","probabilities":{"u1":0.8,"none":0.2}}))
            .unwrap();
    let toolbox = Toolbox::from_bytes(br#"{"find":"Which?","threshold":"0.7"}"#).unwrap();
    engine(&server)
        .call(
            toolbox.question(None).unwrap(),
            [Record::text("answer")],
            |row| {
                assert_eq!(
                    serde_json::to_value(row).unwrap()["answer"],
                    json!({"text":"answer","start":0,"end":6})
                );
            },
        )
        .unwrap();
    assert!(
        Find::new("Which?")
            .unwrap()
            .threshold("0.2:0.8".parse().unwrap())
            .is_err()
    );
    assert!(Find::new(" ").is_err());
    assert!(Toolbox::from_bytes(br#"{"find":"Which?","options":["x"]}"#).is_err());
}

#[test]
fn option_limits_include_none_and_fail_only_the_oversized_record() {
    use thinkthen::Adapter;

    for (adapter, count, expected_requests) in [
        (Adapter::SystemOne, 254, 1),
        (Adapter::SystemOne, 255, 0),
        (Adapter::OpenAi, 255, 0),
    ] {
        let server = Loopback::answering(move |_| {
            let mut probabilities = serde_json::Map::new();
            for i in 1..=count {
                probabilities.insert(format!("u{i}"), json!(0.0));
            }
            probabilities.insert("none".to_owned(), json!(1.0));
            json!({"type":"choice","probabilities":probabilities})
        })
        .unwrap();
        let key_variable = match adapter {
            Adapter::SystemOne => "THINKTHEN_API_KEY",
            Adapter::OpenAi => "OPENAI_API_KEY",
        };
        let backend = Backend::resolve_adapter(adapter, Some(server.base()), None, |name| {
            (name == key_variable).then(|| KEY.to_owned())
        })
        .unwrap();
        let question = Find::new("Which?").unwrap();
        let mut rows = Vec::new();
        let result =
            Engine::new(backend).call(&question, [Record::text("line\n".repeat(count))], |row| {
                rows.push(serde_json::to_value(row).unwrap())
            });
        assert_eq!(result.is_err(), expected_requests == 0);
        assert_eq!(server.count(), expected_requests);
        if expected_requests == 0 {
            assert_eq!(rows[0]["error"]["code"], "backend.limit");
        } else {
            assert_eq!(rows[0]["status"], "answered");
            assert!(rows[0]["answer"].is_null());
            let request = server.requests()[0].json().unwrap();
            assert_eq!(
                request["questions"]["q1"]["criteria"]
                    .as_object()
                    .unwrap()
                    .len(),
                255
            );
            assert_eq!(rows[0]["detail"]["probabilities"]["none"], 1.0);
        }
    }
}

#[test]
fn direct_questions_select_an_answering_line_without_repeating_evidence() {
    let server = Loopback::answering(
        |_| json!({"type":"choice","probabilities":{"u1":0.9,"u2":0.1,"none":0.0}}),
    )
    .unwrap();
    let mut rows = Vec::new();
    engine(&server)
        .call(
            &Find::new("When does the invoice arrive?").unwrap(),
            [Record::text(
                "The invoice arrives Friday.\nThe meeting is Monday.",
            )],
            |row| rows.push(serde_json::to_value(row).unwrap()),
        )
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0]["answer"],
        json!({"text":"The invoice arrives Friday.","start":0,"end":27})
    );
    assert_eq!(server.count(), 1);
    let request = server.requests()[0].json().unwrap();
    let question = &request["questions"]["q1"];
    assert_eq!(
        question["instructions"],
        "Select the line that contains the answer to this question. Select none only if no line contains the answer.\n\nQuestion: When does the invoice arrive?"
    );
    assert_eq!(question["criteria"]["u1"], "The invoice arrives Friday.");
    assert_eq!(question["criteria"]["u2"], "The meeting is Monday.");
    let body = request.to_string();
    assert_eq!(body.matches("The invoice arrives Friday.").count(), 1);
    assert_eq!(body.matches("The meeting is Monday.").count(), 1);
}

#[test]
fn smaller_adapter_limit_leaves_room_for_none_and_preserves_other_records() {
    use loopback::Reply;
    use thinkthen::Adapter;

    let server = Loopback::start(|request| {
        let body = request.json().unwrap();
        assert_eq!(body["questions"].as_array().unwrap().len(), 1);
        let choices = body["questions"][0]["choices"].as_array().unwrap();
        assert_eq!(choices.len(), 255);
        assert_eq!(choices.last().unwrap()["value"], "none");
        let probabilities: Vec<_> = choices
            .iter()
            .map(|choice| {
                json!({"value":choice["value"],"probability":if choice["value"] == "none" {1.0} else {0.0}})
            })
            .collect();
        Reply::json(
            200,
            &json!({"answers":[{"type":"choice","probabilities":probabilities}]}),
        )
    })
    .unwrap();
    let backend = Backend::resolve_adapter(Adapter::OpenAi, Some(server.base()), None, |name| {
        (name == "OPENAI_API_KEY").then(|| KEY.to_owned())
    })
    .unwrap();
    let mut rows = Vec::new();
    Engine::new(backend)
        .call(
            &Find::new("Which?").unwrap(),
            [
                Record::text("line\n".repeat(255)),
                Record::text("line\n".repeat(254)),
            ],
            |row| rows.push(serde_json::to_value(row).unwrap()),
        )
        .unwrap_err();
    assert_eq!(rows[0]["error"]["code"], "backend.limit");
    assert_eq!(rows[1]["status"], "answered");
    assert!(rows[1]["answer"].is_null());
    assert_eq!(rows[1]["detail"]["probabilities"]["none"], 1.0);
    assert_eq!(server.count(), 1);
}

#[test]
fn a_malformed_find_reply_fails_the_record_without_a_partial_answer() {
    let server = Loopback::answering(|_| json!({"type":"noul", "noul":0.9})).unwrap();
    let mut rows = Vec::new();
    engine(&server)
        .call(
            &Find::new("Which?").unwrap(),
            [Record::text("first\nsecond")],
            |row| rows.push(serde_json::to_value(row).unwrap()),
        )
        .unwrap_err();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["status"], "failed");
    assert_eq!(rows[0]["error"]["code"], "backend.reply");
    assert!(rows[0].get("answer").is_none());
    assert_eq!(server.count(), 1);
}
