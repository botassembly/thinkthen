//! Public preview against two independent wire bodies from the command fixture.
#![allow(clippy::expect_used, reason = "a failed fixture stops this proof")]

use std::num::NonZeroUsize;

use conformance_backend::{Canned, Listener};
use thinkthen::{BatchSetting, CallOptions, Engine, ErrorKind, Question};

#[test]
fn public_plan_reads_every_record_and_discloses_the_same_two_prepared_bodies_without_a_send() {
    const FIRST: &str = r#"{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \"alpha\". asks for a refund"},"q2":{"type":"noul","instructions":"The text is \"beta\". asks for a refund"}}}"#;
    const SECOND: &str = r#"{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \"gamma\". asks for a refund"}}}"#;
    assert_eq!((FIRST.len(), SECOND.len()), (248, 170));
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
        (3, 2, 418)
    );
    assert_eq!(plan.estimated_input_tokens(), (215, 381));
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

#[test]
fn plan_refuses_nonordinary_questions_before_disclosing_a_body_or_sending() {
    let listener =
        Listener::answering(|_| Canned::status(500, "should not send")).expect("listener");
    let engine = Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .no_cache()
        .build()
        .expect("keyless engine");
    let cases = [
        (Question::rank("Rank these?").expect("rank"), "rank"),
        (Question::find("Find one?").expect("find"), "find"),
        (
            Question::find("Find one?")
                .expect("find")
                .offering_none()
                .expect("none"),
            "findnone",
        ),
    ];
    for (question, kind) in cases {
        let error = engine
            .plan(&question, ["private input"])
            .expect_err("this kind has no ordinary details_many plan");
        assert_eq!(error.kind(), ErrorKind::Usage, "{kind}");
        assert_eq!(
            error.detail().message(),
            format!("plan does not take a {kind} question")
        );
        assert!(!format!("{error:?}").contains("private input"));
        assert_eq!(listener.count(), 0, "{kind} disclosed no request body");
    }
}
