//! `relate`: the edges among entities you already hold, in rule order.

use std::io::Write;

mod support;
use thinkthen::{RequestEnvironment, RequestValue};

use thinkthen::{Engine, QuestionInput, Relate, RelationRule};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tt = Engine::from_env()?;
    let ask = Relate::builder()
        .relation(RelationRule::both_ways("contradicts", "*", "*")?)?
        .build()?;
    let rules = [
        QuestionInput::annotation_document(
            r#"{"name":"Book economy class for every flight.","kind":"rule"}"#,
        )?,
        QuestionInput::annotation_document(
            r#"{"name":"Employees may book business class on any flight.","kind":"rule"}"#,
        )?,
    ];
    let mut out = std::io::stdout().lock();
    let call = tt.call(
        "relate",
        support::arguments(ask, support::records(rules)),
        RequestEnvironment::default(),
    )?;
    let RequestValue::Related(row) = call.value() else {
        return Err("expected relations".into());
    };
    for edge in row.result().value() {
        let (name, probability) = (edge.relation(), edge.probability());
        let (source, target) = (edge.source().name(), edge.target().name());
        writeln!(out, "{name} {probability:.2} {source} | {target}")?;
    }
    Ok(())
}
