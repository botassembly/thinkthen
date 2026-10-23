#![cfg(test)]
//! OpenAI wire behavior through the public library.

use loopback::{KEY, Loopback, Reply};
use serde_json::{Value, json};
use thinkthen::{
    Adapter, Backend, ChoiceOption, Choose, Decide, Engine, Function, Record, Score, Tag,
};

fn backend(server: &Loopback) -> Backend {
    Backend::resolve_adapter(Adapter::OpenAi, Some(server.base()), None, |name| {
        (name == "OPENAI_API_KEY").then(|| KEY.to_owned())
    })
    .unwrap()
}

fn call(question: &impl Function, answers: Value) -> (Value, Vec<loopback::Request>) {
    let server = Loopback::start(move |_| Reply::json(200, &json!({"answers": answers}))).unwrap();
    let mut rows = Vec::new();
    let result = Engine::new(backend(&server)).call(
        question,
        [Record::text("Say \"now\"\nplease")],
        |row| {
            rows.push(serde_json::to_value(row).unwrap());
        },
    );
    assert_eq!(
        result.err().map(|error| error.kind().code()),
        rows.iter().find_map(|row| row["error"]["code"].as_str())
    );
    (rows.pop().unwrap(), server.requests())
}

#[test]
fn predicates_send_quoted_evidence_and_return_the_same_verdict() {
    let question = Decide::new("Is it urgent?").unwrap();
    let (row, requests) = call(
        &question,
        json!([{"type":"predicate","name":null,"probability":0.9}]),
    );
    assert_eq!(
        row,
        json!({"id":1,"status":"answered","answer":true,"p":0.9,
            "detail":{"source":{"question_identity":question.identity(),
                "backend":"openai","model":"gpt-6-luna"}}})
    );
    assert_eq!(requests[0].path, "/v1/decisions");
    assert_eq!(
        requests[0].header("authorization"),
        Some(format!("Bearer {KEY}").as_str())
    );
    assert_eq!(
        requests[0].json().unwrap(),
        json!({
            "model":"gpt-6-luna",
            "input":"Each question quotes the text it asks about.",
            "questions":[{"type":"predicate","instructions":"The text is \"Say \\\"now\\\"\\nplease\". Is it urgent?"}]
        })
    );
}

#[test]
fn choices_match_typed_values_and_preserve_descriptions() {
    let question = Choose::new(
        "Which?",
        [
            ChoiceOption::new("z").unwrap().description("First"),
            ChoiceOption::new("a").unwrap(),
        ],
    )
    .unwrap();
    let (row, requests) = call(
        &question,
        json!([{"type":"choice","probabilities":[
            {"value":"a","probability":0.8},{"value":"z","probability":0.2}
        ]}]),
    );
    assert_eq!(row["answer"], "a");
    assert_eq!(row["p"], 0.8);
    assert_eq!(
        requests[0].json().unwrap()["questions"][0]["choices"],
        json!([
            {"value":"z","description":"First"},{"value":"a"}
        ])
    );
}

