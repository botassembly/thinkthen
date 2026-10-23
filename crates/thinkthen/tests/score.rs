#![cfg(test)]
//! `score` through the public library.

use loopback::{KEY, Loopback};
use serde_json::{Value, json};
use thinkthen::{Backend, Engine, Outcome, Record, Score};

#[test]
fn a_scale_returns_its_probability_weighted_position_and_detail() {
    let cases = [
        (vec!["low", "high"], vec![0.2, 0.8], Some(0.8)),
        (
            vec!["none", "low", "middle", "high", "all"],
            vec![0.1, 0.2, 0.3, 0.2, 0.2],
            Some(0.55),
        ),
        (vec!["low", "high"], vec![0.1, 0.895], Some(0.895 / 0.995)),
        (vec!["low", "high"], vec![0.2, 0.5], None),
    ];
    for (levels, probabilities, expected) in cases {
        let map: serde_json::Map<String, Value> = probabilities
            .iter()
            .enumerate()
            .map(|(i, p)| (i.to_string(), json!(p)))
            .collect();
        let server = Loopback::answering(
            move |_| json!({"type": "score", "probabilities": map, "score": 99}),
        )
        .unwrap();
        let backend = Backend::resolve(Some(server.base()), None, |name| {
            (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned())
        })
        .unwrap();
        let question = Score::new("How urgent?", levels.iter().copied()).unwrap();
        let mut rows = Vec::new();
        let result =
            Engine::new(backend).call(&question, [Record::text("Say \"now\"\nplease")], |row| {
                rows.push(row)
            });
        assert_eq!(result.is_ok(), expected.is_some());
        assert_eq!(rows.len(), 1);
        let row = rows.pop().unwrap();
        let value = serde_json::to_value(&row).unwrap();
        match expected {
            Some(expected) => {
                assert_eq!(value["status"], "answered");
                assert!((value["answer"].as_f64().unwrap() - expected).abs() < 1e-12);
                let Outcome::Answered {
                    detail:
                        Some(thinkthen::Detail::Sourced {
                            probabilities: Some(detail),
                            ..
                        }),
                    ..
                } = row.outcome()
                else {
                    panic!("expected scale probabilities");
                };
                let thinkthen::Detail::Levels(detail) = detail.as_ref() else {
                    panic!("expected scale detail");
                };
                assert_eq!(
                    detail.iter().map(|p| p.get()).collect::<Vec<_>>(),
                    probabilities
                );
            }
            None => match row.outcome() {
                Outcome::Failed { error } => assert_eq!(error.kind().code(), "backend.reply"),
                other => panic!("expected a failure, got {other:?}"),
            },
        }
        assert_eq!(
            server.requests()[0].json().unwrap()["questions"]["q1"],
            json!({
                "type": "score", "instructions": "The text is \"Say \\\"now\\\"\\nplease\". How urgent?", "criteria": levels
            })
        );
    }
}

#[test]
fn malformed_scale_answers_fail_without_exposing_backend_text() {
    let cases = [
        json!({"type": "noul", "noul": 0.7}),
        json!({"type": "score"}),
        json!({"type": "score", "probabilities": {"0": 1.0}}),
        json!({"type": "score", "probabilities": {"0": 0.2, "1": 0.8, "2": 0.0}}),
        json!({"type": "score", "probabilities": {"00": 0.2, "1": 0.8}}),
        json!({"type": "score", "probabilities": {"0": -0.1, "1": 1.1}}),
        json!({"type": "score", "probabilities": {"0": 0.0, "1": 0.0}}),
    ];
    for answer in cases {
        let server = Loopback::answering(move |_| answer.clone()).unwrap();
        let backend = Backend::resolve(Some(server.base()), None, |name| {
            (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned())
        })
        .unwrap();
        let question = Score::new("How urgent?", ["low", "high"]).unwrap();
        let mut rows = Vec::new();
        Engine::new(backend)
            .call(&question, [Record::text("text")], |row| rows.push(row))
            .unwrap_err();
        assert_eq!(rows.len(), 1);
        match rows[0].outcome() {
            Outcome::Failed { error } => assert_eq!(error.kind().code(), "backend.reply"),
            other => panic!("expected a failure, got {other:?}"),
        }
    }
}

#[test]
fn a_scale_requires_a_question_and_two_distinct_nonblank_levels() {
    for (text, levels) in [
        (" ", vec!["low", "high"]),
        ("How urgent?", vec![]),
        ("How urgent?", vec!["low"]),
        ("How urgent?", vec!["low", " "]),
        ("How urgent?", vec!["low", "low"]),
    ] {
        assert_eq!(
            Score::new(text, levels).unwrap_err().kind().code(),
            "usage.question"
        );
    }
}
