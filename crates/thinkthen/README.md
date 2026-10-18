# ThinkThen for Rust

ThinkThen is ten functions on top of Jev, TypeSafe's System One model, or any model like it. System One models return probabilities. Your code decides what to do with them. ThinkThen makes these models simple to call from any language or database. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

```sh
cargo add thinkthen
```

Set your hosted backend key: `export THINKTHEN_API_KEY=...`. Backends include Jev, SystemOne, OpenAI and local models. Judgment calls can send your question and text to the selected backend; the default is TypeSafe.

```rust
use thinkthen::{Description, Engine, Question};
thinkthen::choices! { enum Team { Billing => "billing", Shipping => "shipping", Account => "account" } }
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::from_env()?;
    let refund_question = Question::decide("Does the customer ask for a refund?")?.cut();
    let team_question = Question::choose::<Team>("Which team owns this?")?
        .option(Team::Billing, Some(Description::text("Invoices, fees, and refunds.")?))?
        .option(Team::Shipping, Some(Description::text("Parcels and delivery.")?))?
        .option(Team::Account, Some(Description::text("Logins and passwords.")?))?.build()?;
    for text in ["Please refund my order. It arrived broken.",
                 "Thanks for the quick help yesterday!", "I want to send this back."] {
        let refund = engine.decide(&refund_question, text)?;
        println!("{:?}", refund.value());
    }
    for text in ["Please refund the extra fee on my invoice.",
                 "My parcel went to the wrong address.", "I cannot reset my password."] {
        let team = engine.choose(&team_question, text)?;
        println!("{:?}", team.value());
    }
    Ok(())
}
```

Each call exposes `value()`, `probability()`, typed `results()` and `facts()`.

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them. These examples were checked with saved answers; live answers and probabilities can differ. See the [package guide](https://thinkthen.dev/languages/rust/) for results, failures, record inputs and offline replay.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
