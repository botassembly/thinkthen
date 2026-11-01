#![cfg(test)]
//! Rank retains the original call fault when an input producer fails.
use thinkthen::{Code, Rank, Record, Settings};

#[test]
fn rank_preserves_a_call_failure_instead_of_cancellation() {
    let engine = Settings::from_json(r#"{"backend":"stub"}"#)
        .unwrap()
        .engine(|_| None)
        .unwrap();
    let question = Rank::new("Priority?", ["low", "high"]).unwrap();
    let mut records = 0;
    let input = std::iter::from_fn(move || {
        records += 1;
        assert!(records == 1, "input producer failed");
        Some(Record::text("evidence"))
    });
    let mut rows = engine.rows(&question, input).unwrap();
    let failure = rows.next().unwrap().unwrap_err();
    assert_eq!(failure.code(), Code::InternalBug);
    assert_eq!(failure.message(), "the call worker failed");
    assert!(rows.next().is_none(), "one terminal failure ends the call");
}
