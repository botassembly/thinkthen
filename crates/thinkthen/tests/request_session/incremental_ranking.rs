//! Ordinary rank can preserve incremental sends without changing default admission.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "isolated public session fixtures stop on malformed setup"
)]
use super::*;

fn rank(input: RequestInput) -> Request {
    Request::for_function(
        RequestFunction::Rank,
        RequestArguments {
            question: RequestQuestion::Definition {
                value: Question::rank("Relevant?").unwrap().into(),
            },
            input,
            options: RequestOptions {
                batch: Some(RequestBatch::Count(1)),
                ..Default::default()
            },
        },
    )
}

#[test]
fn rank_defaults_to_whole_set_admission_and_can_preserve_prior_sends() {
    for incremental in [false, true] {
        let listener = Listener::answering(response).unwrap();
        let session = engine(&listener)
            .request_session_with_feed_options(
                rank(feed()),
                Surface::Rust,
                RequestSessionFeedOptions {
                    incremental_ranking: incremental,
                    ..Default::default()
                },
            )
            .unwrap();
        framing::push(&session, descriptor("first"));
        framing::push(&session, descriptor(" "));
        session.finish(None).unwrap();
        let packets = drain(&session);
        assert!(!packets.iter().any(|packet| matches!(
            packet,
            RequestSessionResult::Row(_) | RequestSessionResult::Aggregate(_)
        )));
        let Some(RequestSessionResult::Terminal(terminal)) = packets.last() else {
            panic!("terminal")
        };
        assert_eq!(terminal.error.as_ref().unwrap().kind(), ErrorKind::Usage);
        assert_eq!(listener.count(), usize::from(incremental));
        if incremental {
            assert_eq!(terminal.facts.as_ref().unwrap().requests_sent(), 1);
        }
    }
}

#[test]
fn incremental_ranking_refuses_other_functions_and_recovery_before_source_access() {
    let listener = Listener::answering(response).unwrap();
    let source = || RequestInput::Source {
        source: RequestSource {
            paths: vec!["absent-incremental-rank-source".into()],
            framing: None,
            reading: ReaderOptions::default(),
            media: ReaderMedia::Text,
        },
    };
    let options = || RequestSessionFeedOptions {
        incremental_ranking: true,
        ..Default::default()
    };
    let engine = engine(&listener);
    for error in [
        engine
            .request_session_with_feed_options(request(source()), Surface::Rust, options())
            .unwrap_err(),
        engine
            .request_recovering_session_with_feed_options(rank(source()), Surface::Rust, options())
            .unwrap_err(),
    ] {
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert_eq!(
            error.to_string(),
            "incremental ranking requires an ordinary rank session"
        );
    }
    assert_eq!(listener.count(), 0);
}
