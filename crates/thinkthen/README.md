# ThinkThen for Rust

ThinkThen answers typed questions about text. Ask for a yes or no, a label, a number, or records selected and ranked by meaning. Failed calls remain separate from answers. The crate also provides the `thinkthen` command.

## Install

```sh
cargo add thinkthen
```

## Example

Configure a model backend before running this example.

```rust
use thinkthen::{Engine, Question};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::from_env()?;
    let question = Question::decide("Does this ask for a refund?")?.cut();
    let answer = engine.decide(&question, "Please refund my order.")?;
    println!("{:?}", answer.into_value());
    Ok(())
}
```

## Cost

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
