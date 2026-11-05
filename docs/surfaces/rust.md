# Rust functions that answer questions about text

Add the crate with Rust 1.95.0 or later: `cargo add thinkthen@0.2.0 --no-default-features`. The engine is inside the crate. Leaving out `--no-default-features` also builds the `thinkthen` command.

This program asks whether a message requests a refund. Set `THINKTHEN_API_KEY` in the environment first; `Engine::from_env()` reads it from there.

```rust
use thinkthen::{Decide, Engine};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::from_env()?;
    let refund = Decide::new("Does the customer request a refund?")?;
    // into_answer() fails on not sure or a failed record, so neither reads as false.
    let is_refund: bool = engine.run(&refund, "Please refund my order.")?.into_answer()?;
    println!("{is_refund}");
    Ok(())
}
```

<!-- output: fill from the real-backend capture run -->

It prints `true` or `false`.

## Branch on not sure and failures

`into_answer()` returns `Err(NoAnswer)` when a row has no answer. A band sends the middle to a person: below 0.2 is no, 0.8 and above is yes, and between them is not sure.

```rust
use thinkthen::{Decide, Engine, NoAnswer, Threshold};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::from_env()?;
    let refund = Decide::new("Does the customer request a refund?")?
        .threshold(Threshold::band(0.2, 0.8)?);
    match engine.run(&refund, "Please refund my order.")?.into_answer() {
        Ok(true) => println!("refund queue"),
        Ok(false) => println!("no refund requested"),
        Err(NoAnswer::Unsure { probability }) => {
            println!("a person reviews it (probability {})", probability.get())
        }
        Err(NoAnswer::Failed(error)) => println!("cannot judge it: {}", error.code()),
    }
    Ok(())
}
```

<!-- output: fill from the real-backend capture run -->

How not sure and failures arrive:

- `into_answer()` takes the typed answer and `answer()` borrows it. Each returns `NoAnswer::Unsure { probability }` for not sure and `NoAnswer::Failed(error)` for a failed record. Neither ever reads as `false`, and `?` passes either one up.
- A failed record is an ordinary row, so `run` still returns `Ok(row)`, with one exception: blank text, such as empty or all-space text or an empty JSON object or array, makes `run` return `Err` with `record.empty`. Otherwise `run` returns `Err` only when the call itself fails, such as when it is cancelled.
- `error.code()` prints the shared code, such as `record.missing`. `error.code().family()` gives its family, and `error.http_status()` the backend's HTTP status when there was one.
- `Status` and `Outcome` have exactly three cases: answered, unsure and failed. A `match` on them needs no catch-all arm, so the compiler flags a missed not-sure case.
- A row prints as its answer, `unsure`, or `failed:` and the error code. Rows also compare with `==`.

A row also has `position()`, counting records from 1, `id()`, `source()`, `status()`, `probability()`, `error()` and `outcome()`.

## Ask each question of one message

