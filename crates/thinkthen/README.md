# Rust SDK

The Rust SDK asks questions about text from your Rust code. Its first function, decide, answers a yes-or-no question with yes (`true`), no (`false`) or not sure.

Install it with Rust 1.95.0 or later:

```sh
cargo add thinkthen@0.2.0 --no-default-features
```

The crate carries its own engine. `--no-default-features` leaves out the `command` feature, which builds the `thinkthen` command. The `stream` feature adds async streams. The `native` feature holds unstable helpers for language bindings, proxies and database hosts, and they may change in any release. The engine reads your key from `THINKTHEN_API_KEY`. [Configuration](https://thinkthen.dev/configuration/) shows how to get one.

## Ask a question

Ask whether a support message requests a refund:

```rust
use thinkthen::{Decide, Engine};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::from_env()?;
    let refund = Decide::new("Does the customer request a refund?")?;
    let is_refund: bool = engine
        .run(&refund, "Please refund my order.")?
        .into_answer()?;
    println!("{is_refund}");
    Ok(())
}
```

Output:

```text
true
```

`true` means yes: the message asks for a refund.

## Not sure and failures

Set a low and a high bar, and decide is not sure between them. `into_answer()` returns `NoAnswer::Unsure` when decide is not sure, and `NoAnswer::Failed` when ThinkThen could not judge the message, so neither one reads as `false`. Send both to a person.

```rust
use thinkthen::{Decide, Engine, NoAnswer, Threshold};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::from_env()?;
    let refund = Decide::new("Does the customer request a refund?")?
        .threshold(Threshold::band(0.2, 0.8)?);
    let message = "I was charged twice for one invoice.";
    match engine.run(&refund, message)?.into_answer() {
        Ok(is_refund) => println!("{is_refund}"),
        Err(NoAnswer::Unsure { probability }) => println!(
            "Not sure ({}): ask a person.",
            probability.get()
        ),
        Err(NoAnswer::Failed(error)) => {
            println!("{}: {error}", error.code())
        }
    }
    Ok(())
}
```

Output:

```text
Not sure (0.7): ask a person.
```

## Next

The [Rust SDK page](https://thinkthen.dev/sdks/rust/) shows all ten functions. The [Rust guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/rust.md) and the [API reference](https://docs.rs/thinkthen/0.2.0/thinkthen/) cover every option.
