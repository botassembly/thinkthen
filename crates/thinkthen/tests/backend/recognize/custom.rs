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

#[test]
#[allow(
    clippy::cognitive_complexity,
    clippy::too_many_lines,
    reason = "one stored declaration proves precedence, successful replay and semantic misses in sequence"
)]
fn saved_customization_obeys_cli_precedence_and_semantic_cache_identity() {
    let root = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("recognize-custom-identity-{}", std::process::id()));
    std::fs::create_dir_all(&root).expect("folder");
    let saved = root.join("task.json");
    let declaration = json!({"version":1,"recognize":{"instructions":INSTRUCTIONS,"entity_definition":DEFINITION,"kinds":{"amount":DESCRIPTION}}});
    std::fs::write(&saved, declaration.to_string()).expect("saved declaration");
    let reference = format!("@{}", saved.display());
    let recording = root.join("recording");
    let cache = root.join("cache");
    let recording = recording.to_str().expect("recording path");
    let cache = cache.to_str().expect("cache path");
    let listener = Listener::answering(literal).expect("listener");
    let recorded = local(
        &listener,
        &[&reference, "--record", recording, "--no-cache"],
        Some("fake"),
        INPUT.as_bytes(),
    );
    assert_eq!(
        serde_json::from_str::<Value>(&stdout(&recorded)).expect("result"),
        expected("amount")
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 2);
    let replay = local(
        &listener,
        &[&reference, "--replay", recording, "--no-cache"],
        None,
        INPUT.as_bytes(),
    );
    assert_eq!(stdout(&replay), stdout(&recorded));
    assert!(listener.requests().is_empty());
    let filled = local(
        &listener,
        &[&reference, "--cache", cache],
        Some("fake"),
        INPUT.as_bytes(),
    );
    assert_eq!(stdout(&filled), stdout(&recorded));
    assert_eq!(listener.requests().len(), 2);
    let cached = local(
        &listener,
        &[&reference, "--cache", cache],
        None,
        INPUT.as_bytes(),
    );
    assert_eq!(stdout(&cached), stdout(&recorded));
    assert!(listener.requests().is_empty());
    // Each changed semantic field must miss both the recording and the old cache.
    for (field, changed) in [
        ("instructions", "Find amounts under another instruction."),
        ("entity_definition", "An amount under another definition."),
        ("description", "An amount under another label description."),
    ] {
        let mut altered = declaration.clone();
        if field == "description" {
            altered["recognize"]["kinds"]["amount"] = json!(changed);
        } else {
            altered["recognize"][field] = json!(changed);
        }
        std::fs::write(&saved, altered.to_string()).expect("changed declaration");
        let miss = local(
            &listener,
            &[&reference, "--replay", recording, "--no-cache"],
            None,
            INPUT.as_bytes(),
        );
        assert_eq!(miss.status.code(), Some(5));
        assert!(miss.stdout.is_empty());
        assert!(listener.requests().is_empty());
        let called = local(
            &listener,
            &[&reference, "--cache", cache],
            Some("fake"),
            INPUT.as_bytes(),
        );
        assert_eq!(stdout(&called), stdout(&recorded));
        let sent = listener.requests();
        assert_eq!(sent.len(), 2);
        assert!(sent.iter().all(|r| {
            questions(&r.body)
                .iter()
                .all(|q| q["instructions"].as_str().expect("words").contains(changed))
        }));
    }
    // Explicit flags replace only their saved members; label descriptions survive.
    std::fs::write(&saved, declaration.to_string()).expect("original declaration");
    let called = local(
        &listener,
        &[
            &reference,
            "--instructions",
            "Override instructions.",
            "--entity-definition",
            "Override definition.",
            "--no-cache",
        ],
        Some("fake"),
        INPUT.as_bytes(),
    );
    assert_eq!(stdout(&called), stdout(&recorded));
    for r in listener.requests() {
        for q in questions(&r.body) {
            let words = q["instructions"].as_str().expect("words");
            assert!(words.contains("Override instructions."));
            assert!(words.contains("Override definition."));
            assert!(words.contains(DESCRIPTION));
            assert!(!words.contains(INSTRUCTIONS));
            assert!(!words.contains(DEFINITION));
        }
    }
    for field in ["instructions", "entity_definition"] {
        for bad in [json!(" \n "), Value::Null, json!({}), json!(1)] {
            let mut invalid = declaration.clone();
            invalid["recognize"][field] = bad;
            std::fs::write(&saved, invalid.to_string()).expect("invalid declaration");
            let refused = local(
                &listener,
                &[&reference, "--no-cache"],
                Some("fake"),
                INPUT.as_bytes(),
            );
            assert_eq!(refused.status.code(), Some(5));
            assert!(refused.stdout.is_empty());
            assert!(listener.requests().is_empty());
        }
    }
}

