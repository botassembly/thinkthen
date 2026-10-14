# ThinkThen for Rust

ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. It puts these decision models in your code, whatever model, language or database you use. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

```sh
cargo add thinkthen
```

Set your hosted backend key: `export THINKTHEN_API_KEY=...`. Backends include Jev, SystemOne, OpenAI and local models.

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
        let refund = engine.decide(&refund_question, text)?.into_value();
        println!("{refund:?}");
    }
    for text in ["Please refund the extra fee on my invoice.",
                 "My parcel went to the wrong address.", "I cannot reset my password."] {
        let team = engine.choose(&team_question, text)?.into_value();
        println!("{team:?}");
    }
    Ok(())
}
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
