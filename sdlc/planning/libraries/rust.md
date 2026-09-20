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
- Two levels. `thinkthen::*` is the user API of the agreed grammar, and `thinkthen::engine` is what the shims call. A check fails when the user API grows without an ADR.
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

## How little code

There is no binding tool. The crate is the engine every other surface binds. The layer over `thinkthen::engine` holds the builders that stand in for keyword arguments, the `#[derive(tt::Choose)]` macro, the `blocking` mirror, and the re-exports. It holds no threshold math, no retry, no request building, no recording format, and no second HTTP client. It ships as source on crates.io. Every Rust user has a toolchain, so nothing here is prebuilt. The other surfaces build their packages from this crate.

## Tests only this surface needs

- `--no-default-features` builds and resolves no command-line crate.
- A `trybuild` test pins the compile error from a rejected `#[derive(tt::Choose)]`.
- Every conformance case answers the same through async and through `blocking`.
- `blocking` inside a running runtime returns a named error instead of hanging.

## Open questions for the ADR

1. Which runtime does `blocking` own, and does the async API ever build one? `rust-standards.md` admits none until a measurement asks.
2. Is `thinkthen::engine` public and covered by semver, or private with the binding crates inside this workspace?
3. Does `panic = "abort"` stay in the release profile? A library linked into a host process cannot abort.
4. Do the builder steps return `Result` one at a time, as `.field("/text")?` shows, or gather refusals until `send`?
