---
flow: build
priority: 86
opens: Cargo.toml crates/thinkthen/Cargo.toml crates/thinkthen/src/lib.rs crates/thinkthen/src/public crates/thinkthen/tests sdlc/scripts/package sdlc/scripts/install sdlc/scripts/lint sdlc/scripts/policy.py sdlc/ratchet.json README.md sdlc/planning/libraries/rust.md
---

# 0086: Expose the public Rust API

Status: draft, design not reviewed. Owner: Claude. Was marked ready

## Outcome and authority

Expose the public contract frozen by ticket 0084 over the real host-neutral facade built by ticket 0085. One blocking `thinkthen` crate exports `Engine`, the typed question and question-set builders, typed descriptions, call options, cancellation and deadlines, the six error kinds, typed results, all ten engine methods (`decide`, `choose`, `score`, `tag`, `filter`, `rank`, `find`, `annotate`, `recognize`, and `relate`), and only the conveniences approved by 0084. The public layer delegates to the 0085 facade. It does not copy question parsing, planning, scheduling, transport, retry, cache, recording, counter, recognition, or relation logic.

ADR 0017 and `sdlc/planning/libraries/rust.md` govern the implementation. Ticket 0084 owns the exact public names, signatures, builder states, result types, and convenience inventory. Ticket 0085 owns the private typed execution contract. This ticket implements those records without reopening them. A contradiction stops for an 0084 amendment instead of creating a second shape in code. Ian can overturn the public contract through that durable record.

## Public boundary

Keep one package and one published name, `thinkthen`. The library and command remain targets of that package, and the command remains behind the default `cli` feature. Add no `thinkthen-core`, `thinkthen-api`, helper package, derive package, proc-macro target, async mirror, runtime, or C export. The public root re-exports the frozen user types and functions. Private engine modules remain unreachable to an external crate.

The API stays blocking. `Engine` construction validates settings and creates no wire traffic. Explicit warming exists only if 0084 froze it. Every explicit engine call reaches the 0085 facade. Any approved module-level convenience uses one lazily initialized process-wide engine and never constructs an engine per call. Convenience calls and explicit-engine calls share results, errors, cache behavior, counters, the process width gate, fork repair, cancellation, and deadline behavior.

Implement the frozen builders directly in this crate. `Description` supports the ruled typed fields and additional ordered keys. Question, question-set, recognition, and relation builders preserve the production grammar and canonical bytes. Builder steps that can fail return `Result` at the failing step. The declarative `choices!` macro and `Choice` trait live in `thinkthen`; the macro emits the frozen inherent helpers and trait implementation. The first API has no derive and no generated user type. Structured annotate forms use the explicit builder fixed by the typed-builder ruling.

Call options carry only the frozen caller controls. Public cancellation is a clonable token safe to set from another thread. A deadline is an absolute instant derived from the caller's budget at call time. No public poll callback or host signal hook appears; bindings add that behavior later over the private facade. A past deadline or already-cancelled token sends nothing. The six public kinds remain `usage`, `backend`, `local`, `cancelled`, `deadline`, and `defect`, with the frozen retryable signal and structured source data. No public signature returns a string error or `Box<dyn Error>`.

Return the frozen typed values and detailed results without converting through command JSON. Preserve `Answer::Unsure`, ordered bulk results, partial per-question failures, request digests, sends, cache state, model, usage, entity strength, relation probability, and `source`/`target` exactly as 0084 defines them. Slice and iterator entry points use the same 0085 batch spine and return the same values in input order. They do not collect an unbounded iterator before work starts.

## Scope and exclusions

Allowed: the public Rust modules and re-exports; thin conversions to and from the 0085 facade where the frozen public types require them; rustdoc and checked examples; external-crate compile-pass and compile-fail fixtures; public inventory enforcement; focused loopback and replay tests; the existing package, policy, install, lint, and ratchet checks; source-package metadata and Rust-library documentation needed to package this crate from source.

Source-package metadata may add the accurate library-aware description, README, repository, homepage, documentation URL, keywords, and categories required by the accepted package check. Keep the version at its pre-release value and keep `publish = false`. Package the existing source crate only. Do not build or describe release archives, prebuilt binaries, shared or static libraries, headers, installers, taps, download scripts, checksums, signatures, uploads, registry ownership, or publication.

Excluded: C and every other language or database surface; the `surfaces` branch and its integration; FFI symbols or `unsafe`; command behavior, output, diagnostics, or exit codes except a mechanical call-through needed to keep the shared 0085 facade single; new result semantics; a second parser, scheduler, transport, cache, or counter; async APIs; derive or procedural macros; a second crate; live or paid calls; release workflows and GitHub Actions changes.

## Deterministic acceptance

