//! `annotate`: one filled form per record, each value in the set's order.

use std::io::Write;

mod support;
use thinkthen::{RequestEnvironment, RequestValue};

use thinkthen::{Engine, QuestionSet};

const FORM: &str = r#"{"version": 1, "questions": {
    "wants_refund": {"decide": "Does the customer ask for a refund?"},
    "team": {"choose": "Which team owns this?", "options": ["billing", "shipping", "account"]}
}}"#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tt = Engine::from_env()?;
    let form = QuestionSet::from_json(FORM)?;
    let reports = [
        "CSV export fails. Steps: click Export.",
        "I was charged twice this month.",
    ];
    let mut out = std::io::stdout().lock();
    let annotated = tt.call("annotate", support::arguments(form, support::records(reports)), RequestEnvironment::default())?;
    let RequestValue::Annotations(rows) = annotated.value() else { return Err("expected annotations".into()); };
    for record in rows {
        let values = record.result().members();
        for value in values {
            write!(out, "{}={:?} ", value.name(), value.value().ok_or("missing annotation value")?)?;
        }
        writeln!(out, "| {}", support::original(record.original())?)?;
    }
    Ok(())
}