#[test]
#[allow(
    clippy::too_many_lines,
    clippy::type_complexity,
    clippy::excessive_nesting,
    reason = "the independent gold table and its controlled replies stay together as one literal-span behavior"
)]
fn caller_defined_literal_spans_keep_units_punctuation_repetitions_and_absence() {
    // Gold spans and token answers are frozen independently of the recognizer.
    let cases: &[(&str, &[&str], &[(&str, usize, usize)])] = &[
        (
            "Length: 12.5 mm.",
            &["OUT", "OUT", "BEGIN", "INSIDE", "INSIDE", "END", "OUT"],
            &[("12.5 mm", 8, 15)],
        ),
        (
            "Date: 2026-10-07.",
            &[
                "OUT", "OUT", "BEGIN", "INSIDE", "INSIDE", "INSIDE", "END", "OUT",
            ],
            &[("2026-10-07", 6, 16)],
        ),
        (
            "Count: 3 items.",
            &["OUT", "OUT", "BEGIN", "END", "OUT"],
            &[("3 items", 7, 14)],
        ),
        (
            "Code: AB-12.",
            &["OUT", "OUT", "BEGIN", "INSIDE", "END", "OUT"],
            &[("AB-12", 6, 11)],
        ),
        (
            "Dose label: 5 mg.",
            &["OUT", "OUT", "OUT", "BEGIN", "END", "OUT"],
            &[("5 mg", 12, 16)],
        ),
        (
            "TOTAL 42.75",
            &["OUT", "BEGIN", "INSIDE", "END"],
            &[("42.75", 6, 11)],
        ),
        (
            "Address: https://example.org/a.",
            &[
                "OUT", "OUT", "BEGIN", "INSIDE", "INSIDE", "INSIDE", "INSIDE", "INSIDE", "INSIDE",
                "INSIDE", "END", "OUT",
            ],
            &[("https://example.org/a", 9, 30)],
        ),
        (
            "Word: plain.",
            &["OUT", "OUT", "SINGLE", "OUT"],
            &[("plain", 6, 11)],
        ),
        ("TOTAL missing.", &["OUT", "OUT", "OUT"], &[]),
        (
            "42.75 and 42.75",
            &["BEGIN", "INSIDE", "END", "OUT", "BEGIN", "INSIDE", "END"],
            &[("42.75", 0, 5), ("42.75", 10, 15)],
        ),
        (
            "é 42.75",
            &["OUT", "BEGIN", "INSIDE", "END"],
            &[("42.75", 2, 7)],
        ),
    ];
    for &(text, tags, gold) in cases {
        let tags = tags.to_vec();
        let listener = Listener::answering(move |body| {
            let request: Value = serde_json::from_slice(body).expect("request");
            let mut answers = serde_json::Map::new();
            for (id, q) in request["questions"].as_object().expect("questions") {
                let criteria = q["criteria"].as_object().expect("criteria");
                let chosen = if criteria.contains_key("BEGIN") {
                    tags[id
                        .strip_prefix('q')
                        .expect("question id")
                        .parse::<usize>()
                        .expect("ordinal")
                        - 1]
                } else if criteria.contains_key("none of these") {
                    "value"
                } else {
                    marked(q["instructions"].as_str().expect("words")).1
                };
                assert!(criteria.contains_key(chosen));
                let probabilities = criteria
                    .keys()
                    .map(|label| {
                        (
                            label.clone(),
                            json!(if label == chosen { 1.0 } else { 0.0 }),
                        )
                    })
                    .collect::<serde_json::Map<_, _>>();
                answers.insert(
                    id.clone(),
                    json!({"type":"choice","choice":chosen,"probabilities":probabilities}),
                );
            }
            Canned::ok(&json!({"model":"local-1","answers":answers}).to_string())
        })
        .expect("listener");
        let result = local(
            &listener,
            &[
                "--kind",
                "value=The requested literal value, including its units and spelling.",
                "--no-cache",
            ],
            Some("fake"),
            text.as_bytes(),
        );
        let output: Value = serde_json::from_str(&stdout(&result)).expect("result");
        let entities:Vec<Value>=gold.iter().map(|(span,start,end)|json!({"text":span,"start":start,"end":end,"length":end-start,"kind":"value","strength":1.0})).collect();
        assert_eq!(output, json!({"entities":entities}), "{text}");
        assert_eq!(
            listener.requests().len(),
            if gold.is_empty() { 1 } else { 2 }
        );
    }
}

