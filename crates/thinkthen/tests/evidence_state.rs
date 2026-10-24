#![cfg(test)]
//! Selected name and relation evidence survives packing and dependent rounds.

use loopback::{KEY, Loopback, Reply};
use serde_json::{Value, json};
use thinkthen::{
    Adapter, Annotate, Backend, Engine, Recognize, Record, Relate, RelationRule, Toolbox,
};

fn questions(body: &Value) -> Vec<&Value> {
    match &body["questions"] {
        Value::Object(items) => items.values().collect(),
        Value::Array(items) => items.iter().collect(),
        _ => panic!("wire questions"),
    }
}

fn labels(question: &Value) -> Vec<&str> {
    if let Some(criteria) = question["criteria"].as_object() {
        criteria.keys().map(String::as_str).collect()
    } else {
        question["choices"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|choice| choice["value"].as_str().unwrap())
            .collect()
    }
}

fn reply(question: &Value, adapter: Adapter) -> Value {
    let words = question["instructions"].as_str().unwrap();
    let labels = labels(question);
    if labels.is_empty() {
        return match adapter {
            Adapter::SystemOne => json!({"type":"noul", "noul":0.9}),
            Adapter::OpenAi => json!({"type":"predicate", "probability":0.9}),
        };
    }
    let boundary = labels.contains(&"BEGIN");
    let named = ["Ada", "Bea", "Zoë", "Acme", "Orbit"]
        .iter()
        .any(|name| words.contains(&format!("[[{name}]]")));
    let company = ["Acme", "Orbit"]
        .iter()
        .any(|name| words.contains(&format!("[[{name}]]")));
    let chosen = if boundary {
        if named { "SINGLE" } else { "OUT" }
    } else if company {
        "company"
    } else {
        "person"
    };
    match adapter {
        Adapter::SystemOne => {
            let probabilities: serde_json::Map<String, Value> = labels
                .iter()
                .map(|label| ((*label).to_owned(), json!(f64::from(*label == chosen))))
                .collect();
            json!({"type":"choice", "probabilities":probabilities})
        }
        Adapter::OpenAi => {
            let probabilities: Vec<_> = labels
                .iter()
                .map(|label| json!({"value":label,"probability":f64::from(*label == chosen)}))
                .collect();
            json!({"type":"choice", "probabilities":probabilities})
        }
    }
}

fn server(adapter: Adapter) -> Loopback {
    Loopback::start(move |request| {
        let body = request.json().unwrap();
        let questions = questions(&body);
        if questions.len() == 3 {
            return Reply::json(413, &json!({}));
        }
        let answers = match adapter {
            Adapter::SystemOne => Value::Object(
                body["questions"]
                    .as_object()
                    .unwrap()
                    .iter()
                    .map(|(key, question)| (key.clone(), reply(question, adapter)))
                    .collect(),
            ),
            Adapter::OpenAi => Value::Array(
                questions
                    .iter()
                    .map(|question| reply(question, adapter))
                    .collect(),
            ),
        };
        Reply::json(200, &json!({"answers":answers}))
    })
    .unwrap()
}

fn engine(server: &Loopback, adapter: Adapter) -> Engine {
    Engine::new(
        Backend::resolve_adapter(adapter, Some(server.base()), None, |_| Some(KEY.to_owned()))
            .unwrap(),
    )
}

fn check_wording(question: &Value, round: usize) {
    let words = question["instructions"].as_str().unwrap();
    let required = match round {
        0 => &[
            "[[ ]]",
            "position",
            "one of these kinds",
            "other names",
            "ordinary words",
            "dates",
            "numbers",
            "marks",
            "OUT",
            "spelling",
        ][..],
        1 => &[
            "refer to in this text",
            "listed kind of name",
            "none of these",
            "not a proper name",
            "what they name",
        ][..],
        _ => return,
    };
    let lower = words.to_lowercase();
    for term in required {
        assert!(
            lower.contains(&term.to_lowercase()),
            "missing {term}: {words}"
        );
    }
    if round == 0 {
        for (tag, position) in [
            ("BEGIN", "first piece"),
            ("INSIDE", "middle piece"),
            ("END", "last piece"),
            ("SINGLE", "one-piece"),
            ("OUT", "category description"),
        ] {
            let description = if question["criteria"].is_object() {
                &question["criteria"][tag]
            } else {
                &question["choices"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|choice| choice["value"] == tag)
                    .unwrap()["description"]
            }
            .as_str()
            .unwrap();
            assert!(description.to_lowercase().contains(position));
            assert!(description.contains("proper name"));
            if tag == "OUT" {
                assert!(description.contains("even when it matches a listed kind"));
            } else {
                assert!(description.contains("identifying a particular entity"));
            }
        }
    }
    if round == 1 {
        let decline = if question["criteria"].is_object() {
            &question["criteria"]["none of these"]
        } else {
            &question["choices"]
                .as_array()
                .unwrap()
                .iter()
                .find(|choice| choice["value"] == "none of these")
                .unwrap()["description"]
        }
        .as_str()
        .unwrap();
        assert!(decline.contains("not a proper name"));
        assert!(decline.contains("listed kinds"));
    }
}

