use super::{Judged, Mode, Output};
use crate::core::Outcome;
use crate::engine::usage::Counters;
use crate::failure::Failure;

fn row(name: &str, probability: Option<f64>) -> Judged {
    Judged {
        model: None,
        printed: Some(name.to_owned()),
        outcome: Outcome::Yes,
        replayed: false,
        probability,
        partial_failure: false,
        profile_mismatch: None,
    }
}

#[test]
fn top_sink_never_retains_more_than_the_winners_after_a_callback() {
    let mut written = Vec::new();
    let usage = Counters::new(None);
    let mut output = Output::ordered(&mut written, Some(2), &usage);
    for (name, probability) in [
        ("first", 0.5),
        ("low", 0.1),
        ("best", 0.9),
        ("later tie", 0.5),
        ("equal best", 0.9),
    ] {
        output
            .take(row(name, Some(probability)))
            .expect("row accepted");
        let Mode::Ordered { held, .. } = &output.mode else {
            panic!("rank uses the ordered sink");
        };
        assert!(held.len() <= 2, "only top N payloads survive a callback");
    }
    output.ended().expect("the ranked run completed");
    assert_eq!(written, b"best\nequal best\n");
    assert_eq!(usage.run_snapshot().records, 5, "dropped rows still count");
}

#[test]
fn a_discarded_missing_probability_remains_an_end_only_defect() {
    let mut written = Vec::new();
    let usage = Counters::new(None);
    let mut output = Output::ordered(&mut written, Some(1), &usage);
    output.take(row("good", Some(0.9))).expect("first row");
    output
        .take(row("bad", None))
        .expect("defect remains deferred");
    assert!(matches!(
        output.ended(),
        Err(Failure::Defect("a ranked row carries no probability"))
    ));
    assert!(written.is_empty());
}