#[test]
fn tags_judge_each_label_in_answer_order() {
    let question = Tag::new("What applies?", ["urgent", "refund"]).unwrap();
    let (row, requests) = call(
        &question,
        json!([
            {"type":"predicate","probability":0.9},{"type":"predicate","probability":0.1}
        ]),
    );
    assert_eq!(row["answer"], json!(["urgent"]));
    assert_eq!(
        requests[0].json().unwrap()["questions"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
}

#[test]
fn scores_use_level_probabilities_in_caller_order() {
    let question = Score::new("How urgent?", ["low", "high"]).unwrap();
    let (row, requests) = call(
        &question,
        json!([{"type":"score","score":99,"probabilities":[
            {"label":"high","value":1,"probability":0.8},{"label":"low","value":0,"probability":0.2}
        ]}]),
    );
    assert_eq!(row["answer"], 0.8);
    assert_eq!(row["detail"]["probabilities"], json!([0.2, 0.8]));
    assert_eq!(
        requests[0].json().unwrap()["questions"][0]["levels"],
        json!([
            {"label":"low"},{"label":"high"}
        ])
    );
}

#[test]
fn malformed_answers_fail_only_their_asks_without_exposing_reply_text() {
    let question = Decide::new("Is it urgent?").unwrap();
    for answers in [
        json!({"q1":{"type":"predicate","probability":0.9}}),
        json!([]),
        json!([{"type":"refusal","name":null}]),
        json!([{"type":"predicate","probability":1.1}]),
        json!([{"type":"predicate","probability":"secret"}]),
        json!([{"type":"choice","probabilities":[]}]),
    ] {
        let (row, _) = call(&question, answers);
        assert_eq!(row["status"], "failed");
        assert_eq!(row["error"]["code"], "backend.reply");
        assert!(!row.to_string().contains("secret"));
    }
    let server = Loopback::start(|_| {
        Reply::json(
            200,
            &json!({"answers":[
                {"type":"refusal"},{"type":"predicate","probability":0.9}
            ]}),
        )
    })
    .unwrap();
    let mut rows = Vec::new();
    Engine::new(backend(&server))
        .call(
            &question,
            [Record::text("one"), Record::text("two")],
            |row| {
                rows.push(serde_json::to_value(row).unwrap());
            },
        )
        .unwrap_err();
    assert_eq!(rows[0]["error"]["code"], "backend.reply");
    assert_eq!(rows[1]["answer"], true);
}

#[test]
fn distributions_reject_missing_duplicate_unknown_and_invalid_values() {
    let choice = Choose::new(
        "Which?",
        [
            ChoiceOption::new("a").unwrap(),
            ChoiceOption::new("b").unwrap(),
        ],
    )
    .unwrap();
    for probabilities in [
        json!([{"value":"a","probability":0.9}]),
        json!([{"value":"a","probability":0.5},{"value":"a","probability":0.5}]),
        json!([{"value":"a","probability":0.5},{"value":true,"probability":0.5}]),
        json!([{"value":"a","probability":-0.1},{"value":"b","probability":1.1}]),
    ] {
        let (row, _) = call(
            &choice,
            json!([{"type":"choice","probabilities":probabilities}]),
        );
        assert_eq!(row["error"]["code"], "backend.reply");
    }
    let score = Score::new("How urgent?", ["low", "high"]).unwrap();
    for probabilities in [
        json!([]),
        json!([{"label":"low","probability":0.5},{"label":"low","probability":0.5}]),
        json!([{"label":"low","probability":0.5},{"label":"other","probability":0.5}]),
        json!([{"label":"low","probability":-0.1},{"label":"high","probability":1.1}]),
    ] {
        let (row, _) = call(
            &score,
            json!([{"type":"score","probabilities":probabilities}]),
        );
        assert_eq!(row["error"]["code"], "backend.reply");
    }
}

#[test]
fn described_verdicts_use_boolean_choices() {
    let question = Decide::new("Is it urgent?").unwrap().yes("Now").no("Later");
    let (row, requests) = call(
        &question,
        json!([{"type":"choice","probabilities":[
            {"value":false,"probability":0.1},{"value":true,"probability":0.9}
        ]}]),
    );
    assert_eq!(row["answer"], true);
    assert_eq!(row["p"], 0.9);
    assert_eq!(
        requests[0].json().unwrap()["questions"][0]["choices"],
        json!([
            {"value":true,"description":"Now"},{"value":false,"description":"Later"}
        ])
    );
}

#[test]
fn adapter_selection_keeps_explicit_settings_and_rejects_missing_keys() {
    let server = Loopback::start(|_| {
        Reply::json(
            200,
            &json!({"answers":[
                {"type":"predicate","probability":0.9}
            ]}),
        )
    })
    .unwrap();
    let key = thinkthen::Key::new(KEY).unwrap();
    let backend = Backend::new(server.base(), key, "chosen-model").adapter(Adapter::OpenAi);
    assert!(!format!("{backend:?}").contains(KEY));
    Engine::new(backend)
        .call(
            &Decide::new("Is it?").unwrap(),
            [Record::text("text")],
            |_| {},
        )
        .unwrap();
    assert_eq!(
        server.requests()[0].json().unwrap()["model"],
        "chosen-model"
    );
    for value in [None, Some(" ")] {
        let error = Backend::resolve_adapter(Adapter::OpenAi, Some(server.base()), None, |_| {
            value.map(str::to_owned)
        })
        .unwrap_err();
        assert_eq!(error.kind().code(), "usage.key-missing");
    }
}

#[test]
fn a_choice_above_the_protocol_limit_fails_before_sending() {
    let server = Loopback::start(|_| panic!("an invalid choice must not send")).unwrap();
    let question = Choose::new(
        "Which?",
        (0..256).map(|i| ChoiceOption::new(i.to_string()).unwrap()),
    )
    .unwrap();
    let mut rows = Vec::new();
    Engine::new(backend(&server))
        .call(&question, [Record::text("text")], |row| {
            rows.push(serde_json::to_value(row).unwrap())
        })
        .unwrap_err();
    assert_eq!(rows[0]["error"]["code"], "backend.limit");
    assert_eq!(server.count(), 0);
}

#[test]
fn the_adapter_limit_closes_requests_between_records() {
    let question = Decide::new("Is it urgent?").unwrap();
    for count in [400, 401] {
        let server = Loopback::start(|request| {
            let count = request.json().unwrap()["questions"]
                .as_array()
                .unwrap()
                .len();
            Reply::json(
                200,
                &json!({"answers":vec![json!({"type":"predicate","probability":0.9}); count]}),
            )
        })
        .unwrap();
        let mut rows = Vec::new();
        Engine::new(backend(&server))
            .batch("500".parse().unwrap())
            .call(
                &question,
                (0..count).map(|i| Record::text(i.to_string())),
                |row| rows.push(serde_json::to_value(row).unwrap()),
            )
            .unwrap();
        assert_eq!(rows.len(), count);
        assert!(rows.iter().all(|row| row["answer"] == true));
        assert_eq!(server.count(), if count == 400 { 1 } else { 2 });
        assert!(server.requests().iter().all(|request| {
            request.json().unwrap()["questions"]
                .as_array()
                .unwrap()
                .len()
                <= 400
        }));
    }
}
