//! Caller-defined literal spans through the real native and command boundaries.
use super::{local, marked, questions, stdout};
use crate::harness::{Canned, Listener};
use serde_json::{Value, json};

const INPUT: &str = "é TOTAL 42.75.";
const INSTRUCTIONS: &str = "Return literal receipt amounts.\nExclude the TOTAL label.";
const DEFINITION: &str = "The complete numeric amount, including its decimal point.";
const DESCRIPTION: &str = "Receipt total amount, not its label.";

fn literal(body: &[u8]) -> Canned {
    let request: Value = serde_json::from_slice(body).expect("request");
    let mut answers = serde_json::Map::new();
    for (id, q) in request["questions"].as_object().expect("questions") {
        let criteria = q["criteria"].as_object().expect("criteria");
        let words = q["instructions"].as_str().expect("words");
        let (before, span, after) = marked(words);
        let chosen = if criteria.contains_key("BEGIN") {
            match span {
                "42" => "BEGIN",
                "." if before.ends_with("42") && after.starts_with("75") => "INSIDE",
                "75" => "END",
                _ => "OUT",
            }
        } else if criteria.contains_key("none of these") {
            "amount"
        } else {
            "42.75"
        };
        assert!(
            criteria.contains_key(chosen),
            "missing controlled option {chosen}"
        );
        let probabilities = criteria
            .keys()
            .map(|label| {
                (
                    label.clone(),
                    Value::from(if label == chosen { 1.0 } else { 0.0 }),
                )
            })
            .collect::<serde_json::Map<_, _>>();
        answers.insert(
            id.clone(),
            json!({"type":"choice","choice":chosen,"probabilities":probabilities}),
        );
    }
    Canned::ok(&json!({"model":"local-1","answers":answers}).to_string())
}

fn expected(kind: &str) -> Value {
    json!({"entities":[{"text":"42.75","start":8,"end":13,"length":5,"kind":kind,"strength":1.0}]})
}

#[test]
fn caller_semantics_reach_all_stages_and_keep_literal_offsets() {
    let listener = Listener::answering(literal).expect("listener");
    let engine = thinkthen::Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .api_key("fake")
        .expect("key")
        .no_cache()
        .build()
        .expect("engine");
    let ask = thinkthen::Recognize::builder()
        .instructions(INSTRUCTIONS)
        .expect("instructions")
        .entity_definition(DEFINITION)
        .expect("definition")
        .kind(
            thinkthen::Kind::new(
                "amount",
                Some(thinkthen::Description::text(DESCRIPTION).expect("description")),
            )
            .expect("kind"),
        )
        .expect("kind")
        .build()
        .expect("declaration");
    let result = engine.recognize(&ask, INPUT).expect("result");
    assert_eq!(
        serde_json::from_str::<Value>(&result.value().to_json()).expect("JSON"),
        expected("amount")
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 2);
    let mut stages = [false; 3];
    for request in requests {
        for q in questions(&request.body) {
            let words = q["instructions"].as_str().expect("words");
            assert!(words.contains(INSTRUCTIONS));
            assert!(words.contains(DEFINITION));
            assert!(words.contains(DESCRIPTION));
            for legacy in [
                "proper name",
                "Ordinary words",
                "dates, numbers",
                "not a proper name",
            ] {
                assert!(!words.contains(legacy), "legacy restriction: {words}");
            }
            let criteria = q["criteria"].as_object().expect("criteria");
            let at = if criteria.contains_key("BEGIN") {
                0
            } else if criteria.contains_key("none of these") {
                1
            } else {
                2
            };
            stages[at] = true;
            if at == 1 {
                assert_eq!(criteria["amount"], DESCRIPTION);
                assert_eq!(
                    criteria["none of these"],
                    "The span fails the caller declaration or no listed kind applies."
                );
            }
        }
    }
    assert_eq!(stages, [true; 3]);
}

#[test]
fn each_custom_input_enables_values_without_an_extra_switch() {
    for args in [
        vec!["--instructions", INSTRUCTIONS],
        vec!["--entity-definition", DEFINITION],
        vec!["--kind", "amount=Receipt total amount, not its label."],
    ] {
        let listener = Listener::answering(literal).expect("listener");
        let output = local(&listener, &args, Some("fake"), INPUT.as_bytes());
        assert_eq!(output.status.code(), Some(0));
        let result: Value = serde_json::from_str(&stdout(&output)).expect("JSON");
        assert_eq!(
            result,
            expected(if args[0] == "--kind" {
                "amount"
            } else {
                "ENTITY"
            })
        );
        assert_eq!(listener.requests().len(), 2);
    }
}

#[test]
fn blank_cli_customization_refuses_before_sending() {
    let listener = Listener::answering(literal).expect("listener");
    for flag in ["--instructions", "--entity-definition"] {
        let output = local(&listener, &[flag, " \n "], Some("fake"), INPUT.as_bytes());
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        assert!(listener.requests().is_empty());
    }
}
