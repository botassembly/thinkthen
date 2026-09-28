//! The shared cases over a Series and a frame, against the slice forms.
//!
//! Each case in `RUN` runs on its own case arm twice: once through the
//! door, and once through the `thinkthen` slice form on a second engine.
//! The two give equal values. `NOT_RUN` names every other case and why the
//! door cannot carry it, and the test holds the two lists to the case file.

#![allow(clippy::expect_used, reason = "a failed fixture stops the proof")]

mod common;

use std::collections::BTreeMap;

use conformance_backend::Backend;
use serde_json::value::RawValue;
use thinkthen::PolarsEngine;
use thinkthen::polars::prelude::{AnyValue, DataFrame, IntoColumn, Series};
use thinkthen::{
    Answer, CallOptions, Engine, Judgment, LoadedQuestion, Question, QuestionKind, QuestionSet,
};

const CASES: &str = include_str!("../../../../conformance/cases.json");

const RUN: &[&str] = &[
    "01-decide-yes-captured",
    "02-decide-no",
    "03-decide-band-unsure",
    "04-decide-band-yes",
    "05-decide-meanings",
    "06-choose-billing",
    "07-choose-unsure",
    "08-choose-tie",
    "09-tag-two",
    "10-tag-none",
    "11-score-middle",
    "12-score-upper",
    "17-annotate-mixed",
    "17-annotate-partial",
    "18-annotate-two-groups",
    "27-decide-many",
    "28-decide-many-repeated-texts",
    "32-score-equal-distribution",
    "33-tag-threshold-excludes",
    "34-annotate-repeated-texts",
    "35-annotate-score-repeated-texts",
    "36-annotate-two-columns",
    "37-annotate-choose-one",
    "38-annotate-score-one",
    "39-annotate-tag-one",
];

/// Each case the door does not run, by its number or its whole id, and why.
const NOT_RUN: &[(&str, &str)] = &[
    (
        "13 14 15 16 18-find-second 19 26 31 41 42 43 44 45 46 47 48 49 50 51 52",
        "filter, rank, find, recognize, and relate, which the door does not carry",
    ),
    (
        "20 21 22 23 24 25",
        "a fault injection, and 0095 gives a binding no public fault hook",
    ),
    (
        "29 30",
        "a question as JSON text or a file, and the door takes built questions",
    ),
    ("40", "counters, and the door adds none"),
];

/// One case as written: its id, verb, question bytes, and evidence texts.
/// The question keeps its bytes and key order, because the case arm matches
/// request bodies exactly.
struct Case {
    id: String,
    verb: String,
    question: String,
    texts: Vec<String>,
}

fn cases() -> Vec<Case> {
    let document: BTreeMap<String, Box<RawValue>> =
        serde_json::from_str(CASES).expect("the shared cases");
    let listed: Vec<BTreeMap<String, Box<RawValue>>> =
        serde_json::from_str(document.get("cases").expect("a case list").get())
            .expect("the case list");
    let word = |case: &BTreeMap<String, Box<RawValue>>, key: &str| -> String {
        serde_json::from_str(case.get(key).expect("a field").get()).expect("a string")
    };
    listed
        .iter()
        .map(|case| {
            let exchanges: Vec<serde_json::Value> = case
                .get("exchanges")
                .map_or(Ok(Vec::new()), |raw| serde_json::from_str(raw.get()))
                .expect("the exchanges");
            let asked = case.get("question").or_else(|| case.get("question_set"));
            Case {
                id: word(case, "id"),
                verb: word(case, "verb"),
                question: asked.map_or("", |raw| raw.get()).to_owned(),
                // A case with a record sends one JSON record, and its set reads the parts.
                texts: case.get("record").map_or_else(
                    || {
                        let evidence = exchanges.iter().map(|one| one["evidence"].as_str());
                        evidence
                            .map(|one| one.unwrap_or_default().to_owned())
                            .collect()
                    },
                    |raw| vec![raw.get().to_owned()],
                ),
            }
        })
        .collect()
}

fn skipped(id: &str) -> bool {
    NOT_RUN.iter().any(|(named, _)| {
        named
            .split(' ')
            .any(|part| part == id || (part.len() == 2 && id.get(..3) == Some(&format!("{part}-"))))
    })
}

#[test]
fn every_shared_case_the_door_carries_matches_the_slice_form() {
    let backend = Backend::start().expect("a backend");
    let mut ran = Vec::new();
    let cases = cases();
    for case in &cases {
        let run = RUN.contains(&case.id.as_str());
        assert!(
            run != skipped(&case.id),
            "{} must be run or named not run, and not both",
            case.id
        );
        if !run {
            continue;
        }
        let base = format!("{}/case/{}/v1", backend.origin(), case.id);
        let (door, slice) = (common::engine(&base), common::engine(&base));
        let texts: Vec<&str> = case.texts.iter().map(String::as_str).collect();
        let (through_door, through_slice) = if case.verb == "annotate" {
            annotated(&door, &slice, &case.question, &texts)
        } else {
            let (door, slice) = judged(&door, &slice, &case.question, &texts);
            (vec![door], vec![slice])
        };
        assert_eq!(through_door, through_slice, "{}", case.id);
        ran.push(case.id.as_str());
    }
    assert_eq!(ran, RUN);
}

