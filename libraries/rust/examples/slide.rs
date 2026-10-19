//! The deck's Rust slide: build a question once, and the band's middle is unsure.

use std::io::Write;

mod support;
use thinkthen::{RequestEnvironment, RequestValue};

use thinkthen::{Engine, Question};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tt = Engine::from_env()?;
    let refund = Question::decide("Does the customer ask for a refund?")?.band(0.2, 0.8)?;
    let mut out = std::io::stdout().lock();
    let call = tt.call(
        "decide",
        support::arguments(refund, support::text("I want to send this back.")),
        RequestEnvironment::default(),
    )?;
    let RequestValue::Decisions(rows) = call.value() else {
        return Err("expected decisions".into());
    };
    let answer = rows.first().ok_or("missing decision")?.result();
    match answer.value() {
        thinkthen::Answer::Unsure => writeln!(out, "a person reads it")?,
        thinkthen::Answer::Yes => writeln!(out, "Yes")?,
        thinkthen::Answer::No => writeln!(out, "No")?,
    }
    Ok(())
}
