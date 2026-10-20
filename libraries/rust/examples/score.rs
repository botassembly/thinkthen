//! `score`: a position on your own scale, 0 to 2 for three levels.

use std::io::Write;

mod support;
use thinkthen::{RequestEnvironment, RequestValue};

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
        let call = tt.call(
            "score",
            support::arguments(urgency.clone(), support::text(text)),
            RequestEnvironment::default(),
        )?;
        let RequestValue::Scores(rows) = call.value() else {
            return Err("expected scores".into());
        };
        writeln!(
            out,
            "{} {text}",
            rows.first().ok_or("missing score")?.result().value()
        )?;
    }
    Ok(())
}
