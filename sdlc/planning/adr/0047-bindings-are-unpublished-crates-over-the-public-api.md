# ADR 0047: Bindings are unpublished crates over the public API

- Status: Draft, not reviewed. Written by Claude for tickets 0093 and 0094. Ian can overturn any item
- Date: 2026-09-24

## Context

Main is the spine (`sdlc/planning/one-line-plan-2026-09-24.md`), and each of the nine surfaces becomes a binding over the 0086 public Rust API. Three records bear on where binding code lives.

- ADR 0017 section 1: one crate named `thinkthen` holds core, engine, and command, and nothing named `thinkthen-core` or `thinkthen-cli` is ever published. Section 8 step 2: the engine exports no C symbol, and the C binding owns every exported name. Section 9: the C surface ships a shared library, a static library, and a header.
- ADR 0037, Ian on 2026-09-22: "Whatever you need, let's support everything through C." C, C++, Go, and Java use one C door.
- Ticket 0084: no second crate, C symbol, or connector is public in the `thinkthen` package. The workspace forbids `unsafe` code, and every C door and host FFI needs it.

## Decision

1. Each surface binding is its own workspace crate under `libraries/<host>` or `databases/<host>`. It sets `publish = false` and depends on `crates/thinkthen` by path with default features off. It uses only the public API, which the compiler enforces. No binding depends on another binding.
2. The C door is the crate `thinkthen-c` at `libraries/c`, library name `thinkthen`, built as `cdylib` and `staticlib`. It owns every exported symbol and the header. It never goes to crates.io. It ships in release archives, as ADR 0017 section 9 says.
3. A binding crate may relax the workspace's `unsafe_code = "forbid"` for its FFI edge alone. `thinkthen` keeps the forbid.
4. No shared binding-helper crate. Host-neutral pieces live in the public API (ticket 0095: host deadline numbers, `ErrorKind::name`, result JSON). Each binding keeps one panic guard and one error-kind table at its own edge (error-index rows R1-31 and R2-31).
5. Each native package links its own copy of the engine. The width cap, counters, cache coordination, and fork state hold per loaded copy in one process. A Python wheel and a DuckDB extension in one process have two caps. Every surface page says so. A cross-image cap is deferred past 0.1, because it needs a process-global primitive outside Rust statics.
6. The Polars surface at 0.1 is the Python Polars door, per the 2026-09-21 Polars ruling in ADR 0017. The branch's Rust Polars `Series` door is deferred past 0.1. A Rust user maps a `Series` of text into `decide_many`.
7. Relate from rows (all three databases): the binding dedupes rows by name and kind in first-seen order and calls `relate` once. It maps each edge back to every row with that pair. The engine's 255 cap counts unique pairs.

## Why this is consistent with ADR 0017 and ADR 0037

ADR 0017 bans publishing a split of the engine's layers. A binding crate adds no layer and holds no engine logic, and it is never published to crates.io, so the one Rust package stays `thinkthen`. ADR 0017 already expects a C binding that owns the exported names outside the engine. ADR 0037 asks for everything through C. A separate crate is the only way to export C symbols while `thinkthen` forbids `unsafe` and exports none. The rejected alternative put the C door behind a feature of `thinkthen`. That puts `unsafe` and exported symbols into the published package and breaks 0084's rule.

## Every surface ticket

Each surface ticket follows ticket 0093's pattern and changes only host code.

- Replace the connector, engine static, and stand-in path dependencies with the path dependency above.
- Map types per section 3.1 of `sdlc/planning/surfaces-port-guide.md`. Use `CallOptions::interrupt` for the host's interrupt channel, `deadline_seconds` or `deadline_millis` for host numbers, and one-question `annotate` for bulk `choose`, `score`, and `tag`.
- Run the shared cases through the 0092 loopback backend. Re-run the surface's error-index rows against the real engine. Report a skipped case as not run.
- Come under main's lint, deny, policy, and ratchet, and register in the surface rung that refuses a missing surface.
- Write any ruling that lived only in notes as an ADR (R2-29).

## Consequences

- 0077's and 0078's duplicate-image blocker closes as a stated limit.
- The ADR 0037 header moves from `contract/include/thinkthen.h` to `libraries/c/include/thinkthen.h` in ticket 0094.
- Ian can overturn item 5 (per-copy cap) and item 6 (Rust Polars door deferred). Both appear in the "Needs Ian" list of the 2026-09-24 spine design pass.
