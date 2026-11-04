//! Table cells for one outcome: the answer as a spreadsheet value, details on request.

use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

/// Suffixes of the detail columns that follow each answer column with details.
pub(crate) const DETAIL_SUFFIXES: [&str; 5] =
    ["_status", "_probability", "_error", "_detail", "_source"];

/// One appended cell.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum Cell {
    /// No value: an answer that is empty, such as find with no match.
    Empty,
    /// A missing value: not sure or failed. Empty in CSV and TSV, `#N/A` in xlsx.
    Missing,
    /// A yes or no answer.
    Bool(bool),
    /// A score, rank or probability.
    Number(Number),
    /// Text, including list answers joined with `; `.
    Text(String),
}

impl Cell {
    /// The cell's delimited-text form.
    pub(crate) fn text(&self) -> String {
        match self {
            Self::Empty | Self::Missing => String::new(),
            Self::Bool(true) => "TRUE".into(),
            Self::Bool(false) => "FALSE".into(),
            Self::Number(number) => number.to_string(),
            Self::Text(text) => text.clone(),
        }
    }

    /// Whether a filter answer keeps its row.
    pub(crate) const fn kept(&self) -> bool {
        matches!(self, Self::Bool(true))
    }
}

/// The cells for one row or annotation entry: its answer, then details on request.
pub(crate) fn outcome(value: &Value, details: bool) -> Vec<Cell> {
    let mut cells = vec![answer(value)];
    if !details {
        return cells;
    }
    let text = |value: Option<&Value>| match value {
        None | Some(Value::Null) => Cell::Empty,
        Some(Value::String(text)) => Cell::Text(text.clone()),
        Some(Value::Number(number)) => Cell::Number(number.clone()),
        Some(value) => Cell::Text(value.to_string()),
    };
    cells.push(text(value.get("status")));
    cells.push(text(value.get("probability")));
    cells.push(match value.get("error") {
        Some(error) if !error.is_null() => Cell::Text(format!(
            "{}: {}",
            error
                .get("code")
                .and_then(Value::as_str)
                .unwrap_or_default(),
            error
                .get("message")
                .and_then(Value::as_str)
                .unwrap_or_default()
        )),
        _ => Cell::Empty,
    });
    cells.push(text(value.get("detail")));
    cells.push(text(value.get("source")));
    cells
}

fn answer(value: &Value) -> Cell {
    if value.get("status").and_then(Value::as_str) != Some("answered") {
        return Cell::Missing;
    }
    match value.get("answer") {
        None | Some(Value::Null) => Cell::Empty,
        Some(Value::Bool(answer)) => Cell::Bool(*answer),
        Some(Value::Number(answer)) => Cell::Number(answer.clone()),
        Some(Value::String(answer)) => Cell::Text(answer.clone()),
        Some(Value::Array(items)) if items.is_empty() => Cell::Empty,
        Some(Value::Array(items)) => {
            Cell::Text(items.iter().map(item).collect::<Vec<_>>().join("; "))
        }
        Some(answer) => Cell::Text(item(answer)),
    }
}

/// A readable list item: a label, a span's text, `text (kind)` for a name,
/// or `source relation target` for a relation.
fn item(value: &Value) -> String {
    let text = |value: &Value| {
        value
            .get("text")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_owned()
    };
    match value {
        Value::String(label) => label.clone(),
        Value::Object(fields) => {
            if let (Some(source), Some(Value::String(relation)), Some(target)) = (
                fields.get("source"),
                fields.get("relation"),
                fields.get("target"),
            ) {
                format!("{} {relation} {}", text(source), text(target))
            } else if let Some(Value::String(kind)) = fields.get("kind") {
                format!("{} ({kind})", text(value))
            } else {
                text(value)
            }
        }
        value => value.to_string(),
    }
}