fn check_states(server: &Loopback, adapter: Adapter, expected: &[(&str, [usize; 3])]) {
    let mut counts = vec![[0; 3]; expected.len()];
    for request in server.requests() {
        let body = request.json().unwrap();
        let state = body[match adapter {
            Adapter::SystemOne => "state",
            Adapter::OpenAi => "input",
        }]
        .as_str()
        .unwrap();
        let at = expected
            .iter()
            .position(|(text, _)| *text == state)
            .unwrap();
        let items = questions(&body);
        if items.len() == 3 {
            continue;
        } // Refused requests are checked for state too.
        for question in items {
            let words = question["instructions"].as_str().unwrap();
            let labels = labels(question);
            let round = if labels.contains(&"BEGIN") {
                0
            } else if labels.is_empty() {
                2
            } else {
                1
            };
            counts[at][round] += 1;
            check_wording(question, round);
            if round < 2 {
                let marked = words
                    .split_once("[[")
                    .unwrap()
                    .1
                    .split_once("]]")
                    .unwrap()
                    .0;
                assert!(
                    state.contains(marked),
                    "question from another record: {words}"
                );
            } else if state != "Each question quotes the text it asks about." {
                assert!(words.starts_with(&format!(
                    "The text is {}. Does the text itself state that ",
                    serde_json::to_string(state).unwrap()
                )));
                assert!(words.contains("works for"));
            }
        }
    }
    for (actual, (_, expected)) in counts.iter().zip(expected) {
        assert_eq!(actual, expected);
    }
}

#[test]
fn selected_text_is_shared_evidence_for_both_adapters_and_all_name_rounds() {
    for adapter in [Adapter::SystemOne, Adapter::OpenAi] {
        for relations in [false, true] {
            let server = server(adapter);
            let mut rows = Vec::new();
            let records = ["Ada joined Acme", "Zoë joined Orbit"].map(|text| {
                Record::json(
                    &json!({"ignored":"wrong evidence", "nested":{"text":text}}).to_string(),
                    &"/nested/text".parse().unwrap(),
                )
            });
            if relations {
                engine(&server, adapter)
                    .call(
                        &Relate::new(vec![
                            RelationRule::parse("works_for=person:company").unwrap(),
                        ])
                        .unwrap(),
                        records,
                        |row| rows.push(serde_json::to_value(row).unwrap()),
                    )
                    .unwrap();
            } else {
                engine(&server, adapter)
                    .call(
                        &Recognize::new(["person", "company"]).unwrap(),
                        records,
                        |row| rows.push(serde_json::to_value(row).unwrap()),
                    )
                    .unwrap();
            }
            assert_eq!(rows.len(), 2);
            for (row, (person, company)) in rows.iter().zip([("Ada", "Acme"), ("Zoë", "Orbit")]) {
                assert_eq!(row["status"], "answered");
                if relations {
                    assert_eq!(row["answer"].as_array().unwrap().len(), 1);
                    assert_eq!(row["answer"][0]["source"]["text"], person);
                    assert_eq!(row["answer"][0]["target"]["text"], company);
                } else {
                    assert_eq!(row["answer"].as_array().unwrap().len(), 2);
                    assert_eq!(row["answer"][0]["text"], person);
                    assert_eq!(row["answer"][1]["text"], company);
                }
            }
            check_states(
                &server,
                adapter,
                &[
                    ("Ada joined Acme", [3, 2, usize::from(relations)]),
                    ("Zoë joined Orbit", [3, 2, usize::from(relations)]),
                ],
            );
        }
        let server = server(adapter);
        let toolbox = Toolbox::from_bytes(br#"{"questions":{"decision":{"decide":"Is this stated?","on":"/decision"},"names":{"recognize":["person","company"],"on":"/fields/names"},"relations":{"relate":[{"name":"works_for","source":"person","target":"company"}],"on":"/fields/relations"}}}"#).unwrap();
        let record = json!({"container":{"decision":"yes", "fields":{"names":"Ada one two three four five six seven left", "relations":"Ada one two three four five six seven right Acme"}}, "fields":{"names":"wrong"}});
        let mut rows = Vec::new();
        engine(&server, adapter)
            .annotate(
                &Annotate::new(&toolbox),
                [Record::json(
                    &record.to_string(),
                    &"/container".parse().unwrap(),
                )],
                |row| rows.push(serde_json::to_value(row).unwrap()),
            )
            .unwrap();
        assert_eq!(rows.len(), 1);
        let answer = &rows[0]["answer"];
        assert_eq!(answer["decision"]["answer"], true);
        assert_eq!(answer["names"]["answer"].as_array().unwrap().len(), 1);
        assert_eq!(answer["names"]["answer"][0]["text"], "Ada");
        assert_eq!(answer["relations"]["answer"].as_array().unwrap().len(), 1);
        assert_eq!(answer["relations"]["answer"][0]["source"]["text"], "Ada");
        check_states(
            &server,
            adapter,
            &[
                ("Each question quotes the text it asks about.", [0, 0, 1]),
                ("Ada one two three four five six seven left", [9, 1, 0]),
                (
                    "Ada one two three four five six seven right Acme",
                    [10, 2, 1],
                ),
            ],
        );
    }
}

#[test]
fn shared_evidence_counts_toward_the_request_byte_limit() {
    // Invisible prefix scalars are omitted from marked snippets but remain evidence.
    let text = format!("{}Ada", "\u{feff}".repeat(32_000));
    for adapter in [Adapter::SystemOne, Adapter::OpenAi] {
        let server = server(adapter);
        let mut rows = Vec::new();
        engine(&server, adapter)
            .call(
                &Recognize::new(["person"]).unwrap(),
                [Record::text(&text)],
                |row| rows.push(serde_json::to_value(row).unwrap()),
            )
            .unwrap_err();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0]["status"], "failed");
        assert_eq!(rows[0]["error"]["code"], "backend.limit");
        assert_eq!(server.count(), 0);
    }
}
