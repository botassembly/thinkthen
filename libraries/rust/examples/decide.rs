//! `decide`: yes or no for each text, at the default cut.

use std::io::Write;

mod support;
use thinkthen::{RequestEnvironment, RequestValue};

use thinkthen::{Engine, Question};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tt = Engine::from_env()?;
    let ask = Question::decide("Does the customer ask for a refund?")?.cut();
    let mut out = std::io::stdout().lock();
    for text in [
        "Please refund my order. It arrived broken.",
        "Thanks for the quick help yesterday!",
    ] {
        let call = tt.call("decide", support::arguments(ask.clone(), support::text(text)), RequestEnvironment::default())?;
        let RequestValue::Decisions(rows) = call.value() else { return Err("expected decisions".into()); };
        let answer = match rows.first().ok_or("missing decision")?.result().value() {
            thinkthen::Answer::Yes => "Yes",
            thinkthen::Answer::No => "No",
            thinkthen::Answer::Unsure => "Unsure",
        };
        writeln!(out, "{answer}: {text}")?;
    }
    Ok(())
}
