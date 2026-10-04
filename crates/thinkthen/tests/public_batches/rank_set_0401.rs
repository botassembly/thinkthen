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
fn one_member_matches_single_rank_body_answer_and_facts() {
    let _serial = serial();
    let listener = Listener::answering(answer).expect("listener");
    let engine = engine(listener.base());
    let question = Question::rank("First?").expect("rank");
    let plain = engine.rank(&question, ["a", "b", "c"]).expect("plain");
    let bodies = listener.requests();
    let set =
        RankSet::from_json(r#"{"version":1,"questions":{"private_name":{"decide":"First?"}}}"#)
            .expect("set");
    let ranked = engine.rank_set(&set, ["a", "b", "c"]).expect("set rank");
    assert_eq!(
        ranked
            .value()
            .iter()
            .map(|row| (row.index(), row.input(), row.probability()))
            .collect::<Vec<_>>(),
        plain
            .value()
            .iter()
            .map(|row| (row.index(), row.input(), row.probability()))
            .collect::<Vec<_>>()
    );
    assert_eq!(
        listener
            .requests()
            .iter()
            .map(|request| &request.body)
            .collect::<Vec<_>>(),
        bodies
            .iter()
            .map(|request| &request.body)
            .collect::<Vec<_>>()
    );
    assert_eq!(ranked.facts().records(), plain.facts().records());
    assert!(!format!("{set:?}").contains("private_name"));
    assert!(!format!("{ranked:?}").contains("private_name"));
}