Each call asks one question of one message and gives the answer in its Rust type. Filter and rank judge many records, so they have [their own section](#keep-the-refund-requests-then-rank-them).

```rust
use thinkthen::{
    AnnotatableQuestion, Annotate, Choose, Decide, Engine, Find, Recognize, Relate, RelationRule,
    Score, Tag,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::from_env()?;

    let refund = Decide::new("Does the customer request a refund?")?;
    let is_refund: bool = engine.run(&refund, "Please refund my order.")?.into_answer()?;
    println!("refund: {is_refund}");

    let about = Choose::new("What is the message about?", ["billing", "shipping", "other"])?;
    let topic: String = engine.run(&about, "Please refund my order.")?.into_answer()?;
    println!("topic: {topic}");

    let appear = Tag::new("Which topics appear?", ["billing", "shipping"])?;
    let topics: Vec<String> = engine
        .run(&appear, "I was charged twice and my parcel is late again.")?
        .into_answer()?;
    println!("topics: {topics:?}");

    // 0 is routine, 1 is urgent.
    let how_urgent = Score::new("How urgent is the request?", ["routine", "urgent"])?;
    let urgency: f64 = engine.run(&how_urgent, "Please refund my order.")?.into_answer()?;
    println!("urgency: {urgency}");

    // Spans are UTF-8 byte offsets, so they slice the message directly.
    let message = "Morgan works for Example Group.\nPlease refund my order.";
    let requested = Find::new("Which line states the requested action?")?;
    match engine.run(&requested, message)?.into_answer()? {
        Some(action) => println!("action: {}", &message[action.start()..action.end()]),
        None => println!("action: no line answers"),
    }

    let people_and_groups = Recognize::new(["person", "organization"])?;
    let names = engine.run(&people_and_groups, "Morgan works for Example Group.")?.into_answer()?;
    for name in &names {
        println!("name: {}: {}", name.kind(), name.text());
    }

    let works_for = Relate::new([RelationRule::new("works_for", "person", "organization")?])?;
    let relations = engine.run(&works_for, message)?.into_answer()?;
    for relation in &relations {
        println!(
            "relation: {} --{}--> {}",
            relation.source().text(),
            relation.relation(),
            relation.target().text()
        );
    }

    // Annotate asks several named questions of one message.
    let triage = Annotate::new([
        ("refund", AnnotatableQuestion::from(refund)),
        ("topic", about.into()),
        ("urgency", how_urgent.into()),
    ])?;
    let answers = engine.run(&triage, "Please refund my order.")?.into_answer()?;
    for (question, answer) in &answers {
        println!("{question}: {answer}");
    }
    Ok(())
}
```

<!-- output: fill from the real-backend capture run -->

Look for one line per answer. Find gives `None` when no line answers. Recognize and relate give an empty `Vec` when the message names nothing that fits. Each annotate answer prints bare, or as `unsure` or `failed:` with its code, because one question in the form can be not sure while the others answer.

## Keep the refund requests, then rank them

`engine.filter` yields only the items it keeps, in input order. The closure turns each item into its record. A not-sure item is left out. A failed record yields `Err(FilterError)` in its place, with the item, and reading goes on. `engine.filter_items` yields every item with its row instead.

For a rank, `engine.rows` yields one row for each record in rank order, with failed records last. Rank 1 is the most urgent. Each rank row keeps its own record, and `row.record()` returns it.

```rust
use thinkthen::{Engine, Filter, Rank};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::from_env()?;
    let messages = vec![
        "Please refund my order.",
        "Where is my parcel?",
        "I was charged twice for one invoice.",
    ];

    let refund = Filter::new("Does the customer request a refund?")?;
    for refund_request in engine.filter(&refund, messages.clone(), |message| *message)? {
        match refund_request {
            Ok(message) => println!("refund: {message}"),
            Err(failure) => println!("cannot judge {:?}: {}", failure.item(), failure.error().code()),
        }
    }

    let by_urgency = Rank::new("How urgent is the request?", ["routine", "urgent"])?;
    for ranked in engine.rows(&by_urgency, messages)? {
        let ranked = ranked?;
        let rank = ranked.answer()?;
        let (_, message) = ranked.record().expect("a rank row keeps its record").original();
        println!("{rank} {}", String::from_utf8_lossy(message));
    }
    Ok(())
}
```

<!-- output: fill from the real-backend capture run -->

- Records go to a worker thread, so `rows`, `filter` and `filter_items` take owned records. Collect borrowed ones into a `Vec` first.
- Each item from `rows` is a `Result`. An `Err` there means the call failed, and the iteration ends after it. A failed record is an `Ok` row whose `into_answer()` returns its error.
- A `FilterError` without an item is a call failure, and the iteration ends after it.
- Dropping the rows, or calling `cancel()` on them, stops the call. `engine.cancel(Cancel)` shares one cancel signal across calls.
- `engine.open(&question)` returns a `Call` for pushing records one at a time: send through `call.sender()` and read `call.rows()` on another thread.

## Judge JSON records

A record is a `&str`, a `String`, a `serde_json::Value` or a built `Record`. A JSON object's `id` comes back on the row's `id()`. `.field(pointer)` points the question at one field, with the pointer written as RFC 6901 says. `Record::missing()` stands for a value you do not have; it fails as `record.missing` with no backend call. Add `serde_json` with `cargo add serde_json`.

```rust
use serde_json::json;
use thinkthen::{Decide, Engine, NoAnswer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::from_env()?;
    let inbox = vec![
        json!({"id": "a1", "text": "Please refund my order."}),
        json!({"id": "a2", "text": "Where is my parcel?"}),
        json!({"id": "a3"}),
    ];
    let refund = Decide::new("Does the customer request a refund?")?.field("/text".parse()?);
    for message in engine.rows(&refund, inbox)? {
        let message = message?;
        let id = message.id().and_then(|id| id.as_str()).unwrap_or("no id");
        match message.answer() {
            Ok(is_refund) => println!("{id} answered {is_refund}"),
            Err(NoAnswer::Unsure { .. }) => println!("{id} unsure: send to a person"),
            Err(NoAnswer::Failed(error)) => println!("{id} failed: {}", error.code()),
        }
    }
    Ok(())
}
```

<!-- output: fill from the real-backend capture run -->

Message `a3` has no `/text`, so it fails with `record.missing` before any request, and the loop goes on.

## Read rows as an async stream

Add the `stream` feature: `cargo add thinkthen@0.2.0 --no-default-features --features stream`. `rows.into_stream()` gives a `RowStream`, a `futures_core::Stream` and nothing else, so `StreamExt::next` works with no clash. The crate brings no async runtime; this example uses `futures::executor::block_on` from `cargo add futures`.

```rust
use futures::StreamExt;
use thinkthen::{Decide, Engine};

async fn refunds() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::from_env()?;
    let refund = Decide::new("Does the customer request a refund?")?;
    let messages = vec!["Please refund my order.", "Where is my parcel?"];
    let mut answers = engine.rows(&refund, messages)?.into_stream();
    while let Some(answered) = answers.next().await {
        let is_refund: bool = answered?.into_answer()?;
        println!("{is_refund}");
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    futures::executor::block_on(refunds())
}
```

<!-- output: fill from the real-backend capture run -->

Dropping the stream cancels the call, as dropping the rows does.

## Choose the backend and settings

`Engine::from_env()` reads what the command reads:

- `THINKTHEN_BACKEND` picks the backend: `systemone`, the default, or `openai`.
- The backend's key variable holds the key: `THINKTHEN_API_KEY` for `systemone`, `OPENAI_API_KEY` for `openai`. `THINKTHEN_BASE_URL` changes the `systemone` address.
- `THINKTHEN_BATCH` sets how many records go in one request.

`Settings` builds an engine from values in code. Its `engine` method takes the function that looks up any variable it still needs:

```rust
use thinkthen::{BackendName, Engine, Key, Settings};

fn engine_for(key: &str) -> Result<Engine, Box<dyn std::error::Error>> {
    let engine = Settings::default()
        .with_backend(BackendName::OpenAi)
        .api_key(Key::new(key).ok_or("blank key")?)
        .engine(|name| std::env::var(name).ok())?;
    Ok(engine.named("refund-triage")?)
}
```

`engine.named("refund-triage")` puts that name on every row's `source().name()`. `Settings` also takes `model`, `url`, `batch` and `retry`. `Key` never prints its contents. [Where record text goes](service.md) describes the default service and connection settings.

## Features

- `command` (on by default) builds the `thinkthen` command. Library users turn it off with `--no-default-features`.
- `stream` adds `Rows::into_stream` and `RowStream`.

## Upgrade from 0.1

| 0.1 | 0.2.0 |
| --- | --- |
| `Question::decide("...")?.cut()` | `Decide::new("...")?` |
| `tt.decide(&question, text)?.value()` | `engine.run(&question, text)?.into_answer()?` |
| `match` on `Answer::Yes`, `No`, `Unsure` | `match` on `Ok(true)`, `Ok(false)`, `Err(NoAnswer::Unsure { .. })`, `Err(NoAnswer::Failed(error))` |
| `tt.filter(&question, items)?.value()` | `engine.filter(&question, items, \|item\| *item)?.collect::<Result<Vec<_>, _>>()?` |
| a `choices!` enum | `Choose::new(question, labels)`, with a `String` answer |
| `QuestionSet::load("form.json")` and `tt.annotate` | `Annotate::new([(name, question), ...])` and `engine.run` |
| `--features bundled-sqlite` | `--no-default-features` |

`Engine::from_env()` keeps its name. One `Engine` runs every question type.

## Reference

The [Rust API reference](https://docs.rs/thinkthen/0.2.0/thinkthen/) lists every type and method. The [Rust page on thinkthen.dev](https://thinkthen.dev/languages/rust/) shows each function's call beside the other languages.
