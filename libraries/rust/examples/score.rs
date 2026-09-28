//! `score`: a position on your own scale, 0 to 2 for three levels.

use std::io::Write;

use thinkthen::{Engine, Question};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tt = Engine::from_env()?;
    let urgency = Question::score("How urgent is this?")?
        .level("Routine.", None)?
        .level("Soon.", None)?
        .level("Immediate.", None)?
        .build()?;
    let mut out = std::io::stdout().lock();
    for text in [
        "Please update my mailing address when you can.",
        "Nobody can log in to the site right now.",
    ] {
        writeln!(out, "{} {text}", tt.score(&urgency, text)?)?;
    }
    Ok(())
}
