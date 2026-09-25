# thinkthen-polars

Ask `thinkthen` questions of a Polars `Series` or `DataFrame` from Rust. The crate adds one trait, `PolarsEngine`, to your own `thinkthen::Engine`. Each method reads a text column in place and makes one engine call over the whole column. That call takes the same batch path as a slice of strings, at the same throttle, and the answers come back in input order.

This door supports Polars 0.55 only. Polars changes its Rust API between minor versions. The crate re-exports the `polars` and `thinkthen` it was built with, so `thinkthen_polars::polars` names the exact version your `Series` must come from. It needs Rust 1.95, the repository's toolchain. It is not published yet.

## Example

```rust,no_run
use thinkthen::{CallOptions, EngineBuilder, Question};
use thinkthen_polars::polars::prelude::{NamedFrom, Series};
use thinkthen_polars::PolarsEngine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = EngineBuilder::from_env()?
        .base_url("http://127.0.0.1:8080/v1")?
        .model("jev-latest")?
        .throttle(8)?
        .max_requests(Some(10_000))?
        .cache_at("answers")?
        .cache_bytes(1 << 30)?
        .build()?;
    let notes = Series::new("note".into(), ["Please refund my order.", "Thanks, all good."]);
    let refund = Question::decide("Does the writer ask for a refund?")?.cut();
    let asked = engine.decide_series(&refund, &notes, CallOptions::new())?;
    assert_eq!(asked.len(), 2);
    Ok(())
}
```

The address, key, and cache come from the environment unless a setter overrides them. `default_cache` and `no_cache` are the other two cache setters. The throttle is the most requests in flight at once. It holds per loaded copy of the library, so a program that loads two copies can run up to twice the throttle.

## The five methods

| Method | Question | Result |
| --- | --- | --- |
| `decide_series` | a `Question` or `BandedQuestion` | `Boolean`, null for not sure |
| `choose_series` | a choose `Question` | `String`, null where nothing fits |
| `score_series` | a score `Question` | `Float64`, the position from 0 to one less than the number of levels |
| `tag_series` | a tag `Question` | `List(String)` |
| `annotate_frame` | a `QuestionSet` and the name of the text column | your frame plus one column per question, in set order |

Every method takes `CallOptions`. One deadline, cancel token, and interrupt check cover the whole column. A cancel stops new requests, and sent requests finish. The call runs on your thread and starts no worker of its own.

In a frame, a decide column is `Boolean`, a choose column `String`, a score column `Float64`, and a tag column `String` holding the JSON array text, as the Python door writes it. When any row's answer to one question failed, that question's whole column becomes `String`. Each cell then holds the member's text from `AnnotatedRecord::value_json`, unchanged. A failed cell holds the engine's marker, such as `{"failed":{"kind":"backend","cause":"missing_probability"}}`. A choice keeps its plain label, and a not-sure or nothing-fits cell stays null. Your own columns come back unchanged.

A failed row in a series call ends the call with the engine's `Backend` error. The engine refuses a reply with no usable answer, and a series asks one question. A frame ends the same way when its set has one member, or when a request chunk's only answer failed.

Labels known only at run time come from `Question::choose_labels`, `Question::tag_labels`, or `Question::from_json`. A typed `ChooseQuestion<C>` or `TagQuestion<C>` goes into a set through `QuestionSetBuilder::choose` or `tag`, and `annotate_frame` asks it.

## Refusals

Each refusal happens before any request and returns `Error::Usage`:

- `the column {name} is {dtype}, not text`. Cast a `Categorical` column to `String` first.
- `the column holds nulls; the engine needs text, and NA rows are the caller's to drop`
- `the frame holds no column {on}`
- `the frame already holds a column named {name}`
- `{method} needs a {verb} question, and this one is a {kind} question`

An empty column returns an empty column of the method's type and sends nothing. `Error::kind` gives the engine's kind, or `Usage` or `Defect` for the door's own refusals. No message or `Debug` line holds a text from your column.

## Licenses

`deny.toml` equals the repository's file plus four license exceptions for crates in the Polars tree. Each license is permissive and carries no copyleft term.

- `foldhash` (Zlib), the default hasher of `hashbrown` 0.17 under `polars-arrow` and `polars-compute`.
- `slotmap` (Zlib), under `polars-async` and `polars-utils`.
- `xxhash-rust` (BSL-1.0), a direct dependency of `polars-core`.
- `ar_archive_writer` (Apache-2.0 WITH LLVM-exception), a build dependency of `psm` under `stacker` under `polars-utils`. It runs at build time only.

## Checks

`check.sh` runs `cargo fmt`, Clippy, and the tests offline, with a fake key and a closed loopback address. It prints "not run" and exits 77 only when cargo's cache lacks the locked crates.
