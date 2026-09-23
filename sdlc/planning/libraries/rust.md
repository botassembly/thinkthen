# The Rust surface: goals and anti-goals

Shared rules live in [README.md](README.md). This page holds only what is particular to Rust.

## What really good looks like

A Rust user adds one line to `Cargo.toml` and the build stays fast. The crate pulls few dependencies, writes no `unsafe`, and returns one error enum. It forces no runtime on the caller and never makes anyone parse a string to learn what failed. It is also the engine the other six surfaces bind, so one API serves a person and a shim at once.

```rust
use thinkthen as tt;

impl tt::Evidence for Note {
    fn evidence(&self) -> Result<&str, tt::Error> {
        Ok(&self.text)
    }
}

let engine = tt::Engine::from_env()?;
let question = tt::Question::decide("Names a delivery date.")?.cut_at(0.9)?;
let dated: Vec<Note> = engine
    .filter(&question, notes)
    .collect::<Result<_, _>>()?;
```

Rust is blocking (ADR 0017 pick 7). Plain calls return `Result`, no `.await` exists, and the engine holds no async runtime and no resident thread. Experiment 211 matched the async stand-in's width bench at 9.666 s with zero threads held between calls.

## Goals

- One published crate named `thinkthen`, the binary behind a default `cli` feature, and `cargo add thinkthen --no-default-features` resolving no argument parser.
- One crate-root API exposes the ten functions and question setup. The `engine` module stays private. A separate binding crate consumes the public crate-root API after 0086; only a shim compiled inside the `thinkthen` crate may call private modules. A check fails when the public API grows without an ADR.
- Blocking is the only surface (ADR 0017 pick 7 and section 2). No async mirror, no runtime, `block_on` nowhere. Scoped threads run a batch at width and vanish when it ends.
- A decision is `Question` for a cut and `BandedQuestion` for a band, so `filter` with a band is unrepresentable. `ChooseQuestion<C>` and `TagQuestion<C>` retain the `Choice` type from builder through call. Parsed choices and tags bind through checked `Question::into_choose::<C>` and `into_tag::<C>`. `load` returns `LoadedQuestion::Question` or `LoadedQuestion::Banded`. The decide builder's `.cut()` closes with the grammar default (205).
- One `thinkthen::Error` enum from `thiserror`, with the six named variants of ADR 0017 section 3. Each carries a safe `ErrorDetail`; callers match the variant or use `kind()`, and `retryable()` is true only for the ruled backend failures.
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
- **Rust has no barrier, so the waste is shape.** Any `IntoIterator` reaches the engine through the record's `Evidence` implementation. `String` and `&str` work without an implementation. The widest container is the one the caller already holds. The batch spine returns results in order, and `decide_many` uses that same spine (ADR 0017 section 8, step 2).
- **One request at a time in a caller's loop.** A `for` loop that judges each record sends one request and waits. The bulk verbs hand the iterator to the engine, which runs it at width `jobs` and keeps input order. Memory follows the width and not the length of the input, so an endless iterator runs flat.
- **A map built before the call.** The engine sends an equal pair of question and evidence once when the batch shares a cache, and it answers an existing entry with no request. A caller who de-duplicates first pays for a hash table and saves nothing.
- **Width set twice.** `jobs` is one number for the whole process, the engine's scheduler owns it, and two clients in one process share it.
- **The one-at-a-time form is serial.** `tt::filter` and `decide_many` are the bulk forms, and the documents show them first.

## How little code

There is no binding tool. Separate binding crates call the public `thinkthen::*` surface after 0086; they cannot call `core` or `engine`. A shim inside the package may call private owners. The crate root holds typed builders, question types, and `choices!` with `Choice`. The dropped derive would require a proc-macro crate and second published name. This layer holds no threshold math, retry, request building, recording format, or second HTTP client.

## Tests only this surface needs

- `--no-default-features` builds and resolves no command-line crate.
- Dependency-free compile fixtures pin stable required phrases from rejected question values, including `filter` with `BandedQuestion`.
- Compile failures prove a `ChooseQuestion<Team>` or `TagQuestion<Topic>` cannot be called with another `Choice` type.
- Every conformance case answers identically through the slice form and through an iterator.
- An endless iterator at width `jobs` holds memory flat over a long run.
- A bench counts rows a second through the bulk form against the stub backend. This number is the one every other surface is measured against, so a gap on another page is a defect in that shim.

## Open questions for the ADR

1. Answered by ADR 0017 section 2: no runtime at all. `rust-standards.md` admits none, and 211 measured the blocking engine at parity.
2. Settled by Ian on 2026-09-20 and amended by the authorized additions: the engine module is private. Separate binding crates use the public crate-root API after 0086. Only code compiled inside `thinkthen` may call private owners. Every library shows the ten functions and question setup and nothing else.
3. Answered (ADR 0017 section 9): library artifacts build with `panic = "unwind"`, and only the command binary may abort.
4. Answered by experiment 205: builder steps return `Result` at once. Refusing at the step costs one `?` and points at the exact step that is wrong; the builder's refusals come from the core's own validator. An engine `from_parts` constructor may drop the double parse on first build.
5. Input order. The batch spine returns results in order (ADR 0017 section 8), which is what every other surface promises.
6. Answered by experiment 205: the `choices!` macro plus the `Choice` trait, no derive, no second published name.
