//! Additive eager API ownership, independent turns and combined observations.
use super::*;
use serde_json::{Value, json};
use thinkthen::{Call, Evidence, RankSet, SetRanked};

pub(super) const SET: &str =
    r#"{"version":1,"questions":{"first":{"decide":"First?"},"second":{"decide":"Second?"}}}"#;

pub(super) fn answer(body: &[u8]) -> Canned {
    let body: Value = serde_json::from_slice(body).expect("request JSON");
    let answers = body
        .get("questions")
        .expect("questions key")
        .as_object()
        .expect("questions")
        .iter()
        .map(|(key, question)| {
            let text = question
                .get("instructions")
                .expect("instructions key")
                .as_str()
                .expect("instructions");
            let probability = match (
                text.contains("Second?"),
                text.contains("\"a\""),
                text.contains("\"b\""),
            ) {
                (false, true, _) => 0.7,
                (false, _, true) => 0.6,
                (false, _, _) => 0.5,
                (true, true, _) => 1.0,
                (true, _, true) => 0.98,
                (true, _, _) => 0.99,
            };
            (
                key.clone(),
                json!({"type":"noul","noul":probability,"confidence":0.01}),
            )
        })
        .collect::<serde_json::Map<_, _>>();
    Canned::ok(&json!({"model":"jev-latest","answers":answers,"usage":{"input_tokens":6,"output_tokens":2}}).to_string())
}

#[test]
fn original_items_move_once_and_member_order_and_attribution_are_independent() {
    let _serial = serial();
    // No Clone, Send or Serialize implementation; Rc forces caller ownership.
    struct Original {
        id: usize,
        text: &'static str,
        drops: std::rc::Rc<std::cell::Cell<usize>>,
    }
    impl Evidence for Original {
        fn evidence(&self) -> &str {
            self.text
        }
    }
    impl Drop for Original {
        fn drop(&mut self) {
            self.drops.set(self.drops.get() + 1);
        }
    }
    let listener = Listener::answering(answer).expect("listener");
    let engine = engine(listener.base());
    let set = RankSet::from_json(SET).expect("rank set");
    let drops = std::rc::Rc::new(std::cell::Cell::new(0));
    let items = ["a", "b", "c"]
        .into_iter()
        .enumerate()
        .map(|(id, text)| Original {
            id,
            text,
            drops: drops.clone(),
        });
    let call = engine.rank_set(&set, items).expect("rank set call");
    assert!(!format!("{call:?}").contains("first"));
    assert_eq!(
        call.value()
            .iter()
            .map(|row| (
                row.index(),
                row.input().id,
                row.question_name(),
                row.probability()
            ))
            .collect::<Vec<_>>(),
        [
            (0, 0, "first", 0.7),
            (1, 1, "first", 0.6),
            (2, 2, "second", 0.99)
        ]
    );
    assert_eq!(call.facts().records(), 3);
    assert_eq!(drops.get(), 0);
    for row in call.into_value() {
        assert!(!format!("{row:?}").contains(row.question_name()));
        let item = row.into_input();
        assert!(item.id < 3);
    }
    assert_eq!(drops.get(), 3);
}

#[test]
fn member_questions_keep_receipts_and_n_record_facts_and_one_row_event_per_original() {
    let _serial = serial();
    let listener = Listener::answering(answer).expect("listener");
    let engine = engine(listener.base());
    let set = RankSet::from_json(SET).expect("set");
    let seen = Mutex::new(Vec::new());
    let row_indexes = Mutex::new(Vec::new());
    let observe = |event: RecordObservation<'_>| {
        assert!(!format!("{event:?}").contains("first"));
        match event {
            RecordObservation::Question { .. } => seen
                .lock()
                .expect("seen")
                .push(serde_json::to_value(event).expect("question event")),
            RecordObservation::Row { index, .. } => {
                row_indexes.lock().expect("indexes").push(index)
            }
        }
    };
    let result: Call<Vec<SetRanked<&str>>> = engine
        .rank_set_with(
            &set,
            ["a", "b", "c"],
            CallOptions::new()
                .batch(BatchSetting::Max)
                .observe(&observe),
        )
        .expect("call");
    assert_eq!(result.facts().records(), 3);
    assert_eq!(result.facts().requests_sent(), 1);
    assert_eq!(
        (
            result.facts().input_tokens(),
            result.facts().output_tokens()
        ),
        (Some(6), Some(2))
    );
    assert_eq!(*row_indexes.lock().expect("indexes"), [0, 1, 2]);
    let seen = seen.lock().expect("seen");
    assert_eq!(
        seen.iter()
            .map(|row| (
                row["index"].as_u64(),
                row["member"].as_str(),
                row["probabilities"].as_f64()
            ))
            .collect::<Vec<_>>(),
        [
            (Some(0), Some("first"), Some(0.7)),
            (Some(0), Some("second"), Some(1.0)),
            (Some(1), Some("first"), Some(0.6)),
            (Some(1), Some("second"), Some(0.98)),
            (Some(2), Some("first"), Some(0.5)),
            (Some(2), Some("second"), Some(0.99))
        ]
    );
    assert_eq!(
        seen.iter()
            .map(|row| row["requests"].as_array().expect("keys").len())
            .collect::<Vec<_>>(),
        [1; 6]
    );
    assert_eq!(listener.count(), 1);
}

#[test]
fn reversed_members_use_literal_turns_and_equal_text_keeps_two_positions() {
    let _serial = serial();
    let listener = Listener::answering(answer).expect("listener");
    let engine = engine(listener.base());
    let set = RankSet::from_json(
        r#"{"version":1,"questions":{"second":{"decide":"Second?"},"first":{"decide":"First?"}}}"#,
    )
    .expect("set");
    let result = engine.rank_set(&set, ["a", "b", "c"]).expect("call");
    assert_eq!(
        result
            .value()
            .iter()
            .map(|row| (row.index(), row.question_name(), row.probability()))
            .collect::<Vec<_>>(),
        [(0, "second", 1.0), (2, "second", 0.99), (1, "first", 0.6)]
    );
    let result = engine
        .rank_set(&set, ["a", "a", "b"])
        .expect("equal originals");
    assert_eq!(
        result
            .value()
            .iter()
            .map(SetRanked::index)
            .collect::<Vec<_>>(),
        [0, 1, 2]
    );
}

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
    let valid =
        RankSet::from_json(r#"{"version":1,"questions":{"private_name":{"decide":"First?"}}}"#)
            .expect("valid set");
    assert!(!format!("{valid:?}").contains("private_name"));
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
