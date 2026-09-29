//! `tag`: every label that reaches the cut.

use std::io::Write;

use thinkthen::{Engine, Question};

thinkthen::choices! { enum Label { Praise => "praise": "A positive comment", Bug => "bug", Billing => "billing" } }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tt = Engine::from_env()?;
    let labels = Question::tag::<Label>("Which labels fit this message?")?
        .label(Label::Praise, None)?
        .label(Label::Bug, None)?
        .label(Label::Billing, None)?
        .cut()?;
    let text = "Love the new dashboard, but export crashes the app, and I was charged twice.";
    let tags = tt.tag(&labels, text)?.into_value();
    writeln!(std::io::stdout().lock(), "{tags:?}")?;
    Ok(())
}
