//! What the SQL hosts take from the public API instead of copying engine code
//! (ticket 0347): per-engine usage and its sum, the inline relation parser, a
//! loaded question asked directly, and a host's own refusal as `Error`.
#![allow(
    clippy::expect_used,
    reason = "a failed loopback fixture stops this proof"
)]

use conformance_backend::{Canned, Listener};
use thinkthen::{
    Counters, Engine, Error, ErrorKind, LoadedQuestion, Question, QuestionKind, RelationRule,
};

const YES: &str = r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":12,"output_tokens":3}}"#;

fn engine(listener: &Listener) -> Engine {
    Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .api_key("sk-test")
        .expect("key")
        .no_cache()
        .build()
        .expect("engine")
}

/// Two engines in one process count apart; clones share; `Sum` adds them.
#[test]
fn each_engine_counts_its_own_calls_and_counters_sum() {
    let listener = Listener::answering(|_| Canned::ok(YES)).expect("listener");
    let (first, second) = (engine(&listener), engine(&listener));
    let question = Question::decide("asks for a refund")
        .expect("question")
        .cut();
    first.decide(&question, "Refund me.").expect("first");
    first
        .clone()
        .decide(&question, "Refund me now.")
        .expect("clone");
    second.decide(&question, "Refund me.").expect("second");
    let read = |counts: Counters| {
        (
            counts.requests_sent(),
            counts.input_tokens(),
            counts.output_tokens(),
        )
    };
    assert_eq!(read(first.usage()), (2, 24, 6));
    assert_eq!(read(second.usage()), (1, 12, 3));
    let total: Counters = [first.usage(), second.usage()].into_iter().sum();
    assert_eq!(read(total), (3, 36, 9));
    assert_eq!(first.usage() + second.usage(), total);
    assert_eq!(read(std::iter::empty::<Counters>().sum()), (0, 0, 0));
    assert_eq!(listener.count(), 3);
}

/// The command's `--relation` spellings, and its refusal for every other one.
#[test]
fn inline_relation_rules_read_as_the_command_reads_them() {
    for (text, name, source, target) in [
        ("caused_by", "caused_by", "*", "*"),
        (
            "works_for=person:organization",
            "works_for",
            "person",
            "organization",
        ),
        (
            "works_for=ANY:organization",
            "works_for",
            "*",
            "organization",
        ),
    ] {
        let rule = RelationRule::parse_inline(text, false).expect(text);
        assert_eq!(
            (rule.name(), rule.source(), rule.target()),
            (name, source, target)
        );
    }
    for bad in [
        "a:b", "a=b", "a=b:c:d", "a=b=c:d", "a=:b", "a=b:", "=b:c", " ", "", "a\u{7}",
    ] {
        let error = RelationRule::parse_inline(bad, true).expect_err(bad);
        assert_eq!(
            (error.kind(), error.detail().message()),
            (
                ErrorKind::Usage,
                "a relate relation is NAME=SOURCE_KIND:TARGET_KIND, or a bare NAME"
            ),
            "{bad:?}"
        );
    }
}

/// A loaded question, banded or not, is asked with no match on its arms.
#[test]
fn a_loaded_question_is_asked_directly() {
    let listener =
        Listener::answering(|_| Canned::status(500, "should not send")).expect("listener");
    let engine = engine(&listener);
    let banded = Question::from_json(r#"{"decide":"asks for a refund","threshold":"0.2:0.8"}"#)
        .expect("banded");
    assert!(matches!(banded, LoadedQuestion::Banded(_)));
    let choose =
        Question::from_json(r#"{"choose":"Which team?","options":["billing","shipping"]}"#)
            .expect("choose");
    assert_eq!(
        (banded.kind(), choose.kind()),
        (QuestionKind::Decide, QuestionKind::Choose)
    );
    for loaded in [&banded, &choose] {
        let plan = engine.plan(loaded, ["Refund me."]).expect("plan");
        assert_eq!((plan.records(), plan.requests()), (1, 1));
    }
    let refused = engine
        .decide(&choose, "Refund me.")
        .expect_err("decide takes no choose question");
    assert_eq!(
        refused.detail().message(),
        "decide does not take a choose question"
    );
    assert_eq!(listener.count(), 0);
}

/// A host raises its own refusal as the one public error.
#[test]
fn a_host_builds_its_refusal_as_the_public_error() {
    let error = Error::new(ErrorKind::Local, "the named file did not read");
    assert_eq!(
        (error.kind(), error.detail().message(), error.retryable()),
        (ErrorKind::Local, "the named file did not read", false)
    );
    assert!(error.facts().is_none() && error.send_budget_denial().is_none());
}
