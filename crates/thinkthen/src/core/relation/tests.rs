use serde::Deserialize;

use super::{Lead, RelationEntity, RelationRule, pair_edges, plan_pairs};
use crate::core::adapters::built_in;
use crate::core::probability::Probability;
use crate::core::{Answer, ModelName, Plan};

struct Case {
    name: &'static str,
    entities: &'static [(&'static str, &'static str)],
    rules: &'static [(&'static str, &'static str, &'static str, bool)],
    lead: Lead,
    text: Option<&'static str>,
    expected: &'static [(&'static str, usize, usize, usize)],
    state_names: &'static [&'static str],
}

#[rustfmt::skip]
const CASES: &[Case] = &[
    Case { name: "cross-kind either", entities: &[("Acme","organization"),("Ada","person")], rules: &[("knows","person","organization",true)], lead: Lead::Known, text: None,
        expected: &[("Is it true that i1 knows i2, or that i2 knows i1?",0,0,1)], state_names: &["Acme","Ada"] },
    Case { name: "two rules share one state", entities: &[("Ada","person"),("S1","song"),("S2","song")], rules: &[("sang","person","song",false),("wrote","person","song",false)], lead: Lead::Known, text: None,
        expected: &[("Is it true that i1 sang i2?",0,0,1),("Is it true that i1 sang i3?",0,0,2),("Is it true that i1 wrote i2?",1,0,1),("Is it true that i1 wrote i3?",1,0,2)], state_names: &["Ada","S1","S2"] },
    Case { name: "directed wildcard", entities: &[("Ada","person"),("Acme","organization"),("Bob","person")], rules: &[("knows","*","*",false)], lead: Lead::Known, text: None,
        expected: &[("Is it true that i1 knows i2?",0,0,1),("Is it true that i1 knows i3?",0,0,2),("Is it true that i2 knows i1?",0,1,0),("Is it true that i2 knows i3?",0,1,2),("Is it true that i3 knows i1?",0,2,0),("Is it true that i3 knows i2?",0,2,1)], state_names: &["Ada","Acme","Bob"] },
    Case { name: "unordered wildcard", entities: &[("Ada","person"),("Acme","organization"),("Bob","person")], rules: &[("knows","*","*",true)], lead: Lead::Known, text: None,
        expected: &[("Is it true that i1 knows i2, or that i2 knows i1?",0,0,1),("Is it true that i1 knows i3, or that i3 knows i1?",0,0,2),("Is it true that i2 knows i3, or that i3 knows i2?",0,1,2)], state_names: &["Ada","Acme","Bob"] },
    Case { name: "line names", entities: &[("a","*"),("b","*")], rules: &[("r","*","*",false)], lead: Lead::Known, text: None,
        expected: &[("Is it true that i1 r i2?",0,0,1),("Is it true that i2 r i1?",0,1,0)], state_names: &["a","b"] },
    Case { name: "unnamed kind consumes no id", entities: &[("Ada","person"),("Paris","place"),("Acme","organization")], rules: &[("works for","person","organization",false)], lead: Lead::Known, text: None,
        expected: &[("Is it true that i1 works for i2?",0,0,2)], state_names: &["Ada","Acme"] },
    Case { name: "stated wording", entities: &[("Ada","person"),("Acme","organization")], rules: &[("works for","person","organization",false)], lead: Lead::Stated, text: Some("Ada met Acme."),
        expected: &[("Does the text itself state that i1 works for i2?",0,0,1)], state_names: &["Ada","Acme"] },
];

#[derive(Deserialize)]
struct Request {
    state: State,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct State {
    #[serde(default)]
    evidence: Option<String>,
    entities: Vec<StateEntity>,
}

#[derive(Deserialize)]
struct StateEntity {
    id: String,
    name: String,
    kind: String,
}

#[test]
fn pair_questions_keep_rule_order_ids_and_wording() {
    for case in CASES {
        let entities = case
            .entities
            .iter()
            .map(|(name, kind)| RelationEntity::new(name, kind).expect("entity"))
            .collect::<Vec<_>>();
        let rules = case
            .rules
            .iter()
            .map(|(name, source, target, either)| RelationRule {
                name: (*name).to_owned(),
                source: (*source).to_owned(),
                target: (*target).to_owned(),
                reads: (*name).to_owned(),
                either: *either,
                single: false,
            })
            .collect::<Vec<_>>();
        let planned = plan_pairs(case.text, &entities, &rules, case.lead)
            .expect("plan")
            .expect("questions");
        let got = planned
            .questions
            .iter()
            .zip(&planned.pairs)
            .map(|(question, pair)| {
                let crate::core::Question::Decide { text, .. } = question else {
                    panic!("pair is not yes/no")
                };
                (
                    text.as_json().as_str().expect("words"),
                    pair.rule,
                    pair.source,
                    pair.target,
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(got, case.expected, "{}", case.name);
        let body = built_in::encode(
            &Plan::authored(
                planned.evidence,
                ModelName::new("local-1").expect("model"),
                planned.questions,
            )
            .expect("plan"),
        )
        .expect("request");
        let state: Request = serde_json::from_slice(&body).expect("typed state without relation");
        assert_eq!(state.state.evidence.as_deref(), case.text, "{}", case.name);
        let names = state
            .state
            .entities
            .iter()
            .map(|item| item.name.as_str())
            .collect::<Vec<_>>();
        assert_eq!(names, case.state_names, "{}", case.name);
        for (place, item) in state.state.entities.iter().enumerate() {
            assert_eq!(item.id, format!("i{}", place + 1), "{}", case.name);
            assert!(!item.kind.is_empty(), "{}", case.name);
        }
    }
}

#[test]
fn an_unordered_edge_keeps_input_endpoint_order_at_the_cut() {
    let entities = vec![
        RelationEntity::new("Acme", "organization").expect("entity"),
        RelationEntity::new("Ada", "person").expect("entity"),
    ];
    let rules = vec![RelationRule {
        name: "partner".to_owned(),
        source: "person".to_owned(),
        target: "organization".to_owned(),
        reads: "partners with".to_owned(),
        either: true,
        single: false,
    }];
    let planned = plan_pairs(None, &entities, &rules, Lead::Known)
        .expect("plan")
        .expect("one question");
    let edges = pair_edges(
        &entities,
        &rules,
        &planned.pairs,
        &[Answer::new_yes_no(
            Probability::new(0.5).expect("probability"),
        )],
        0.5,
    );
    assert_eq!(edges.len(), 1);
    assert_eq!(
        crate::core::json_line(&edges[0]).expect("edge JSON"),
        r#"{"relation":"partner","source":{"name":"Acme","kind":"organization"},"target":{"name":"Ada","kind":"person"},"probability":0.5}"#
    );
}
