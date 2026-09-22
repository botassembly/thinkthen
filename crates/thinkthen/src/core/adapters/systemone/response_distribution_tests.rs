//! The evidence-backed System One distribution boundary.

use super::decode;
use crate::core::adapters::systemone::DecodeError;
use crate::core::adapters::systemone::tests::{disruption_plan, team_plan};
use crate::core::answer::Value;
use crate::core::probability::Probability;
use crate::core::reply::AnswerOutcome;

fn choice_with(values: [f64; 4]) -> String {
    let [v0, v1, v2, v3] = values;
    concat!(
        r#"{"model":"jev-latest","answers":{"q1":{"type":"choice","#,
        r#""probabilities":{"billing":V0,"shipping":V1,"account":V2,"other":V3}}}}"#,
    )
    .replace("V0", &v0.to_string())
    .replace("V1", &v1.to_string())
    .replace("V2", &v2.to_string())
    .replace("V3", &v3.to_string())
}

fn score_with(values: [f64; 3]) -> String {
    let [v0, v1, v2] = values;
    concat!(
        r#"{"model":"jev-latest","answers":{"q1":{"type":"score","#,
        r#""probabilities":{"0":V0,"1":V1,"2":V2}}}}"#,
    )
    .replace("V0", &v0.to_string())
    .replace("V1", &v1.to_string())
    .replace("V2", &v2.to_string())
}

#[test]
fn a_distribution_total_error_names_the_rule_without_reply_values() {
    for (member, total) in [("0.2", "0.8"), ("0.3", "1.2")] {
        let body = concat!(
            r#"{"model":"jev-latest","answers":{"q1":{"type":"choice","#,
            r#""probabilities":{"billing":VALUE,"shipping":VALUE,"account":VALUE,"other":VALUE}}}}"#,
        )
        .replace("VALUE", member);
        let error = decode(&team_plan(), body.as_bytes()).expect_err("an invalid total");
        assert_eq!(
            error.to_string(),
            format!(
                "the answer to question `q1` has probability total {total}, member count 4, and tolerance 0.01; the total differs from one by more than the tolerance"
            )
        );
        assert!(!error.to_string().contains(member));
    }
}

#[test]
fn systemone_accepts_one_hundredth_on_both_sides_and_preserves_members() {
    for values in [
        [0.24, 0.25, 0.25, 0.25],
        [0.240_000_000_000_000_1, 0.25, 0.25, 0.25],
        [0.26, 0.25, 0.25, 0.25],
        [0.260_000_000_000_000_1, 0.25, 0.25, 0.25],
    ] {
        let reply = decode(&team_plan(), choice_with(values).as_bytes())
            .expect("System One rounds to hundredths");
        let [AnswerOutcome::Answered(answer)] = reply.outcomes() else {
            panic!("one answer");
        };
        assert_eq!(
            answer
                .distribution()
                .expect("a distribution")
                .probabilities()
                .map(Probability::as_f64)
                .collect::<Vec<_>>(),
            values
        );
    }

    for (values, expected) in [
        ([0.0, 0.01, 0.98], 1.989_898_989_899),
        ([0.01, 0.0, 1.0], 1.980_198_019_802),
    ] {
        let reply =
            decode(&disruption_plan(), score_with(values).as_bytes()).expect("a rounded score");
        let [AnswerOutcome::Answered(answer)] = reply.outcomes() else {
            panic!("one answer");
        };
        assert_eq!(
            answer
                .distribution()
                .expect("a distribution")
                .probabilities()
                .map(Probability::as_f64)
                .collect::<Vec<_>>(),
            values
        );
        let Value::Score(score) = answer.read(None).0 else {
            panic!("a score");
        };
        assert_eq!(score, expected);
    }
}

#[test]
fn systemone_refuses_totals_beyond_one_hundredth() {
    for total in [0.98, 1.02, 0.85, 1.15, 0.0] {
        let member = total / 4.0;
        let body = choice_with([member; 4]);
        assert!(
            matches!(
                decode(&team_plan(), body.as_bytes()),
                Err(DecodeError::DistributionTotal { place: 0, .. })
            ),
            "{total}"
        );
    }
    let body = score_with([1.0, 1.0, 1.0]);
    assert!(matches!(
        decode(&disruption_plan(), body.as_bytes()),
        Err(DecodeError::DistributionTotal { place: 0, .. })
    ));
}
