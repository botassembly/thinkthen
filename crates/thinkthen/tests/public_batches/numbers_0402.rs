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
        let listener = Listener::serving(vec![Canned::ok(reply)]).expect("listener");
        let engine = engine(listener.base());
        let thinkthen::LoadedQuestion::Question(asked) =
            Question::from_json(source).expect("dynamic question")
        else {
            panic!("unbanded question");
        };
        let call = engine.details(&asked, "fixture evidence").expect("details");
        let details = call.value();
        assert_eq!(
            details.value(),
            &expected,
            "0402 weighted value or choice cut"
        );
        assert_eq!(
            details.confidence(),
            Some(confidence),
            "0402 vendor confidence"
        );
        let json: serde_json::Value = serde_json::from_str(&details.to_json()).expect("JSON");
        assert_eq!(
            json["answer"]["confidence"], confidence,
            "0402 serialized confidence"
        );
        assert_eq!(listener.count(), 1, "0402 one counted request");
        assert_eq!(call.facts().requests_sent(), 1);
        assert_eq!(details.requests().len(), 1);
        if let Judgment::Score(_) = expected {
            assert_eq!(details.nearest(), Some("Work is blocked."));
            let Probabilities::Named(probabilities) = details.probabilities() else {
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
            let listener = Listener::serving(vec![Canned::ok(&reply)]).expect("listener");
            let engine = engine(listener.base());
            let asked = Question::decide("Does this pass?")
                .expect("question")
                .band(low, high)
                .expect("band");
            let call = engine.details(&asked, "fixture evidence").expect("details");
            assert_eq!(
                call.value().value(),
                &Judgment::Decision(expected),
                "0402 band boundary p={probability} under {low}:{high}"
            );
            assert_eq!(
                call.value().probabilities(),
                &Probabilities::YesNo { yes: probability }
            );
            assert_eq!(call.value().confidence(), None);
            assert_eq!(listener.count(), 1);
        }
    }
}
