//! The replay smoke (ticket 0335): one `decide` through `Engine::from_env`,
//! with the question and text `sdlc/scripts/smoke` names.

use std::io::Write;

use thinkthen::{Answer, Engine, Question};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let question = Question::decide(&std::env::var("THINKTHEN_SMOKE_QUESTION")?)?.cut();
    let text = std::env::var("THINKTHEN_SMOKE_TEXT")?;
    let value = match Engine::from_env()?.decide(&question, &text)?.into_value() {
        Answer::Yes => "true",
        Answer::No => "false",
        Answer::Unsure => "null",
    };
    writeln!(std::io::stdout().lock(), "smoke: {value}")?;
    Ok(())
}
