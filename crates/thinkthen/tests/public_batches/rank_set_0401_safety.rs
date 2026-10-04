//! Whole-input preflight, whole-call failure, cancellation and shared budgets.
use super::rank_set_0401::{SET, answer};
use super::*;
use thinkthen::RankSet;

#[test]
fn blank_later_record_and_record_limit_fail_before_counted_sends() {
    let _serial = serial();
    let listener = Listener::answering(answer).expect("listener");
    let engine = engine(listener.base());
    let set = RankSet::from_json(SET).expect("set");
    let error = engine
        .rank_set(&set, ["a", "b", " \t"])
        .expect_err("blank later record");
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(error.to_string(), "evidence is text, not white space");
    let limited = Engine::builder()
        .base_url(listener.base())
        .expect("url")
        .api_key("sk-public-batches")
        .expect("key")
        .max_requests(Some(2))
        .expect("limit")
        .no_cache()
        .build()
        .expect("limited engine");
    let error = limited
        .rank_set(&set, ["a", "b", "c"])
        .expect_err("limit preflight");
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(
        error.to_string(),
        "this engine answers at most 2 records in one call"
    );
    assert_eq!(listener.count(), 0);
}

#[test]
fn invalid_authored_rank_settings_and_debug_names_do_not_escape() {
    for (json, message) in [
        (
            r#"{"version":1,"threshold":0.5,"questions":{"private_name":{"decide":"secret wording"}}}"#,
            "`threshold`: rank question sets take no threshold or on",
        ),
        (
            r#"{"version":1,"questions":{"private_name":{"decide":"secret wording","on":""}}}"#,
            "`questions.private_name.on`: rank question sets take no threshold or on",
        ),
        (
            r#"{"version":1,"questions":{"private_name":{"decide":"secret wording","threshold":0.5}}}"#,
            "`questions.private_name.threshold`: rank question sets take no threshold or on",
        ),
        (
            r#"{"version":1,"questions":{"private_name":{"score":"secret wording","levels":["low","high"]}}}"#,
            "`questions.private_name`: rank takes only decide questions",
        ),
    ] {
        let error = RankSet::from_json(json).expect_err("authored forbidden setting");
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert_eq!(error.to_string(), message);
        assert!(!format!("{error:?}").contains("secret wording"));
    }
    let place = std::env::temp_dir().join(format!("rank-set-load-{}", std::process::id()));
    std::fs::create_dir_all(&place).expect("owned folder");
    let path = place.join("question.json");
    std::fs::write(&path, "{}").expect("invalid file");
    assert_eq!(
        RankSet::load(&path).expect_err("file grammar").kind(),
        ErrorKind::Local
    );
    std::fs::write(&path, " ".repeat(1_048_577)).expect("oversize file");
    assert_eq!(
        RankSet::load(&path).expect_err("capped load").kind(),
        ErrorKind::Local
    );
}

#[test]
fn any_failed_member_refuses_the_whole_call_after_question_observations() {
    let _serial = serial();
    let listener = Listener::answering(|body| {
        if String::from_utf8_lossy(body).contains("late") {
            Canned::ok(r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9}}}"#)
        } else {
            answer(body)
        }
    })
    .expect("listener");
    let engine = engine(listener.base());
    let set = RankSet::from_json(SET).expect("set");
    let seen = Mutex::new(Vec::new());
    let observe = |event: RecordObservation<'_>| {
        if let RecordObservation::Question {
            index,
            member,
            detail,
            ..
        } = event
        {
            seen.lock().expect("seen").push((
                index,
                member.map(str::to_owned),
                detail.failure().is_some(),
            ));
        }
    };
    let options = CallOptions::new()
        .batch(BatchSetting::Records(std::num::NonZeroUsize::MIN))
        .observe(&observe);
    let error = engine
        .rank_set_with(&set, ["a", "late secret evidence"], options)
        .expect_err("failed member");
    assert_eq!(error.kind(), ErrorKind::Backend);
    assert_eq!(listener.count(), 2);
    assert_eq!(
        error
            .facts()
            .map(|facts| (facts.records(), facts.requests_sent())),
        Some((1, 2))
    );
    assert_eq!(
        *seen.lock().expect("seen"),
        [
            (0, Some("first".to_owned()), false),
            (0, Some("second".to_owned()), false),
            (1, Some("first".to_owned()), false),
            (1, Some("second".to_owned()), true)
        ]
    );
    assert!(!format!("{error:?}").contains("secret evidence"));
}

#[test]
fn cancellation_in_a_member_observer_stops_the_whole_call_and_joins_workers() {
    let _serial = serial();
    let listener = Listener::answering(answer).expect("listener");
    let engine = engine(listener.base());
    let set = RankSet::from_json(SET).expect("set");
    let token = CancelToken::new();
    let observe = |event: RecordObservation<'_>| {
        if matches!(
            event,
            RecordObservation::Question {
                index: 0,
                member: Some("first"),
                ..
            }
        ) {
            token.cancel();
        }
    };
    let options = CallOptions::new()
        .batch(BatchSetting::Records(std::num::NonZeroUsize::MIN))
        .cancel(&token)
        .observe(&observe);
    let error = engine
        .rank_set_with(&set, ["a", "b", "c"], options)
        .expect_err("cancel whole set");
    assert_eq!(error.kind(), ErrorKind::Cancelled);
    assert_eq!(listener.count(), 1);
    assert_eq!(error.facts().map(|facts| facts.requests_sent()), Some(1));
    // A subsequent call on the same engine proves the cancelled workers and
    // width reservation have completed before the failed call returns.
    let again = engine
        .rank_set(&set, ["a"])
        .expect("joined engine reusable");
    assert_eq!(again.facts().records(), 1);
    assert_eq!(listener.count(), 2);
}

#[test]
fn one_budget_spans_all_member_requests_and_retains_final_error_facts() {
    let _serial = serial();
    let listener = Listener::answering(answer).expect("listener");
    let engine = engine(listener.base());
    let set = RankSet::from_json(SET).expect("set");
    let options = CallOptions::new()
        .batch(BatchSetting::Records(std::num::NonZeroUsize::MIN))
        .max_requests_total(Some(thinkthen::process_requests_sent() + 1));
    let error = engine
        .rank_set_with(&set, ["a", "b"], options)
        .expect_err("shared send budget");
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(listener.count(), 1);
    assert_eq!(
        error
            .facts()
            .map(|facts| (facts.records(), facts.requests_sent())),
        Some((1, 1))
    );
}
