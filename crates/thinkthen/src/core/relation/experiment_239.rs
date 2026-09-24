use std::collections::BTreeMap;

use serde::Deserialize;

use super::{RelationRule, assemble_edges, plan};
use crate::core::answer::Distribution;
use crate::core::probability::Probability;
use crate::core::recognize::RecognizedName;
use crate::core::{Answer, Question};

const INPUT: &str = include_str!("../../../tests/fixtures/recognize-239/input.json");
const QSETS: [&str; 3] = [
    include_str!("../../../tests/fixtures/recognize-239/qset-00.json"),
    include_str!("../../../tests/fixtures/recognize-239/qset-01.json"),
    include_str!("../../../tests/fixtures/recognize-239/qset-02.json"),
];
const REPLAYS: [&str; 3] = [
    include_str!("../../../tests/fixtures/recognize-239/replay-00.jsonl"),
    include_str!("../../../tests/fixtures/recognize-239/replay-01.jsonl"),
    include_str!("../../../tests/fixtures/recognize-239/replay-02.jsonl"),
];

#[derive(Clone, Deserialize)]
struct RecordedQuestion {
    choose: String,
    options: BTreeMap<String, String>,
}

#[derive(Deserialize)]
struct QuestionSet {
    questions: BTreeMap<String, RecordedQuestion>,
}

#[derive(Deserialize)]
struct Input { entities: Vec<InputEntity> }
#[derive(Deserialize)]
struct InputEntity { name: String, kind: String }
#[derive(Deserialize)]
struct Replay { answers: BTreeMap<String, RecordedResult> }
#[derive(Deserialize)]
struct RecordedResult { answer: RecordedAnswer }
#[derive(Deserialize)]
struct RecordedAnswer { probabilities: BTreeMap<String, f64> }

#[test]
fn experiment_239_replays_only_the_recorded_cross_kind_planner_proof() {
    let input: Input = serde_json::from_str(INPUT).expect("input JSON");
    let entities = input.entities.iter()
        .enumerate()
        .map(|(place, item)| entity(item, place))
        .collect::<Vec<_>>();
    for (prefix, target, reads, option_count, edge_count) in [
        ("c_sung_by_", "person", "has its lead vocal sung by", 5, 142),
        ("c_appears_on_", "album", "appears on", 14, 123),
    ] {
        let rule = RelationRule {
            name: prefix[2..prefix.len() - 1].to_owned(),
            source: "song".to_owned(),
            target: target.to_owned(),
            either: false,
            reads: reads.to_owned(),
        };
        let plans = plan(&entities, &rule).expect("plan");
        let planned = plans.first().expect("concrete plan");
        assert_eq!(planned.questions.len(), 184);
        let recorded = recorded_questions(prefix);
        for (question, expected) in planned.questions.iter().zip(&recorded) {
            let Question::Choose { text, options } = question else {
                panic!("choice")
            };
            assert_eq!(options.count(), option_count);
            assert_eq!(text.as_json().as_str(), Some(expected.choose.as_str()));
            for (name, description) in options.descriptions() {
                assert_eq!(
                    description.and_then(|held| held.as_json().as_str()),
                    expected.options.get(name).map(String::as_str)
                );
            }
        }
        let answers = recorded_answers(prefix, &planned.questions);
        let edges = assemble_edges(&entities, &planned.relation, &planned.mappings, &answers, 0.5);
        assert_eq!(edges.len(), edge_count);
        let expected = recorded.iter().zip(recorded_probabilities(prefix)).flat_map(|(question, probabilities)| {
            let source = fixture_entity(&input, &question.choose);
            question.options.iter().filter_map(|(label, description)| {
                probabilities.get(label).filter(|probability| **probability >= 0.5).map(|probability| (source.name.clone(), fixture_entity(&input, description).name.clone(), *probability))
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();
        let actual = edges.iter().map(|edge| (edge.source.name.clone(), edge.target.name.clone(), edge.probability)).collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }
}

fn fixture_entity<'a>(input: &'a Input, description: &str) -> &'a InputEntity {
    let start = description.find("Item ").expect("recorded item") + 5;
    let end = description[start..].find(" (").expect("recorded kind") + start;
    let place = description[start..end].parse::<usize>().expect("item number") - 1;
    input.entities.get(place).expect("truth entity")
}

fn entity(item: &InputEntity, start: usize) -> RecognizedName {
    RecognizedName {
        name: item.name.clone(),
        kind: item.kind.clone(),
        start,
        end: start + item.name.chars().count(),
        strength: 1.0,
    }
}

fn recorded_questions(prefix: &str) -> Vec<RecordedQuestion> {
    let mut found = QSETS
        .iter()
        .flat_map(|text| {
            serde_json::from_str::<QuestionSet>(text)
                .expect("qset")
                .questions
        })
        .filter(|(name, _)| name.starts_with(prefix))
        .map(|(name, question)| (index(&name), question))
        .collect::<Vec<_>>();
    found.sort_by_key(|(place, _)| *place);
    found.into_iter().map(|(_, question)| question).collect()
}

fn recorded_answers(prefix: &str, questions: &[Question]) -> Vec<Answer> {
    recorded_probabilities(prefix).into_iter().zip(questions).map(|(probabilities, question)| {
        let Question::Choose { options, .. } = question else { panic!("choice") };
        let entries = options.names().map(|label| (label.to_owned(), Probability::new(probabilities[label]).expect("probability"))).collect();
        Answer::new_choice(Distribution::with_tolerance(entries, 0.02).expect("distribution"), None).expect("answer")
    }).collect()
}

fn recorded_probabilities(prefix: &str) -> Vec<BTreeMap<String, f64>> {
    let mut found = vec![None; 184];
    for replay in REPLAYS {
        let replay: Replay = serde_json::from_str(replay).expect("replay");
        for (name, result) in replay.answers {
            if !name.starts_with(prefix) {
                continue;
            }
            let place = index(&name) - 1;
            found[place] = Some(result.answer.probabilities);
        }
    }
    found.into_iter().map(|answer| answer.expect("recorded answer")).collect()
}

fn index(name: &str) -> usize {
    name.rsplit('_')
        .next()
        .and_then(|part| part.parse().ok())
        .expect("fixture index")
}