/// One cell as text: `true`, a number's JSON text, a label, or null.
fn text(value: &AnyValue<'_>) -> Option<String> {
    match value {
        AnyValue::Null => None,
        AnyValue::Boolean(held) => Some(held.to_string()),
        AnyValue::Float64(held) => serde_json::to_string(held).ok(),
        AnyValue::String(held) => Some((*held).to_owned()),
        AnyValue::List(held) => {
            let labels: Vec<Option<String>> = held.iter().map(|one| text(&one)).collect();
            serde_json::to_string(&labels).ok()
        }
        other => Some(format!("unexpected {other:?}")),
    }
}

/// A column's cells as text, in row order.
type Cells = Vec<Option<String>>;

fn cells(series: &Series) -> Cells {
    series.iter().map(|value| text(&value)).collect()
}

/// A judgment's value as the same text a door cell reads as.
fn judgment(value: &Judgment) -> Option<String> {
    match value {
        Judgment::Decision(Answer::Unsure) | Judgment::Choice(None) => None,
        Judgment::Decision(answer) => Some((*answer == Answer::Yes).to_string()),
        Judgment::Choice(Some(label)) => Some(label.clone()),
        Judgment::Score(position) => serde_json::to_string(position).ok(),
        Judgment::Tags(labels) => serde_json::to_string(labels).ok(),
    }
}

/// One question over the texts: the door's column against the slice form,
/// `decide_many` for a decision and `details` for the others.
fn judged(door: &Engine, slice: &Engine, question: &str, texts: &[&str]) -> (Cells, Cells) {
    let column = common::column(texts);
    let options = CallOptions::new;
    let decided = |listed: Vec<Answer>| -> Cells {
        let answers = listed.into_iter().map(Judgment::Decision);
        answers.map(|answer| judgment(&answer)).collect()
    };
    let answered = match Question::from_json(question).expect("the case's question") {
        LoadedQuestion::Banded(banded) => {
            let listed = slice
                .decide_many(&banded, texts.to_vec())
                .map(|row| row.map(|row| *row.value()));
            let listed = decided(listed.collect::<Result<_, _>>().expect("the slice form"));
            return (
                cells(
                    &door
                        .decide_series(&banded, &column, options())
                        .expect("the door"),
                ),
                listed,
            );
        }
        LoadedQuestion::Question(asked) if asked.kind() == QuestionKind::Decide => {
            let listed = slice
                .decide_many(&asked, texts.to_vec())
                .map(|row| row.map(|row| *row.value()));
            let listed = decided(listed.collect::<Result<_, _>>().expect("the slice form"));
            return (
                cells(
                    &door
                        .decide_series(&asked, &column, options())
                        .expect("the door"),
                ),
                listed,
            );
        }
        LoadedQuestion::Question(asked) => asked,
    };
    let through_door = match answered.kind() {
        QuestionKind::Choose => door.choose_series(&answered, &column, options()),
        QuestionKind::Score => door.score_series(&answered, &column, options()),
        _ => door.tag_series(&answered, &column, options()),
    };
    let listed = texts
        .iter()
        .map(|one| {
            slice
                .details(&answered, one)
                .map(|found| judgment(found.value()))
        })
        .collect::<Result<_, _>>()
        .expect("the one-text form");
    (cells(&through_door.expect("the door")), listed)
}

/// A set over the texts: each typed answer against its record's JSON member.
fn annotated(door: &Engine, slice: &Engine, set: &str, texts: &[&str]) -> (Vec<Cells>, Vec<Cells>) {
    let set = QuestionSet::from_json(set).expect("the case's set");
    // Case 18's set names a member `body`, so the records ride in `record`.
    let records = common::column(texts).with_name("record".into());
    let frame = DataFrame::new(texts.len(), vec![records.into_column()]).expect("a frame");
    let out = door
        .annotate_frame(&set, &frame, "record", CallOptions::new())
        .expect("the door");
    let records: Vec<String> = slice
        .annotate(&set, texts.to_vec())
        .map(|record| record.map(|record| record.value_json()))
        .collect::<Result<_, _>>()
        .expect("the slice form");
    let mut through_door = Vec::new();
    let mut through_slice = Vec::new();
    for (name, _) in set.members() {
        let column = out.column(name).expect("a new column");
        through_door.push(cells(column.as_materialized_series()));
        through_slice.push(
            records
                .iter()
                .map(|record| {
                    let members: BTreeMap<String, &RawValue> =
                        serde_json::from_str(record).expect("a record's JSON");
                    let raw = members.get(name).expect("the member").get();
                    match raw {
                        "null" => None,
                        _ if raw.starts_with("{\"failed\"") => None,
                        _ if raw.starts_with('"') => serde_json::from_str(raw).ok(),
                        _ => Some(raw.to_owned()),
                    }
                })
                .collect(),
        );
    }
    (through_door, through_slice)
}
