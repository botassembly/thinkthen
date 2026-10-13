# ThinkThen for Rust

ThinkThen puts ten functions on top of any model: decide, choose, tag, score, filter, rank, find, annotate, recognize and relate. Your code branches on typed answers, gets not sure instead of a guess, packs many records into few requests, and caches each question so you pay once. Tests replay saved answers with no key or network. Use Jev, OpenAI or a local model from every language and database.

```sh
cargo add thinkthen
```

Set your hosted backend key: `export THINKTHEN_API_KEY=...`.

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

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
