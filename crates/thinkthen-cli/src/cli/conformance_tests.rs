//! Command diagnostics and child environment isolation for the shared fault cases.

use serde::Deserialize;
use serde_json::value::RawValue;

mod command;
mod runner;

const CASES: &str = include_str!("../../../../conformance/cases.json");

#[derive(Deserialize)]
struct Document {
    cases: Vec<Case>,
}

#[derive(Deserialize)]
struct Case {
    id: String,
    verb: String,
    question: Option<Box<RawValue>>,
    question_form: Option<QuestionForm>,
    evidence: Option<String>,
    operation: Option<Injection>,
    expect: Expect,
}

#[derive(Clone, Copy, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum QuestionForm {
    Text,
    File,
}

#[derive(Deserialize)]
struct Injection {
    injection: String,
}

#[derive(Deserialize)]
struct Expect {
    error: Option<ExpectedError>,
}

#[derive(Deserialize)]
struct ExpectedError {
    kind: String,
}
