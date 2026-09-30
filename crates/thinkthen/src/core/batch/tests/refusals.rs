use super::{
    BUILT_IN, BatchError, BatchRecord, Batcher, Closed, Evidence, LimitKind, Setting, backend,
    decide, loopback, profile, run, shape, structured, team_plan, text,
};

/// The batches a batcher closes before its first refusal, and that refusal.
fn refusal(batcher: &mut Batcher, records: Vec<BatchRecord>) -> (Vec<(usize, Closed)>, BatchError) {
    let mut closed = Vec::new();
    let error = records
        .into_iter()
        .find_map(|record| batcher.push(record, &mut closed).err())
        .expect("a refusal");
    (shape(&closed), error)
}

#[test]
fn a_record_over_the_profile_alone_is_refused_on_its_own_push() {
    let urgent = || text("Help! My payouts have been failing for 3 days.");
    let refused = |limits: &str, question, records| {
        let mut batcher = Batcher::new(loopback(), profile(limits), question, Setting::Max, None)
            .expect("batcher");
        let (closed, error) = refusal(&mut batcher, records);
        let BatchError::Profile(over) = error else {
            panic!("a profile refusal")
        };
        let next = batcher.push(text("b"), &mut Vec::new()).is_ok();
        (closed, (over.kind, over.limit, over.actual), next)
    };
    let cases = [
        (
            r#""max_request_bytes":150"#,
            decide("Q"),
            vec![urgent()],
            vec![],
            (LimitKind::RequestBytes, 150, 195),
            true,
        ),
        (
            r#""max_options":3"#,
            team_plan().questions().first().cloned().expect("choose"),
            vec![text("x")],
            vec![],
            (LimitKind::Options, 3, 4),
            false,
        ),
        (
            r#""max_request_bytes":150"#,
            decide("Q"),
            vec![text("a"), urgent()],
            vec![(1, Closed::Limit)],
            (LimitKind::RequestBytes, 150, 195),
            true,
        ),
    ];
    for (limits, question, records, closed, over, next) in cases {
        assert_eq!(refused(limits, question, records), (closed, over, next));
    }
}

#[test]
fn a_context_is_refused_without_echoing_it() {
    let beside = |profile, question| {
        let context = Evidence::new("secret context").expect("context");
        Batcher::new(loopback(), profile, question, Setting::Max, Some(context))
    };
    let refused = beside(None, structured()).err();
    assert_eq!(refused, Some(BatchError::StructuredQuestionWithContext));
    let refused = beside(profile(r#""max_evidence_bytes":5"#), decide("Q")).err();
    let over = BatchError::ContextOverLimit {
        kind: LimitKind::EvidenceBytes,
        limit: 5,
        actual: 14,
    };
    assert_eq!(refused, Some(over));
    let huge = Evidence::new("x".repeat(100_000)).expect("context");
    let built_in = backend(BUILT_IN, "jev-latest");
    let refused = Batcher::new(built_in, None, decide("Q"), Setting::Max, Some(huge)).err();
    let over = BatchError::ContextOverLimit {
        kind: LimitKind::RequestBytes,
        limit: 96_000,
        actual: 100_048,
    };
    assert_eq!(refused, Some(over));
    let batcher = beside(profile(r#""max_request_bytes":200"#), decide("Q")).expect("batcher");
    let error =
        run(batcher, vec![text("short"), text(&"secret ".repeat(40))]).expect_err("late overflow");
    let limit = (LimitKind::RequestBytes, 200);
    assert!(
        matches!(error, BatchError::ContextOverLimit { kind, limit: most, .. } if (kind, most) == limit)
    );
    assert!(!format!("{error:?} {error}").contains("secret"));
}
