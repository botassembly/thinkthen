# The Rust surface: goals and anti-goals

Shared rules live in [README.md](README.md). This page holds only what is particular to Rust.

## What really good looks like

A Rust user adds one line to `Cargo.toml` and the build stays fast. The crate pulls few dependencies, writes no `unsafe`, and returns one error enum. It forces no runtime on the caller and never makes anyone parse a string to learn what failed. It is also the engine the other six surfaces bind, so one API serves a person and a shim at once.

```rust
use thinkthen as tt;

let engine = tt::Engine::from_env();
let dated: Vec<Note> = engine
    .filter("Names a delivery date.", notes)
    .field("/text")?
    .threshold(0.9)?
    .collect()?;
```

Rust is blocking (ADR 0017 pick 7). Plain calls return `Result`, no `.await` exists, and the engine holds no async runtime and no resident thread. Experiment 211 matched the async stand-in's width bench at 9.666 s with zero threads held between calls.

## Goals

- One published crate named `thinkthen`, the binary behind a default `cli` feature, and `cargo add thinkthen --no-default-features` resolving no argument parser.
- Two levels. `thinkthen::*` is the user API: the eight verbs and the question setup. `thinkthen::engine` is what the shims call, and it is hidden from the documents and outside the semver promise. A check fails when the user API grows without an ADR.
- Blocking is the only surface (ADR 0017 pick 7 and section 2). No async mirror, no runtime, `block_on` nowhere. Scoped threads run a batch at width and vanish when it ends.
- A question is `Question` for a cut and `BandedQuestion` for a band, so the third arm is unavoidable and `filter`-with-a-band is unrepresentable. `load` returns `Loaded::Cut` or `Loaded::Banded` and the caller matches. The builder takes a `.cut()` step that closes with the grammar's default rule (205).
- One `thinkthen::Error` enum from `thiserror`, with a named variant per conformance failure kind — the six of ADR 0017 section 3, each carrying `retryable`.
- A client is `Send + Sync`. A test drives several requests at once from two threads.
- The crate names its `rust-version` honestly: the core needs edition 2024 (let chains), so the floor is 1.88 at minimum, and the repo pins 1.93.1 (205). `cargo public-api` fails a breaking release under a minor version.

## Anti-goals

- Never let a panic reach the boundary. A panic unwinding into Python or Ruby ends the host process. Library artifacts build with `panic = "unwind"`; only the command binary may abort (ADR 0017 section 9). The door's `catch_unwind` only works in an unwind build, so that corollary stands beside the anti-goal (205).
- Never publish `thinkthen-core`. Ian ruled every public name is `thinkthen`, and a second name is a promise kept forever.
- Never put a heavy dependency in the default features. Argument parsing and printing belong to the binary.
- No `unsafe` in the user layer. The engine's C door owns the crate's only two `unsafe` functions, and edition 2024 requires the `unsafe` block inside `tt_free_string`; the audit knows exactly where to look (205).
- Never return `Box<dyn Error>` or a string error. A caller branches on the kind, and a string makes them parse prose.

## Where this language wastes time

- **A fresh TLS handshake per call.** The engine pools one connection for the life of the process, sized to the width gate. 211 measured the HTTP client's default pool of ten idle connections churning 488 connections where 33 serve, so the pool is sized to the gate.
- **No runtime exists to build.** 211 measured zero threads held between calls where the async stand-in parked 16. The transient spike is one worker per in-flight request plus the HTTP client's resolver, one short-lived thread per simultaneous dial; a synchronous numeric resolver halves the ramp, and the spike never persists.
- **Copying the evidence.** `filter` returns the very records it is handed, and the engine borrows the text into the request buffer.
- **Generics all the way down.** A verb generic at every layer compiles the engine again at each call site. Keep it outermost over one inner function that is not generic.
- **Rust has no barrier, so the waste is shape.** A slice, any `IntoIterator`, and any `Iterator` reach the engine as borrowed `&str`. The widest container is the one the caller already holds. The batch spine takes an iterator and returns results in order, and `decide_many` over a slice is a thin wrapper (ADR 0017 section 8, step 2).
- **One request at a time in a caller's loop.** A `for` loop that judges each record sends one request and waits. The bulk verbs hand the iterator to the engine, which runs it at width `jobs` and keeps input order. Memory follows the width and not the length of the input, so an endless iterator runs flat.
- **A map built before the call.** The engine sends an equal pair of question and evidence once when the batch shares a cache, and it answers an existing entry with no request. A caller who de-duplicates first pays for a hash table and saves nothing.
- **Width set twice.** `jobs` is one number for the whole process, the engine's scheduler owns it, and two clients in one process share it.
- **The one-at-a-time form is serial.** `tt::filter` and `decide_many` are the bulk forms, and the documents show them first.

## How little code

There is no binding tool. The crate is the engine every other surface binds. The layer over `thinkthen::engine` holds the builders that stand in for keyword arguments, the `choices!` macro with the `Choice` trait (the `#[derive(tt::Choose)]` plan is dropped: it needs a proc-macro crate and a second published name, against the ruling; the macro emits inherent `from_label`/`labels` methods beside the trait impl, and the macro and the module take different names so one `use` tree cannot collide), the re-exports, and the question types. It holds no threshold math, no retry, no request building, no recording format, and no second HTTP client. It ships as source on crates.io. Every Rust user has a toolchain, so nothing here is prebuilt. The other surfaces build their packages from this crate. The line ceiling is set per module: the user layer measured 484 lines with two of eight verbs, and all eight land it near 700–800 code lines (205). When the core inlines as a module, its doctests die and its unused re-exports warn; the integration rewrites them as unit tests or accepts them with a comment, and prunes what the engine does not use.

## Tests only this surface needs

- `--no-default-features` builds and resolves no command-line crate.
- A `trybuild` test pins the compile error from a rejected question value, including `filter`-with-a-band under `BandedQuestion`.
- Every conformance case answers identically through the slice form and through an iterator.
- An endless iterator at width `jobs` holds memory flat over a long run.
- A bench counts rows a second through the bulk form against the stub backend. This number is the one every other surface is measured against, so a gap on another page is a defect in that shim.

## Open questions for the ADR

1. Answered by ADR 0017 section 2: no runtime at all. `rust-standards.md` admits none, and 211 measured the blocking engine at parity.
2. Settled by Ian on 2026-09-20: the engine is private. Every library shows the eight verbs and the question setup and nothing else. The binding crates live in this workspace and are never published, so they can call a hidden module with no promise to anyone.
3. Answered (ADR 0017 section 9): library artifacts build with `panic = "unwind"`, and only the command binary may abort.
4. Answered by experiment 205: builder steps return `Result` at once. Refusing at the step costs one `?` and points at the exact step that is wrong; the builder's refusals come from the core's own validator. An engine `from_parts` constructor may drop the double parse on first build.
5. Input order. The batch spine returns results in order (ADR 0017 section 8), which is what every other surface promises.
6. Answered by experiment 205: the `choices!` macro plus the `Choice` trait, no derive, no second published name.
