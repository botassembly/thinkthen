use super::{RelationEntity, RelationEntityView, RelationRule, plan};
use crate::core::recognize::RecognizedName;

fn entity(name: &str, kind: &str, start: usize) -> RecognizedName {
    RecognizedName {
        text: name.to_owned(),
        start,
        end: start + name.chars().count(),
        length: name.chars().count(),
        kind: kind.to_owned(),
        strength: 1.0,
    }
}

fn rule(source: &str, target: &str, either: bool) -> RelationRule {
    RelationRule {
        name: "linked_to".to_owned(),
        source: source.to_owned(),
        target: target.to_owned(),
        either,
        reads: "is linked to".to_owned(),
    }
}

#[test]
fn cross_kind_choice_asks_the_larger_side_with_smaller_side_options() {
    let entities = [
        entity("Ada", "person", 0),
        entity("Grace", "person", 4),
        entity("Acme", "organization", 10),
    ];
    let plans = plan(&entities, &rule("person", "organization", false)).expect("plan");
    let planned = plans.first().expect("concrete plan");
    assert_eq!(planned.questions.len(), 2);
    for question in &planned.questions {
        let crate::core::Question::Choose { options, .. } = question else {
            panic!("cross-kind planner did not choose");
        };
        assert_eq!(options.count(), 2);
    }
}

#[test]
fn same_kind_h_asks_unordered_or_ordered_pairs_and_never_self_pairs() {
    let entities = [
        entity("A", "person", 0),
        entity("B", "person", 2),
        entity("C", "person", 4),
    ];
    assert_eq!(
        plan(&entities, &rule("person", "person", true))
            .expect("plan")
            .first()
            .expect("concrete plan")
            .questions
            .len(),
        3
    );
    assert_eq!(
        plan(&entities, &rule("person", "person", false))
            .expect("plan")
            .first()
            .expect("concrete plan")
            .questions
            .len(),
        6
    );
    let either = plan(&entities, &rule("person", "person", true)).expect("plan");
    let question = either
        .first()
        .and_then(|planned| planned.questions.first())
        .expect("question");
    let crate::core::Question::Decide { text, yes, no } = question else {
        panic!("H question");
    };
    assert_eq!(
        text.as_json().as_str(),
        Some("Does the relation hold between i1 and i2?")
    );
    assert_eq!((yes, no), (&None, &None));
}

#[test]
fn wildcards_expand_to_concrete_kinds_in_first_seen_order() {
    let entities = [
        entity("Ada", "person", 0),
        entity("Acme", "organization", 4),
        entity("Grace", "person", 9),
    ];
    let plans = plan(&entities, &rule("*", "*", false)).expect("plan");
    let standalone = RelationEntity::new("Ada", "person").expect("entity");
    assert_eq!(standalone.name(), entities[0].name());
    assert_eq!(standalone.kind(), entities[0].kind());
    let kinds = plans
        .iter()
        .map(|plan| (plan.relation.source.as_str(), plan.relation.target.as_str()))
        .collect::<Vec<_>>();
    assert_eq!(
        kinds,
        [
            ("person", "person"),
            ("person", "organization"),
            ("organization", "person"),
            ("organization", "organization"),
        ]
    );
    assert!(plans.iter().all(|plan| plan.mappings.iter().all(|mapping| {
        !matches!(mapping, super::QuestionMap::Pair { source, target } if source == target)
    })));
}

#[test]
fn a_choice_puts_the_blank_on_the_side_the_options_fill() {
    let wording = |entities: &[RelationEntity]| {
        let plans = plan(entities, &rule("person", "organization", false)).expect("plan");
        let crate::core::Question::Choose { text, .. } = &plans[0].questions[0] else {
            panic!("cross-kind planner did not choose");
        };
        text.as_json().as_str().expect("text").to_owned()
    };
    let one = |name: &str, kind: &str| RelationEntity::new(name, kind).expect("entity");
    assert_eq!(
        wording(&[
            one("Ada", "person"),
            one("Grace", "person"),
            one("Acme", "organization")
        ]),
        "Which listed organization fills the blank: Item 1 (person \"Ada\") is linked to ___? \
         Choose none if no listed organization does."
    );
    assert_eq!(
        wording(&[
            one("Ada", "person"),
            one("Acme", "organization"),
            one("Beta", "organization")
        ]),
        "Which listed person fills the blank: ___ is linked to Item 2 (organization \"Acme\")? \
         Choose none if no listed person does."
    );
}