- An external consumer crate builds offline against the packaged path with default features disabled and exercises every frozen public type, builder, result, engine method, and approved convenience. The same examples compile as doctests. The command parser and command-only modules do not compile in that consumer graph.
- `cargo metadata` and `cargo tree` with `--no-default-features` prove one `thinkthen` package, one library target, no proc-macro target, no second workspace or path package, and no `clap`, `csv-core`, `nix`, or other CLI-only normal dependency. The existing default-feature command build remains green.
- Compile-fail fixtures pin the 0084 type-state refusals, including a banded question passed to `filter`, a wrong question kind, an incomplete builder, and access to private engine internals. Diagnostics are matched by stable substance rather than compiler line decoration. Compile-pass fixtures cover string and built-question inputs, `choices!`, typed descriptions, question sets, all ten methods, and every approved convenience.
- Compile-time assertions prove `Engine` and every frozen shared handle are `Send + Sync`. A loopback test shares one engine between at least two caller threads, observes the one process-wide width rule, receives ordered typed results, and leaves no engine worker alive after both calls return.
- Every applicable shared conformance success and failure case runs through the public Rust layer with no key and no outside network. Explicit-engine and approved convenience paths produce equal values, details, error kinds, retryability, request counts, cache counts, and digests. The convenience path reuses one lazy engine; a counted local server proves repeated calls do not rebuild transport state.
- Every bulk conformance case produces identical ordered results from the frozen slice form and from an iterator that yields the same borrowed texts. A guarded iterator proves bounded pull-ahead at the configured width, and a long finite stand-in run proves memory does not grow with input length. Partial results and stable ties retain input order on both paths.
- Public calls preserve 0085's controls. Counted listener tests cover cancellation before a call, cancellation during gate wait, cancellation during a batch, a past deadline, a deadline during retry wait and blocking send, conflicting explicit width, and a child call after fork. Nothing starts after observed cancellation or deadline, spent calls send zero, already-sent work finishes under the accepted contract, and counters and cache events match the real attempts.
- No panic crosses a public engine method or approved convenience. A private test seam injects a panic below the public door and receives the frozen `defect` error while the process continues. The package rung proves a release library uses `panic = "unwind"`; the existing dedicated command release build remains `panic = "abort"`. No fault hook becomes public.
- A checked public API inventory matches the complete 0084 allowlist and fails on an added, removed, or signature-changed public item. It includes macro and trait exports and excludes private engine paths and the internal doctest harness. The check runs from `lint`; a planted unexpected export proves the check fails. Breaking the inventory requires an ADR and an intentional baseline update.
- `cargo package --locked --offline --allow-dirty` finishes without missing-metadata warnings. The package list contains the one crate's required source, license, README, and tests, and no sibling crate, worktree file, secret, recording credential, build output, release archive, installer, or surface package. Unpack the generated crate into a temporary directory and build its library with `--no-default-features` offline.
- Focused tests fail for the absent public surface before implementation and pass afterward. Run `sdlc/scripts/install`, `sdlc/scripts/lint`, `sdlc/scripts/test`, and `sdlc/scripts/spec` sequentially with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset, followed by `git diff --check`. No live endpoint or paid call runs.

## Budgets

Touch at most fourteen production Rust files and add at most 1,200 nonblank production Rust lines. Touch at most twelve Rust test, fixture, and example files and add at most 1,600 nonblank lines there. Package and inventory enforcement may add at most 260 nonblank script or baseline lines. Documentation and manifests may touch at most six files and add at most 220 net nonblank lines. Keep every Rust file below the existing 500-nonblank-line ceiling.

Add no runtime dependency. The one permitted dependency change is the compile-test development dependency already selected by the 0084 contract, if 0084 requires one; otherwise use the repository's direct compiler probes and add none. Any tool used for public inventory is pinned by the install rung and is not a crate runtime dependency. The ratchet increase equals the measured Rust increase. The implementation record names why each added block earns its lines and where duplicate facade, parser, and result conversion code was removed first.

Stop and re-score before crossing a budget, adding a normal dependency, exposing a name absent from 0084, changing the 0085 facade, weakening a control or panic guarantee, collecting an unbounded input, or touching another surface.

## Dependencies

Ticket 0086 starts only after tickets 0084, 0085, and 0078 land. Ticket 0084 freezes the exact compile contract and public allowlist. Ticket 0085 supplies one private host-neutral facade over the real engine for all ten functions and proves its conformance, ordering, partial results, controls, cache, counters, and no-send rules. Ticket 0078 completes fork recovery and host signal ownership at the private boundary. Through 0085, this ticket also depends on landed 0080 and 0081 result shapes and the preceding cancellation, deadline, and process-wide width controls. Do not substitute the `surfaces` stand-in contract for any landed predecessor.

The existing one-crate boundary from 0055, shared cases from 0052, partial-result contract from 0054, structured descriptions from 0069, package checks, ADR 0017, and the typed-builder ruling remain direct authorities. Ticket 0086 precedes C and all real-engine surface integration, installed-artifact work, and publication.

## Complexity

Contract 4; state and timing 3; reach 4; proof 4; cost of error 4; total 19. Minimum floor: level 4 for a semver-bearing public API over shared concurrency, cancellation, fork, panic, and package boundaries. Final level: 4. Reasons: a wrong signature becomes a lasting source contract, an accidental export widens the promise, a missed unwind can terminate a host, and a convenience that rebuilds or bypasses the real engine changes cost and correctness. Selected implementation model: Soul with high reasoning. Independent design and code review use separate review sessions and reconcile every exported item against 0084.

Re-score if implementation changes public semantics, requires a new dependency or crate, or reaches into C, another language, release artifacts, or publication.
