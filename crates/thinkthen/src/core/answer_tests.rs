use super::{Answer, Distribution, Value};
use crate::core::probability::Probability;
use crate::core::threshold::{Outcome, Threshold};
use proptest::collection::vec;
use proptest::prelude::Strategy;
use proptest::{prop_assert, prop_assert_eq, proptest};

/// One case's distribution, as a label and a probability per entry.
type Odds<'a> = &'a [(&'a str, f64)];

/// One reading case: the odds, the rule, and what the verb then prints.
type Reading<'a> = (Odds<'a>, Option<Threshold>, Value, Outcome);

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

fn choice(entries: &[(&str, f64)]) -> Answer {
    Answer::new_choice(odds(entries), None).expect("one option carries odds")
}

fn rule(text: &str) -> Threshold {
    text.parse().expect("a threshold")
}

#[test]
fn a_pick_clears_the_cut_falls_under_it_or_ties_for_first() {
    let split = &[("bug", 0.5), ("feature", 0.5), ("other", 0.0)];
    let clear = &[("bug", 0.94), ("feature", 0.04), ("other", 0.02)];
    let cases: [Reading<'_>; 6] = [
        (
            clear,
            None,
            Value::Choice(Some("bug".to_owned())),
            Outcome::Yes,
        ),
        (
            clear,
            Some(rule("0.8")),
            Value::Choice(Some("bug".to_owned())),
            Outcome::Yes,
        ),
        (
            clear,
            Some(rule("0.94")),
            Value::Choice(Some("bug".to_owned())),
            Outcome::Yes,
        ),
        (
            clear,
            Some(rule("0.99")),
            Value::Choice(None),
            Outcome::Unresolved,
        ),
        (split, None, Value::Choice(None), Outcome::Unresolved),
        (
            split,
            Some(rule("0.1")),
            Value::Choice(None),
            Outcome::Unresolved,
        ),
    ];
    for (entries, threshold, value, outcome) in cases {
        assert_eq!(
            choice(entries).read(threshold),
            (value, outcome),
            "{threshold:?}"
        );
    }
}

#[test]
fn tag_selects_each_label_at_or_above_one_shared_cut_and_empty_succeeds() {
    let answer = Answer::new_tag(vec![
        (
            "billing".to_owned(),
            Probability::new(0.5).expect("probability"),
        ),
        (
            "urgent".to_owned(),
            Probability::new(0.2).expect("probability"),
        ),
    ]);
    assert_eq!(
        answer.read(None),
        (Value::Tag(vec!["billing".to_owned()]), Outcome::Yes)
    );
    assert_eq!(
        answer.read(Some(rule("0.9"))),
        (Value::Tag(Vec::new()), Outcome::Yes)
    );
    assert_eq!(
        serde_json::to_string(&answer).expect("answer serializes"),
        r#"{"kind":"tag","probabilities":{"billing":0.5,"urgent":0.2}}"#
    );
}

#[test]
fn a_score_is_the_weighted_position_on_the_levels_it_was_given() {
    let cases: [(Odds<'_>, f64); 4] = [
        (&[("low", 0.05), ("mid", 0.30), ("high", 0.65)], 1.6),
        (&[("low", 0.0), ("mid", 1.0), ("high", 0.0)], 1.0),
        (&[("low", 0.5), ("mid", 0.0), ("high", 0.5)], 1.0),
        (&[("low", 1.0), ("mid", 0.0)], 0.0),
    ];
    for (entries, expected) in cases {
        let answer = Answer::new_score(odds(entries), None).expect("one level carries odds");
        let (value, outcome) = answer.read(None);
        let Value::Score(number) = value else {
            panic!("a score prints a number");
        };
        assert!(
            (number - expected).abs() < 1e-12,
            "{number} is not {expected}"
        );
        assert_eq!(outcome, Outcome::Yes);
    }
}

#[test]
fn an_answer_serializes_in_the_shape_the_specification_prints() {
    let answer = Answer::new_choice(
        odds(&[("bug", 0.94), ("feature", 0.04), ("other", 0.02)]),
        Some(Probability::new(0.91).expect("a probability")),
    )
    .expect("one option carries odds");
    assert_eq!(
        serde_json::to_string(&answer).expect("an answer serializes"),
        concat!(
            r#"{"kind":"choice","pick":"bug","#,
            r#""probabilities":{"bug":0.94,"feature":0.04,"other":0.02},"#,
            r#""confidence":0.91}"#,
        )
    );

    let answer = Answer::new_score(odds(&[("low", 0.4), ("high", 0.6)]), None)
        .expect("one level carries odds");
    assert_eq!(
        serde_json::to_string(&answer).expect("an answer serializes"),
        r#"{"kind":"score","level":"high","probabilities":{"low":0.4,"high":0.6}}"#
    );
}

#[test]
fn only_a_yes_no_answer_carries_the_probability_that_rank_orders_by() {
    let probability = Probability::new(0.82).expect("a probability");
    assert_eq!(Answer::new_yes_no(probability).yes(), Some(0.82));
    assert_eq!(choice(&[("bug", 0.94), ("feature", 0.06)]).yes(), None);
    let placed = Answer::new_score(odds(&[("low", 0.4), ("high", 0.6)]), None)
        .expect("one level carries odds");
    assert_eq!(placed.yes(), None);
}

#[test]
fn only_a_pick_carries_a_label_for_the_raw_view() {
    assert_eq!(Value::Choice(Some("bug".to_owned())).label(), Some("bug"));
    assert_eq!(Value::Choice(None).label(), None);
    assert_eq!(Value::YesNo(Some(true)).label(), None);
    assert_eq!(Value::Score(1.6).label(), None);
}

/// A label and a probability, so a generated distribution is a real one.
fn entries() -> impl Strategy<Value = Vec<(String, f64)>> {
    vec((1_usize..8, 0.0_f64..=1.0), 2..8).prop_map(|drawn| {
        let total: f64 = drawn.iter().map(|(_, value)| *value).sum();
        drawn
            .into_iter()
            .enumerate()
            .map(|(place, (width, value))| {
                let value = normalized(place, value, total);
                (format!("{place}{}", "x".repeat(width)), value)
            })
            .collect()
    })
}

fn normalized(place: usize, value: f64, total: f64) -> f64 {
    if total == 0.0 {
        return f64::from(place == 0);
    }
    value / total
}

proptest! {
    /// The bare value of `choose` is one of the options sent, or `null`.
    #[test]
    fn a_pick_is_always_an_option_that_was_sent_or_nothing(
        drawn in entries(),
        mark in 0.001_f64..=1.0,
        cut in proptest::bool::ANY,
    ) {
        let listed: Vec<(&str, f64)> = drawn
            .iter()
            .map(|(label, value)| (label.as_str(), *value))
            .collect();
        let answer = choice(&listed);
        let threshold = cut.then(|| Threshold::cut(mark).expect("a cut"));
        let (value, outcome) = answer.read(threshold);
        let Value::Choice(label) = value else {
            panic!("a pick prints a label or nothing");
        };
        match label {
            Some(label) => {
                prop_assert!(drawn.iter().any(|(sent, _)| *sent == label));
                prop_assert_eq!(outcome, Outcome::Yes);
            }
            None => prop_assert_eq!(outcome, Outcome::Unresolved),
        }
    }
}
