//! `tag`: every label that reaches the cut.

use std::io::Write;

mod support;
use thinkthen::{RequestEnvironment, RequestValue};

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
    let call = tt.call("tag", support::arguments(labels, support::text(text)), RequestEnvironment::default())?;
    let RequestValue::Tags(rows) = call.value() else { return Err("expected tags".into()); };
    let tags = rows.first().ok_or("missing tags")?.result().value().iter().filter_map(|label| Label::from_label(label)).collect::<Vec<_>>();
    writeln!(std::io::stdout().lock(), "{tags:?}")?;
    Ok(())
}
