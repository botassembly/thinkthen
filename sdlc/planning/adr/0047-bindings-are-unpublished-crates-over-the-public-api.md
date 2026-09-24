# ADR 0047: Bindings are unpublished crates over the public API

- Status: Draft, revised after design review. Item 5 awaits Ian. Written by Claude for tickets 0093 and 0094. Ian can overturn any item
- Date: 2026-09-24

## Context

Main is the spine (`sdlc/planning/one-line-plan-2026-09-24.md`), and each of the nine surfaces becomes a binding over the 0086 public Rust API. Four records bear on where binding code lives.

- ADR 0017 section 1: one crate named `thinkthen` holds core, engine, and command, and nothing named `thinkthen-core` or `thinkthen-cli` is ever published. Section 8 step 2: the engine exports no C symbol, and the C binding owns every exported name. Section 9: the C surface ships a shared library, a static library, and a header.
- ADR 0037, Ian on 2026-09-22: "Whatever you need, let's support everything through C." C, C++, Go, and Java use one C door.
- Ticket 0084: no second crate, C symbol, or connector is public in the `thinkthen` package.
- `thinkthen` has `#![forbid(unsafe_code)]` in `lib.rs`, and the root lint table pins `unsafe_code = "forbid"`. Every C door and host FFI needs `unsafe`.

## Decision

1. Each surface binding is its own Cargo workspace with its own `Cargo.lock`, under `libraries/<host>` or `databases/<host>`, as the branch did. It sets `publish = false` and depends on `crates/thinkthen` by path with default features off. It uses only the public API, which the compiler enforces. No binding depends on another binding. Host toolchains (pyo3, napi, pgrx, DuckDB, SQLite, R, Ruby) therefore never enter the root lock, `policy.py`'s dependency tables, or a root gate. R's offline tarball gets a standalone crate.
2. The root workspace sets `exclude = ["libraries", "databases"]` and `default-members = ["crates/thinkthen"]`. A plain root `cargo build` or `cargo test` compiles `thinkthen` alone. `--workspace` adds only the test-only member `conformance/backend` (ticket 0092). A test-only, unpublished root member is allowed under policy, deny, and the ratchet.
3. Lints. `thinkthen` keeps `#![forbid(unsafe_code)]` in `lib.rs`, and the root lint table does not change. Each binding workspace declares `unsafe_code = "deny"` in its own lint table. Only its FFI module carries `#[allow(unsafe_code, reason = "…")]`. `policy.py` gains one accepted binding lint table and one planted failure: `unsafe` outside the FFI module fails.
4. The ratchet. The shared `ratchet.mjs` reader stays unchanged. Each binding workspace keeps its own `ratchet.json` beside its `Cargo.toml`, read by the same reader over that binding's source. The surface rung runs it. Code under `libraries/` and `databases/` never escapes a ceiling, and the root ceiling keeps counting `crates` only.
5. **Awaiting Ian.** Each native package links its own copy of the engine. ADR 0017 section 2, which Ian accepted, says "One width gate for the process". Two copies in one process (a Python wheel and a DuckDB extension) break that claim unless something is done. The filed decision is `notes/todos/2026-09-24-thinkthen-width-cap-with-two-library-copies.md`. The options:
   - (a) State one cap per loaded copy on every surface page. Cost: a page sentence per surface. A host with two copies can run up to twice the width.
   - (b) Refuse a second copy. The first copy claims a process-wide marker, and a second copy's first call fails with `usage`. Cost: a small cross-copy check. A user who loads both packages gets an error.
   - (c) Share one cap across copies with a process-global primitive, such as a named semaphore. Cost: new platform code outside Rust statics and its fork story.
   Recommendation: (a) for 0.1, and (c) later if a user needs it. Counters and fork state are per copy under every option. Durable cache coordination already crosses copies through operating-system file locks.
6. The C door is the crate `thinkthen-c` at `libraries/c`, library name `thinkthen_c`, crate types `cdylib` and `staticlib` only. The distinct name avoids a second `libthinkthen.rlib` beside its dependency. Its build sets the shared library's soname to `libthinkthen.so.0` (install name on macOS). The release step names the files `libthinkthen.so` and `libthinkthen.a`. It owns every exported symbol and the header. It never goes to crates.io. It ships in release archives, as ADR 0017 section 9 says.
7. No shared binding-helper crate. Host-neutral pieces live in the public API (ticket 0095: host deadline numbers, `ErrorKind::name`, result JSON). Each binding keeps one panic guard and one error-kind table at its own edge (error-index rows R1-31 and R2-31). Ticket 0094 is the reference for the FFI edge: the guard, the error table, and the one `unsafe` module.
8. The Polars surface at 0.1 is the Python Polars door, per the 2026-09-21 Polars ruling in ADR 0017. The branch's Rust Polars `Series` door is deferred past 0.1. A Rust user maps a `Series` of text into `decide_many`.
9. Relate from rows (all three databases): the binding dedupes rows by name and kind in first-seen order and calls `relate` once. It maps each edge back to every row with that pair. The engine's 255 cap counts unique pairs.

## Why this is consistent with ADR 0017 and ADR 0037

ADR 0017 bans publishing a split of the engine's layers. A binding crate adds no layer and holds no engine logic, and it is never published to crates.io, so the one Rust package stays `thinkthen`. ADR 0017 section 8 step 2 already puts every exported C name in a C binding outside the engine, and 0084 keeps C symbols out of `thinkthen`. ADR 0037 asks for everything through C. A Cargo feature on `thinkthen` cannot add a `cdylib` crate type. It would also put `unsafe` and exported symbols into the published package. So the C door is a separate crate.

## Every surface ticket

Each surface ticket follows ticket 0093's pattern and changes only host code.

- Replace the connector, engine static, and stand-in path dependencies with the path dependency above, inside the binding's own workspace.
- Map types per section 3.1 of `sdlc/planning/surfaces-port-guide.md`. A binding may use `CallOptions::interrupt` for its host's interrupt channel, or keep the worker-thread pattern. Use `deadline_seconds` or `deadline_millis` for host numbers, and one-question `annotate` for bulk `choose`, `score`, and `tag`.
- Its `check.sh` receives the 0092 loopback port and runs offline. A missing host toolchain reports "not run", never "pass" (R6-2). Heavy Docker checks run from the surface rung, never from `test`.
- Run the shared cases through the 0092 backend. Re-run the surface's error-index rows against the real engine, each with a planted bug that turns its test red. Report a skipped case as not run.
- Come under its own lint table, deny, and ratchet file, and register in the surface rung that refuses a missing surface.
- Write any ruling that lived only in notes as an ADR (R2-29).

## Consequences

- 0077's duplicate-image boundary stays open until Ian answers item 5.
- The ADR 0037 header moves from `contract/include/thinkthen.h` to `libraries/c/include/thinkthen.h` in ticket 0094.
- Ian can overturn every item. Items 5 and 8 touch his rulings most closely.
