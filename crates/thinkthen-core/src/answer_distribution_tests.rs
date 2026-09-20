use super::{Answer, Distribution, Value};
use crate::probability::Probability;

fn odds(entries: &[(&str, f64)]) -> Distribution {
    Distribution::new(
        entries
            .iter()
            .map(|(label, value)| {
                (
                    (*label).to_owned(),
                    Probability::new(*value).expect("a probability"),
                )
            })
            .collect(),
    )
    .expect("a distribution")
}

#[test]
fn a_distribution_accepts_only_one_within_member_count_epsilon() {
    let cases = [
        (&[1.0, 0.0, 0.0][..], true),
        (&[1.0 - 2.0 * f64::EPSILON, 0.0, 0.0][..], true),
        (&[1.0 - 4.0 * f64::EPSILON, 0.0, 0.0][..], false),
        (&[1.0, 2.0 * f64::EPSILON, 0.0][..], true),
        (&[1.0, 4.0 * f64::EPSILON, 0.0][..], false),
        (&[1.0, 1.0, 1.0][..], false),
    ];
    for (values, accepted) in cases {
        let entries = values
            .iter()
            .enumerate()
            .map(|(place, value)| {
                (
                    place.to_string(),
                    Probability::new(*value).expect("a probability"),
                )
            })
            .collect();
        assert_eq!(Distribution::new(entries).is_ok(), accepted, "{values:?}");
    }
}

#[test]
fn normalized_scores_stay_inside_the_level_scale_at_both_endpoints() {
    let cases = [
        (
            &[
                ("low", 1.0 - 2.0 * f64::EPSILON),
                ("mid", 0.0),
                ("high", 0.0),
            ],
            0.0,
        ),
        (
            &[("low", 1.0), ("mid", 2.0 * f64::EPSILON), ("high", 0.0)],
            0.0,
        ),
        (
            &[
                ("low", 0.0),
                ("mid", 0.0),
                ("high", 1.0 - 2.0 * f64::EPSILON),
            ],
            2.0,
        ),
        (
            &[("low", 0.0), ("mid", 2.0 * f64::EPSILON), ("high", 1.0)],
            2.0,
        ),
    ];
    for (entries, expected) in cases {
        let answer = Answer::new_score(odds(entries), None).expect("a level carries odds");
        let Value::Score(score) = answer.read(None).0 else {
            panic!("a score prints a number");
        };
        assert!((0.0..=2.0).contains(&score), "{score}");
        assert_eq!(score, expected);
    }
}

#[test]
fn a_tolerance_edge_distribution_serializes_without_rewriting_members() {
    let reported = [1.0, 3.0 * f64::EPSILON, 0.0];
    let distribution = odds(&[
        ("low", reported[0]),
        ("mid", reported[1]),
        ("high", reported[2]),
    ]);
    let members = distribution
        .probabilities()
        .map(Probability::as_f64)
        .collect::<Vec<_>>();
    assert_eq!(members, reported);
    assert_eq!(
        serde_json::to_string(&distribution).expect("a distribution serializes"),
        r#"{"low":1.0,"mid":6.661338147750939e-16,"high":0.0}"#
    );
}
