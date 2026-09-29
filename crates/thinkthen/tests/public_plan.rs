//! Public preview against two independent wire bodies from the command fixture.
#![allow(clippy::expect_used, reason = "a failed fixture stops this proof")]

use std::num::NonZeroUsize;

use conformance_backend::{Canned, Listener};
use thinkthen::{BatchSetting, CallOptions, Engine, Question};

#[test]
fn public_plan_reads_every_record_and_discloses_the_same_two_prepared_bodies_without_a_send() {
    const FIRST: &str = r#"{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \"alpha\". asks for a refund"},"q2":{"type":"noul","instructions":"The text is \"beta\". asks for a refund"}}}"#;
    const SECOND: &str = r#"{"state":"gamma","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"asks for a refund"}}}"#;
    assert_eq!((FIRST.len(), SECOND.len()), (248, 108));
    let listener =
        Listener::answering(|_| Canned::status(500, "should not send")).expect("listener");
    let engine = Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .no_cache()
        .build()
        .expect("keyless engine");
    let question = Question::decide("asks for a refund")
        .expect("question")
        .cut();
    let plan = engine
        .plan_with(
            &question,
            ["alpha", "beta", "gamma"],
            CallOptions::new().batch(BatchSetting::Records(NonZeroUsize::new(2).expect("two"))),
        )
        .expect("plan");
    assert_eq!(
        (plan.records(), plan.requests(), plan.estimated_bytes()),
        (3, 2, 356)
    );
    assert_eq!(plan.estimated_input_tokens(), (183, 324));
    assert!(!plan.upper_bound());
    assert_eq!(plan.first_body(), Some(FIRST.as_bytes()));
    assert_eq!(listener.count(), 0);
    let refused = engine.plan_with(
        &question,
        ["alpha", "  "],
        CallOptions::new().batch(BatchSetting::Records(NonZeroUsize::new(2).expect("two"))),
    );
    assert!(
        refused.is_err(),
        "a later invalid record refuses the whole preview"
    );
    assert_eq!(listener.count(), 0);
}
