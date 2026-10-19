//! `find`: the one unit that best answers the question, or none.

use std::io::Write;

mod support;
use thinkthen::{RequestEnvironment, RequestValue};

use thinkthen::{Engine, Question};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tt = Engine::from_env()?;
    let deadline = Question::find("Which line gives the refund deadline?")?;
    let policy = [
        "Refunds are issued within 30 days of purchase.",
        "Shipping is free on orders over $50.",
    ];
    let found = tt.call(
        "find",
        support::arguments(deadline, support::records(policy)),
        RequestEnvironment::default(),
    )?;
    let RequestValue::Found(result) = found.value() else {
        return Err("expected found unit".into());
    };
    let value = result.selected().map(support::original).transpose()?;
    writeln!(std::io::stdout().lock(), "{:?}", value)?;
    Ok(())
}
