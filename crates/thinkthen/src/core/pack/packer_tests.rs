use std::sync::Arc;

use super::{Entry, PackError, PackLimits, Packed, Packer};
use crate::core::backend_profile::{BackendProfile, LimitKind};
use crate::core::pack::State;

const MODEL: &str = "\"jev-1\"";

fn state(text: &str) -> State {
    State::new(serde_json::to_string(text).expect("json"), text.len())
}

fn entry(state: &State, question: &str, item: usize) -> Entry<usize> {
    Entry {
        state: state.clone(),
        question: Arc::from(format!(r#"{{"type":"noul","instructions":"{question}"}}"#)),
        options: 0,
        item,
    }
}

fn limits(inputs: usize) -> PackLimits {
    PackLimits {
        image_ceiling: None,
        ceiling: 96_000,
        profile: None,
        inputs,
        questions: None,
        strict_singleton: false,
        context: false,
    }
}

fn items(packed: &[Packed<usize>]) -> Vec<Vec<usize>> {
    packed.iter().map(|request| request.items.clone()).collect()
}

#[test]
fn a_request_closes_at_its_inputs_cap_and_at_a_new_state() {
    let (one, two) = (state("one"), state("two"));
    let mut packer = Packer::new(limits(2), MODEL.to_owned());
    let mut closed = Vec::new();
    packer
        .add(vec![entry(&one, "a", 0)], &mut closed)
        .expect("fits");
    packer
        .add(vec![entry(&one, "b", 1), entry(&one, "c", 2)], &mut closed)
        .expect("fits");
    packer
        .add(vec![entry(&one, "d", 3)], &mut closed)
        .expect("fits");
    packer
        .add(vec![entry(&two, "e", 4)], &mut closed)
        .expect("fits");
    closed.extend(packer.close());
    assert_eq!(items(&closed), [vec![0, 1, 2], vec![3], vec![4]]);
    assert!(!packer.is_open());
}

#[test]
fn the_counted_size_matches_the_joined_body_and_closes_before_the_ceiling() {
    let one = state("one");
    let alone = {
        let mut packer = Packer::new(limits(10), MODEL.to_owned());
        let mut closed = Vec::new();
        packer
            .add(vec![entry(&one, "a", 0)], &mut closed)
            .expect("fits");
        packer.close().expect("open").body.len()
    };
    let mut two = limits(10);
    two.ceiling = alone * 2;
    let mut packer = Packer::new(two, MODEL.to_owned());
    let mut closed = Vec::new();
    for item in 0..12 {
        packer
            .add(vec![entry(&one, "a", item)], &mut closed)
            .expect("fits");
    }
    closed.extend(packer.close());
    assert!(closed.iter().all(|request| request.body.len() <= alone * 2));
    assert!(
        closed
            .iter()
            .all(|request| request.items.len() == 1 || request.body.len() > alone)
    );
    let bodies: Vec<serde_json::Value> = closed
        .iter()
        .map(|request| serde_json::from_slice(&request.body).expect("json"))
        .collect();
    assert_eq!(bodies.len(), closed.len());
}

#[test]
fn one_input_over_the_ceiling_splits_with_its_state_repeated() {
    let one = state("one");
    let mut small = limits(10);
    small.ceiling = 130;
    let mut packer = Packer::new(small, MODEL.to_owned());
    let mut closed = Vec::new();
    packer
        .add(
            (0..4).map(|item| entry(&one, "question", item)).collect(),
            &mut closed,
        )
        .expect("each question fits alone");
    closed.extend(packer.close());
    assert!(closed.len() > 1, "{:?}", items(&closed));
    assert_eq!(
        closed
            .iter()
            .flat_map(|request| request.items.clone())
            .collect::<Vec<_>>(),
        [0, 1, 2, 3]
    );
}

#[test]
fn a_lone_question_over_a_profile_limit_is_refused_and_packs_nothing() {
    let profile = BackendProfile::parse(
        r#"{"schema":"thinkthen.backend-profile/1","name":"small","max_request_bytes":80}"#,
    )
    .expect("a profile");
    let one = state("one");
    let mut limited = limits(10);
    limited.profile = Some(profile);
    let mut packer = Packer::new(limited, MODEL.to_owned());
    let mut closed = Vec::new();
    let refused = packer
        .add(
            vec![entry(&one, "a", 0), entry(&one, &"x".repeat(80), 1)],
            &mut closed,
        )
        .expect_err("over the profile");
    let PackError::Profile(limit) = refused else {
        panic!("a profile refusal")
    };
    assert_eq!((limit.kind, limit.limit), (LimitKind::RequestBytes, 80));
    assert!(!packer.is_open() && closed.is_empty());
}

#[test]
fn a_context_refuses_at_the_ceiling_before_any_question() {
    let context = state(&"c".repeat(200));
    let mut shared = limits(10);
    shared.ceiling = 100;
    shared.context = true;
    let packer: Packer<usize> = Packer::new(shared, MODEL.to_owned());
    assert!(matches!(
        packer.check_state(&context),
        Err(PackError::Context {
            initial: true,
            kind: LimitKind::RequestBytes,
            limit: 100,
            ..
        })
    ));
}