#[test]
#[allow(
    clippy::excessive_nesting,
    reason = "controlled token, kind and edge replies exercise the retained receipt through one command"
)]
fn retained_0031_receipt_total_uses_the_frozen_amount_occurrence() {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../../../specification/fixtures/recognize/caller-defined/receipt-0031.json"
    ))
    .expect("retained receipt");
    assert_eq!(fixture["prior_gold"]["expected"], "28,000");
    let listener = Listener::answering(|body| {
        let request: Value = serde_json::from_slice(body).expect("request");
        let mut answers = serde_json::Map::new();
        for (id, q) in request["questions"].as_object().expect("questions") {
            let criteria = q["criteria"].as_object().expect("criteria");
            let (before, span, after) = marked(q["instructions"].as_str().expect("words"));
            let chosen = if criteria.contains_key("BEGIN") {
                match span {
                    "28" if before.ends_with("TOTAL SAI S ") => "BEGIN",
                    "," if before.ends_with("TOTAL SAI S 28") && after.starts_with("000") => {
                        "INSIDE"
                    }
                    "000" if before.ends_with("TOTAL SAI S 28,") => "END",
                    _ => "OUT",
                }
            } else if criteria.contains_key("none of these") {
                "amount"
            } else {
                "28,000"
            };
            assert!(criteria.contains_key(chosen));
            let probabilities = criteria
                .keys()
                .map(|label| {
                    (
                        label.clone(),
                        json!(if label == chosen { 1.0 } else { 0.0 }),
                    )
                })
                .collect::<serde_json::Map<_, _>>();
            answers.insert(
                id.clone(),
                json!({"type":"choice","choice":chosen,"probabilities":probabilities}),
            );
        }
        Canned::ok(&json!({"model":"local-1","answers":answers}).to_string())
    })
    .expect("listener");
    let result = local(
        &listener,
        &[
            "--instructions",
            "Return the grand total amount due. Exclude subtotal, labels, cash and change.",
            "--kind",
            "amount=The literal grand total including its separator.",
            "--no-cache",
        ],
        Some("fake"),
        fixture["text"].as_str().expect("text").as_bytes(),
    );
    let output: Value = serde_json::from_str(&stdout(&result)).expect("result");
    let mut gold = fixture["entities"][0].clone();
    gold["strength"] = json!(1.0);
    assert_eq!(output, json!({"entities":[gold]}));
    assert!(!listener.requests().is_empty());
}
