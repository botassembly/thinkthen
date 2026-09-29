//! `decide`: yes or no for each text, at the default cut.

use std::io::Write;

use thinkthen::{Engine, Question};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tt = Engine::from_env()?;
    let ask = Question::decide("Does the customer ask for a refund?")?.cut();
    let mut out = std::io::stdout().lock();
    for text in [
        "Please refund my order. It arrived broken.",
        "Thanks for the quick help yesterday!",
    ] {
        writeln!(out, "{:?}: {text}", tt.decide(&ask, text)?.into_value())?;
    }
    Ok(())
}
