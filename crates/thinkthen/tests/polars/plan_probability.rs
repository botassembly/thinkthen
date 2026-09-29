//! Same-call probability, per-call options, plan and null input positions.

use super::*;
use conformance_backend::{Canned, Listener};
use thinkthen::{Description, PolarsCallOptions};

#[test]
fn probability_frame_keeps_value_and_probability_in_one_call_and_refuses_other_kinds() {
    const DECIDE: &str = r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.8}}}"#;
    const CHOOSE: &str = r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"choice","choice":"billing","probabilities":{"billing":0.9,"outage":0.1}}}}"#;
    let listener = Listener::answering(|body| {
        if String::from_utf8_lossy(body).contains(r#""type":"choice""#) {
            Canned::ok(CHOOSE)
        } else {
            Canned::ok(DECIDE)
        }
    })
    .expect("listener");
    let engine = common::engine(listener.base());
    let text = common::column(&["Refund me."]);
    let plan = engine
        .plan_series(&decide(), &text, CallOptions::new())
        .expect("plan");
    assert_eq!((plan.records(), plan.requests()), (1, 1));
    assert_eq!(listener.count(), 0, "a frame plan sends nothing");
    let decided = engine
        .probability_frame(&decide(), &text, CallOptions::new())
        .expect("decide");
    assert_eq!(
        decided
            .value()
            .column("value")
            .expect("value")
            .bool()
            .expect("Boolean")
            .get(0),
        Some(true)
    );
    assert_eq!(
        decided
            .value()
            .column("probability")
            .expect("probability")
            .f64()
            .expect("Float64")
            .get(0),
        Some(0.8)
    );
    let choose = Question::choose_labels("Which team?")
        .expect("question")
        .label("billing", None)
        .expect("billing")
        .label("outage", None)
        .expect("outage")
        .build()
        .expect("choose");
    let chosen = engine
        .probability_frame(&choose, &text, CallOptions::new())
        .expect("choose");
    assert_eq!(
        chosen
            .value()
            .column("value")
            .expect("value")
            .str()
            .expect("String")
            .get(0),
        Some("billing")
    );
    assert_eq!(
        chosen
            .value()
            .column("probability")
            .expect("probability")
            .f64()
            .expect("Float64")
            .get(0),
        Some(0.9)
    );
    assert_eq!(
        listener.count(),
        2,
        "each value and probability used one reply"
    );
}

#[test]
fn call_level_meanings_change_the_body_and_score_tag_refuse_probability() {
    const DECIDE: &str = r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.8}}}"#;
    let listener = Listener::answering(|_| Canned::ok(DECIDE)).expect("listener");
    let engine = common::engine(listener.base());
    let text = common::column(&["Refund me."]);
    let yes = Description::text("The writer requests money back.").expect("yes meaning");
    let no = Description::text("The writer asks for something else.").expect("no meaning");
    let changed = engine
        .column_with(
            &decide(),
            &text,
            PolarsCallOptions::new()
                .threshold("0.9")
                .true_meaning(&yes)
                .false_meaning(&no)
                .probability(true),
        )
        .expect("call-level settings");
    assert_eq!(
        changed
            .value()
            .column("value")
            .expect("value")
            .bool()
            .expect("Boolean")
            .get(0),
        Some(false)
    );
    assert_eq!(
        changed
            .value()
            .column("probability")
            .expect("probability")
            .f64()
            .expect("Float64")
            .get(0),
        Some(0.8)
    );
    let last = listener.requests().pop().expect("first request");
    let body = String::from_utf8(last.body).expect("request UTF-8");
    assert!(body.contains(r#""criteria":{"true":"The writer requests money back.","false":"The writer asks for something else."}"#), "{body}");
    assert_eq!(listener.count(), 1);
    let tag = Question::tag_labels("Which tags?")
        .expect("tag")
        .label("billing", None)
        .expect("label")
        .build()
        .expect("tag question");
    for question in [&score(), &tag] {
        let error = engine
            .probability_frame(question, &text, CallOptions::new())
            .expect_err("no selected probability");
        assert_eq!(error.kind(), ErrorKind::Usage);
    }
    assert_eq!(
        listener.count(),
        1,
        "score and tag were refused before a send"
    );
}

#[test]
fn a_null_middle_row_keeps_its_place_without_becoming_a_question() {
    const BOTH: &str = r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.8},"q2":{"type":"noul","noul":0.2}}}"#;
    let listener = Listener::answering(|_| Canned::ok(BOTH)).expect("listener");
    let engine = common::engine(listener.base());
    let column = Series::new(
        "body".into(),
        [Some("Refund me."), None, Some("No problem.")],
    );
    let plan = engine
        .plan_series(&decide(), &column, CallOptions::new())
        .expect("plan");
    assert_eq!((plan.records(), plan.requests()), (2, 1));
    assert_eq!(listener.count(), 0);
    let out = engine
        .probability_frame(&decide(), &column, CallOptions::new())
        .expect("frame");
    let values = out
        .value()
        .column("value")
        .expect("values")
        .bool()
        .expect("Boolean");
    let probabilities = out
        .value()
        .column("probability")
        .expect("probabilities")
        .f64()
        .expect("Float64");
    assert_eq!(
        values.iter().collect::<Vec<_>>(),
        [Some(true), None, Some(false)]
    );
    assert_eq!(
        probabilities.iter().collect::<Vec<_>>(),
        [Some(0.8), None, Some(0.2)]
    );
    assert_eq!(listener.count(), 1, "two non-null rows shared one exchange");
}
