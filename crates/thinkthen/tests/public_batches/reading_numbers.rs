//! Vendor confidence is preserved; probabilities determine typed outcomes.

use super::*;
use thinkthen::Probabilities;

#[test]
fn reported_confidence_and_wire_score_do_not_replace_public_numbers() {
    let _serial = serial();
    let score =
        include_str!("../../../../specification/fixtures/systemone/score-disruption.response.json");
    let cases = [
        (
            r#"{"score":"How much disruption?","levels":["None.","Work continues with a workaround.","Work is blocked."]}"#,
            score,
            Judgment::Score(1.87),
            0.79,
        ),
        (
            r#"{"choose":"Which label?","options":["first","second"],"threshold":0.8}"#,
            r#"{"model":"jev-latest","answers":{"q1":{"type":"choice","probabilities":{"first":0.9,"second":0.1},"confidence":0.1}}}"#,
            Judgment::Choice(Some("first".into())),
            0.1,
        ),
        (
            r#"{"choose":"Which label?","options":["first","second"]}"#,
            r#"{"model":"jev-latest","answers":{"q1":{"type":"choice","probabilities":{"first":0.5,"second":0.5},"confidence":1.0}}}"#,
            Judgment::Choice(None),
            1.0,
        ),
    ];
    for (source, reply, expected, confidence) in cases {
        assert_public_numbers(source, reply, expected, confidence);
    }
}

#[cfg(test)]
fn assert_public_numbers(source: &str, reply: &'static str, expected: Judgment, confidence: f64) {
    let listener = Listener::answering(move |_| Canned::ok(reply)).expect("listener");
    let engine = engine(listener.base());
    let thinkthen::LoadedQuestion::Question(asked) =
        Question::from_json(source).expect("dynamic question")
    else {
        panic!("unbanded question");
    };
    let (json, actual, supplied_confidence, probabilities, raw_level, requests_sent) =
        match &expected {
            Judgment::Score(_) => {
                let call = engine.score(&asked, "fixture evidence").expect("score");
                let result = &call.results()[0];
                (
                    serde_json::from_str::<serde_json::Value>(&result.to_json().expect("JSON"))
                        .expect("JSON"),
                    Judgment::Score(*call.value()),
                    result.confidence(),
                    result.probabilities(),
                    result.raw_level().map(str::to_owned),
                    call.facts().requests_sent(),
                )
            }
            Judgment::Choice(_) => {
                let call = crate::api::choose(
                    &engine,
                    &asked,
                    crate::api::plain_records(["fixture evidence"]).map(Result::unwrap),
                    CallOptions::new(),
                )
                .expect("choose");
                let result = call.value()[0].result();
                (
                    serde_json::from_str::<serde_json::Value>(&result.to_json().expect("JSON"))
                        .expect("JSON"),
                    Judgment::Choice(result.value().map(str::to_owned)),
                    result.confidence(),
                    result.probabilities(),
                    None,
                    call.facts().requests_sent(),
                )
            }
            _ => panic!("score or choice fixture"),
        };
    assert_eq!(actual, expected, "0402 weighted value or choice cut");
    assert_eq!(
        supplied_confidence,
        Some(confidence),
        "0402 vendor confidence"
    );
    assert_eq!(
        json["answer"]["confidence"], confidence,
        "0402 serialized confidence"
    );
    assert_eq!(listener.count(), 1, "0402 one counted request");
    assert_eq!(requests_sent, 1);
    assert_eq!(json["meta"]["requests"].as_array().unwrap().len(), 1);
    if let Judgment::Score(_) = expected {
        assert_eq!(raw_level.as_deref(), Some("Work is blocked."));
        let Probabilities::Named(probabilities) = probabilities else {
            panic!("score distribution");
        };
        assert_eq!(
            probabilities
                .iter()
                .map(|p| p.probability())
                .collect::<Vec<_>>(),
            vec![0.0, 0.13, 0.87]
        );
        assert_eq!(json["value"], 1.87, "0402 serialized weighted value");
    } else {
        assert_eq!(
            json["value"],
            match expected {
                Judgment::Choice(Some(label)) => serde_json::Value::String(label),
                _ => serde_json::Value::Null,
            }
        );
        assert_eq!(json["answer"]["pick"], "first");
    }
}

#[test]
fn typed_band_endpoints_match_the_illustrative_trust_rows() {
    let _serial = serial();
    let cases = [
        (0.2, Answer::No, Answer::Unsure),
        (0.3, Answer::Unsure, Answer::Unsure),
        (0.5, Answer::Unsure, Answer::Unsure),
        (0.7, Answer::Yes, Answer::Unsure),
        (0.9, Answer::Yes, Answer::Yes),
    ];
    for (probability, symmetric, uneven) in cases {
        for (low, high, expected) in [(0.3, 0.7, symmetric), (0.2, 0.9, uneven)] {
            let reply = format!(
                r#"{{"model":"jev-latest","answers":{{"q1":{{"type":"noul","noul":{probability}}}}}}}"#
            );
            let listener = Listener::answering(move |_| Canned::ok(&reply)).expect("listener");
            let engine = engine(listener.base());
            let asked = Question::decide("Does this pass?")
                .expect("question")
                .band(low, high)
                .expect("band");
            let call = engine.decide(&asked, "fixture evidence").expect("decide");
            assert_eq!(
                call.results()[0].value(),
                expected,
                "0402 band boundary p={probability} under {low}:{high}"
            );
            assert_eq!(
                call.results()[0].probabilities(),
                Probabilities::YesNo { yes: probability }
            );
            assert_eq!(call.results()[0].confidence(), None);
            assert_eq!(listener.count(), 1);
        }
    }
}
