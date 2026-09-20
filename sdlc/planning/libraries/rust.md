# The Rust surface: goals and anti-goals

Shared rules live in [README.md](README.md). This page holds only what is particular to Rust.

## What really good looks like

A Rust user adds one line to `Cargo.toml` and the build stays fast. The crate pulls few dependencies, writes no `unsafe`, and returns one error enum. It forces no runtime on the caller and never makes anyone parse a string to learn what failed. It is also the engine the other six surfaces bind, so one API serves a person and a shim at once.

```rust
use thinkthen as tt;

let dated: Vec<Note> = tt::filter("Names a delivery date.", notes)
    .field("/text")?
    .threshold(0.9)?
    .collect()
    .await?;
```

## Goals

- One published crate named `thinkthen`, the binary behind a default `cli` feature, and `cargo add thinkthen --no-default-features` resolving no argument parser.
- Two levels. `thinkthen::*` is the user API: the eight verbs and the question setup. `thinkthen::engine` is what the shims call, and it is hidden from the documents and outside the semver promise. A check fails when the user API grows without an ADR.
- Async is the default surface, `thinkthen::blocking` mirrors it, and every conformance case runs through both.
- One `thinkthen::Error` enum from `thiserror`, with a named variant per conformance failure kind.
- A client is `Send + Sync`. A test drives several requests at once from two threads.
- The crate names an exact `rust-version` and the gate builds on it. `cargo public-api` fails a breaking release under a minor version.

## Anti-goals

- Never let a panic reach the boundary. A panic unwinding into Python or Ruby ends the host process.
- Never publish `thinkthen-core`. Ian ruled every public name is `thinkthen`, and a second name is a promise kept forever.
- Never put a heavy dependency in the default features. Argument parsing and printing belong to the binary.
- Never write `unsafe` here. The workspace forbids it, and the binding crates own the only unsafe code.
- Never return `Box<dyn Error>` or a string error. A caller branches on the kind, and a string makes them parse prose.
- Never call `block_on` inside the async API. It deadlocks under the caller's own runtime.

## Where this language wastes time

- **A fresh TLS handshake per call.** The engine pools one connection for the life of the process.
- **A runtime built per call.** `Runtime::new` starts threads and installs a reactor. `blocking` builds one and reuses it. The async API builds none.
- **Copying the evidence.** `filter` returns the very records it was handed, and the engine borrows the text into the request buffer.
- **Generics all the way down.** A verb generic at every layer compiles the engine again at each call site. Keep it outermost over one inner function that is not generic.
- **Rust has no barrier, so the waste is shape.** A slice, any `IntoIterator`, and any `Stream` reach the engine as borrowed `&str`. The widest container is the one the caller already holds.
- **One future at a time.** `for` with `.await` inside sends one request and waits. The streaming verbs keep `jobs` futures in flight. `buffered` holds the width in input order, and `buffer_unordered` yields as answers land. Memory follows the width and not the length of the input, so an endless `Stream` runs flat.
- **A map built before the call.** The engine asks an equal pair of question and evidence once inside a batch, and it answers from the cache with no request. A caller who de-duplicates first pays for a hash table and saves nothing.
- **Width set twice.** `jobs` is one number for the whole process, the engine's scheduler owns it, and two clients in one process share it.
- **The one-at-a-time form is serial.** `tt::filter` and the `Stream` form are the bulk forms, and the documents show them first.

## How little code

There is no binding tool. The crate is the engine every other surface binds. The layer over `thinkthen::engine` holds the builders that stand in for keyword arguments, the `#[derive(tt::Choose)]` macro, the `blocking` mirror, and the re-exports. It holds no threshold math, no retry, no request building, no recording format, and no second HTTP client. It ships as source on crates.io. Every Rust user has a toolchain, so nothing here is prebuilt. The other surfaces build their packages from this crate.

## Tests only this surface needs

- `--no-default-features` builds and resolves no command-line crate.
- A `trybuild` test pins the compile error from a rejected `#[derive(tt::Choose)]`.
- Every conformance case answers the same through async and through `blocking`.
- `blocking` inside a running runtime returns a named error instead of hanging.
- A bench counts rows a second through the `Stream` form against the stub backend. This number is the one every other surface is measured against, so a gap on another page is a defect in that shim.
- An endless `Stream` at width `jobs` holds memory flat over a long run.

## Open questions for the ADR

1. Which runtime does `blocking` own, and does the async API ever build one? `rust-standards.md` admits none until a measurement asks.
2. Settled by Ian on 2026-09-20: the engine is private. Every library shows the eight verbs and the question setup and nothing else. The binding crates live in this workspace and are never published, so they can call a hidden module with no promise to anyone.
3. Does `panic = "abort"` stay in the release profile? A library linked into a host process cannot abort.
4. Do the builder steps return `Result` one at a time, as `.field("/text")?` shows, or gather refusals until `send`?
5. Does the streaming form default to input order through `buffered`, or to `buffer_unordered` with the row's own key on each answer? Input order is what every other surface promises.
6. A `#[derive]` for typed choices needs a proc-macro crate, and crates.io would make it a second published name. That fights Ian's ruling of one crate. The choices are a `macro_rules!` form, a plain trait with no macro, or no typed choices in Rust. Experiment 205 tests them.
