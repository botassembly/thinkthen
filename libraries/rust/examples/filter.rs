//! `filter`: the records whose answer is yes, lazily, in input order.

use std::io::Write;

mod support;
use thinkthen::{RequestEnvironment, RequestValue};

use thinkthen::{Engine, Question};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tt = Engine::from_env()?;
    let complaint = Question::decide("Is this a complaint?")?.cut();
    let reviews = [
        "Arrived a day early. Thank you!",
        "The zipper broke the first time I used it.",
    ];
    let kept = tt.call(
        "filter",
        support::arguments(complaint, support::records(reviews)),
        RequestEnvironment::default(),
    )?;
    let RequestValue::Filtered(rows) = kept.value() else {
        return Err("expected filtered records".into());
    };
    let mut out = std::io::stdout().lock();
    for row in rows {
        writeln!(out, "{}", support::original(row.original())?)?;
    }
    Ok(())
}
