use super::{
    QuestionMap, RelationEdge, RelationEntity, RelationRule, plan, plan_pairs, relation_evidence,
};
use crate::core::adapters::built_in;
use crate::core::recognize::RecognizedName;
use crate::core::{ModelName, Plan};

fn entity(name: &str, kind: &str, start: usize) -> RecognizedName {
    RecognizedName {
        name: name.to_owned(),
        kind: kind.to_owned(),
        start,
        end: start + name.chars().count(),
        strength: 1.0,
    }
}

#[test]
fn fixed_option_boundary_keeps_255_and_falls_back_at_256() {
    let rule = RelationRule {
        name: "linked_to".to_owned(),
        source: "person".to_owned(),
        target: "organization".to_owned(),
        either: false,
        reads: "is linked to".to_owned(),
    };
    for (target_count, expected_choice) in [(254, true), (255, false)] {
        let mut entities = (0..256)
            .map(|place| entity(&format!("P{place}"), "person", place))
            .collect::<Vec<_>>();
        entities.extend(
            (0..target_count)
                .map(|place| entity(&format!("O{place}"), "organization", place + 256)),
        );
        let plans = plan(&entities, &rule).expect("plan");
        let planned = plans.first().expect("concrete plan");
        assert_eq!(
            matches!(
                planned.questions.first(),
                Some(crate::core::Question::Choose { .. })
            ),
            expected_choice
        );
    }
}

#[test]
fn generic_edges_serialize_and_cross_kind_either_h_keeps_the_pair() {
    let entities = [
        entity("Acme", "organization", 0),
        entity("Ada", "person", 5),
    ];
    let rule = RelationRule {
        name: "partner".to_owned(),
        source: "person".to_owned(),
        target: "organization".to_owned(),
        reads: "partners with".to_owned(),
        either: true,
    };
    let planned = plan_pairs(&entities, &rule).expect("H plan");
    assert_eq!(
        planned.mappings,
        [QuestionMap::Pair {
            source: 0,
            target: 1
        }]
    );
    let edge = RelationEdge {
        relation: "partner".to_owned(),
        source: RelationEntity::new("Acme", "organization").expect("source"),
        target: RelationEntity::new("Ada", "person").expect("target"),
        probability: 0.5,
    };
    assert_eq!(
        crate::core::json_line(&edge).expect("edge JSON"),
        r#"{"relation":"partner","source":{"name":"Acme","kind":"organization"},"target":{"name":"Ada","kind":"person"},"probability":0.5}"#
    );
}

#[test]
fn exact_relation_state_and_h_wording_use_stable_ids() {
    let entities = [entity("Ada", "person", 0), entity("Grace", "person", 4)];
    let rule = RelationRule {
        name: "linked_to".to_owned(),
        source: "person".to_owned(),
        target: "person".to_owned(),
        either: false,
        reads: "is linked to".to_owned(),
    };
    let plans = plan(&entities, &rule).expect("plan");
    let planned = plans.first().expect("concrete plan");
    let evidence = relation_evidence(Some("Ada met Grace."), &entities, &planned.relation)
        .expect("relation evidence");
    let request = built_in::encode(
        &Plan::new(
            evidence,
            ModelName::new("local-1").expect("model"),
            planned.questions.clone(),
        )
        .expect("request plan"),
    )
    .expect("request");
    assert_eq!(
        String::from_utf8(request).expect("UTF-8"),
        r#"{"state":{"evidence":"Ada met Grace.","entities":[{"id":"i1","name":"Ada","kind":"person"},{"id":"i2","name":"Grace","kind":"person"}],"relation":{"name":"linked_to","source":"person","target":"person","reads":"is linked to","either":false}},"model":"local-1","questions":{"q1":{"type":"noul","instructions":"Does the relation hold from i1 to i2?"},"q2":{"type":"noul","instructions":"Does the relation hold from i2 to i1?"}}}"#
    );
    let mut either = rule;
    either.either = true;
    let planned = plan(&entities, &either)
        .expect("either plan")
        .into_iter()
        .next()
        .expect("concrete plan");
    let evidence = relation_evidence(Some("Ada met Grace."), &entities, &planned.relation)
        .expect("relation evidence");
    let request = built_in::encode(
        &Plan::new(
            evidence,
            ModelName::new("local-1").expect("model"),
            planned.questions,
        )
        .expect("request plan"),
    )
    .expect("request");
    assert_eq!(
        String::from_utf8(request).expect("UTF-8"),
        r#"{"state":{"evidence":"Ada met Grace.","entities":[{"id":"i1","name":"Ada","kind":"person"},{"id":"i2","name":"Grace","kind":"person"}],"relation":{"name":"linked_to","source":"person","target":"person","reads":"is linked to","either":true}},"model":"local-1","questions":{"q1":{"type":"noul","instructions":"Does the relation hold between i1 and i2?"}}}"#
    );
    let standalone = [
        RelationEntity::new("Ada", "person").expect("entity"),
        RelationEntity::new("Grace", "person").expect("entity"),
    ];
    assert_eq!(
        relation_evidence(None, &standalone, &either)
            .expect("standalone state")
            .as_text()
            .expect("state JSON"),
        r#"{"entities":[{"id":"i1","name":"Ada","kind":"person"},{"id":"i2","name":"Grace","kind":"person"}],"relation":{"name":"linked_to","source":"person","target":"person","reads":"is linked to","either":true}}"#
    );
}
