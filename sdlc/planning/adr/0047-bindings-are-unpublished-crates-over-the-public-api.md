# ADR 0047: Bindings are unpublished crates over the public API

- Status: Accepted 2026-09-24 with ticket 0093 after code review; design accepted 2026-09-24 after re-review. Item 5 settled 2026-09-24 by the owner. Owner: Claude. Written by Claude for tickets 0093 and 0094. Ian can overturn any item
- Date: 2026-09-24

## Context

Main is the spine (`sdlc/planning/one-line-plan-2026-09-24.md`), and each of the nine surfaces becomes a binding over the 0086 public Rust API. Four records bear on where binding code lives.

- ADR 0017 section 1: one crate named `thinkthen` holds core, engine, and command, and nothing named `thinkthen-core` or `thinkthen-cli` is ever published. Section 8 step 2: the engine exports no C symbol, and the C binding owns every exported name. Section 9: the C surface ships a shared library, a static library, and a header.
- ADR 0037, Ian on 2026-09-22: "Whatever you need, let's support everything through C." C, C++, Go, and Java use one C door.
- Ticket 0084: no second crate, C symbol, or connector is public in the `thinkthen` package.
- `thinkthen` has `#![forbid(unsafe_code)]` in `lib.rs`, and the root lint table pins `unsafe_code = "forbid"`. Every C door and host FFI needs `unsafe`.

## Decision

1. Each surface binding is its own Cargo workspace with its own `Cargo.lock`, under `libraries/<host>` or `databases/<host>`, as the branch did. It sets `publish = false` and depends on `crates/thinkthen` by path with default features off. It uses only the public API, which the compiler enforces. No binding depends on another binding. Host toolchains (pyo3, napi, pgrx, DuckDB, SQLite, R, Ruby) therefore never enter the root lock, `policy.py`'s dependency tables, or a root gate. R's offline tarball gets a standalone crate.
   - Cargo ignores a path dependency's workspace profile. Each binding workspace therefore copies the root `[profile.release]` (`overflow-checks = true`, `panic = "unwind"`), so every surface ships the engine the root gates tested. `policy.py` checks the copy.
   - A separate lock can resolve `thinkthen`'s normal dependencies to other versions. A `policy.py` check run from `lint` requires each binding's `Cargo.lock` to resolve `thinkthen`'s own normal dependency tree (ureq among them) to the root lock's versions. It compares only the versions `thinkthen` resolves to, because a lock may also hold a second major version for a host crate. If a host crate's exact pin forces a conflict or a root lock bump, the surface ticket records that case.
2. The root workspace excludes `libraries`, `databases`, and 0086's test-only `conformance/consumer` workspace, and sets `default-members = ["crates/thinkthen"]`. A plain root `cargo build` or `cargo test` compiles `thinkthen` alone. `--workspace` adds only the test-only member `conformance/backend` (ticket 0092). A test-only, unpublished root member is allowed under policy, deny, and the ratchet.
3. Lints. `thinkthen` keeps `#![forbid(unsafe_code)]` in `lib.rs`, and the root lint table does not change. Each binding's lint table equals the root table except `unsafe_code = "deny"`, and `policy.py` checks that. Only its FFI module carries `#[allow(unsafe_code, reason = "…")]`. `policy.py` gains one accepted binding lint table and one planted failure: `unsafe` outside the FFI module fails.
4. The ratchet. The shared reader `sdlc/scripts/ratchet.mjs` finds its config from its own location. It gains one optional argument, a config path, and resolves that config's `directory` from the config's own folder. With no argument it reads `sdlc/ratchet.json` as before. There is one reader and no copy. Nine other repositories carry the same reader. Their copies keep working unchanged, because the no-argument call is the same, and each may take the argument later. Each binding keeps its own `ratchet.json` beside its `Cargo.toml`, and `lint` runs the reader once per binding in the surface registry. That file counts the binding's Rust. Host-language code (`.py`, `.ts`, `.R`, `.rb`, SQL) is counted too: the surface ticket adds one `ratchet.<ext>.json` per host language in the same folder, because a config names one extension. Code under `libraries/` and `databases/` never escapes a ceiling, and the root ceiling keeps counting `crates` only.
5. **Settled 2026-09-24: one width cap per loaded copy for 0.1.** Each native package links its own copy of the engine. ADR 0017 section 2, which Ian accepted, says "One width gate for the process". Two copies in one process, such as a Python wheel and a DuckDB extension, each carry their own engine and their own cap. "One width per process" therefore holds per loaded copy only. The owner accepts that for 0.1 and documents it. A shared cap across copies comes later only if a user needs it. The filed question is `notes/todos/2026-09-24-thinkthen-width-cap-with-two-library-copies.md`. Ian can overturn this item.
   - The public name stays `width`. ADR 0017 section 5, ticket 0084, and every surface ticket use it.
   - Each surface page describes width in plain words: the limit on requests in flight at once, per loaded copy. A host that loads two copies can run up to twice the width.
   - Each surface README states that the cap is per loaded copy. `sdlc/planning/surfaces-port-guide.md` carries this as a builder note.
   - Options not taken for 0.1: refuse a second copy, which needs a process-global marker and gives a user who loads both packages an error; or share one cap through a process-global primitive such as a named semaphore, which needs new platform code outside Rust statics and its own fork story. The second option stays open for later.
   - Counters and fork state are per copy under every option. Durable cache coordination already crosses copies through operating-system file locks.
6. The C door is the crate `thinkthen-c` at `libraries/c`, library name `thinkthen_c`, crate types `cdylib` and `staticlib` only. The distinct name avoids a second `libthinkthen.rlib` beside its dependency. Its build sets the shared library's soname to `libthinkthen.so.0` (install name on macOS). The release step names the files `libthinkthen.so` and `libthinkthen.a`. It owns every exported symbol and the header. It never goes to crates.io. It ships in release archives, as ADR 0017 section 9 says.
7. No shared binding-helper crate. Host-neutral pieces live in the public API (ticket 0095: host deadline numbers, `ErrorKind::name`, result JSON). Each binding keeps one panic guard and one error-kind table at its own edge (error-index rows R1-31 and R2-31). Ticket 0094 is the reference for the FFI edge: the guard, the error table, and the one `unsafe` module.
8. The Polars surface at 0.1 is the Python Polars door, per the 2026-09-21 Polars ruling in ADR 0017. The branch's Rust Polars `Series` door is deferred past 0.1. A Rust user maps a `Series` of text into `decide_many`.
9. Relate from rows (all three databases): the binding dedupes rows by name and kind in first-seen order and calls `relate` once. It maps each edge back to every row with that pair. The engine's 255 cap counts unique pairs.

## Why this is consistent with ADR 0017 and ADR 0037

ADR 0017 bans publishing a split of the engine's layers. A binding crate adds no layer and holds no engine logic, and it is never published to crates.io, so the one Rust package stays `thinkthen`. ADR 0017 section 8 step 2 already puts every exported C name in a C binding outside the engine, and 0084 keeps C symbols out of `thinkthen`. ADR 0037 asks for everything through C. A Cargo feature on `thinkthen` cannot add a `cdylib` crate type. It would also put `unsafe` and exported symbols into the published package. So the C door is a separate crate.

## Every surface ticket

Each surface ticket follows ticket 0093's pattern and changes only host code. `libraries/rust` is the reference copy of every file below.

1. Folder. The binding lives at the folder its line in `sdlc/surfaces.txt` names. Change that line from `planned` to `landed` in the same commit. `lint` refuses a binding folder that is not a landed line.
2. `Cargo.toml`. Give it an empty `[workspace]` table, so the binding is its own workspace. Set `publish = false`, and use the root `edition` and `rust-version`. Depend on `thinkthen = { path = "../../crates/thinkthen", default-features = false }` once, in `[dependencies]`. Depend on no other binding. Copy the root `[profile.release]`. Copy the root lint tables into `[lints.rust]` and `[lints.clippy]`, with `unsafe_code = "deny"`. Tests may start the loopback backend through a dev-dependency on `conformance/backend`.
3. `clippy.toml`. Copy `libraries/rust/clippy.toml`, which holds the shared thresholds and test allowances.
4. `Cargo.lock`. Start from a copy of the root lock, then let Cargo prune it offline. `thinkthen`'s own tree must resolve to the root lock's versions.
5. `unsafe`. Only a file named `ffi.rs` holds it, under `#[allow(unsafe_code, reason = "…")]`.
6. Tests. A bare `cargo test` runs every Rust test. No test is `#[ignore]`d, and none returns before its first assertion (R2-28). The check reads `#[test]` functions only. It misses `cfg_attr(…, ignore)`, a skip branch that never returns, and other test macros. A host test framework tightens it when it arrives.
7. `ratchet.json`. Name the source folders under `directory`, never the folder itself, so `target` stays uncounted. Set `max` to the measured count. Add one `ratchet.<ext>.json` per host language beside it.
8. `check.sh`. The surface rung passes the loopback port as `$1`. The check runs offline and exits 0 on a pass. It exits 77 when its host toolchain is missing, and the rung reports "not run", never "pass" (R6-2). A Rust binding's check runs `cargo fmt --check`, Clippy with warnings denied, and `cargo test`.
9. The rung. `sdlc/scripts/surfaces` is the fifth ladder step, after `spec`. It runs every landed surface's `check.sh`, including heavy Docker checks, and never runs from `test`. The CI workflow runs the first four rungs only, and `lint` there still runs the registry, ratchet, and deny checks. `lint` runs `surfaces --registry`. That check refuses a landed surface with no `check.sh`, runs every `ratchet*.json` file of each landed surface, and runs `cargo deny` over each landed lock.
10. `policy.py` checks items 2 through 6 for every `libraries/*/Cargo.toml` and `databases/*/Cargo.toml`, registered or not.
11. `README.md`. State that the width cap holds per loaded copy (item 5).

The host work follows the same list for every surface.

- Map types per section 3.1 of `sdlc/planning/surfaces-port-guide.md`. A binding may use `CallOptions::interrupt` for its host's interrupt channel, or keep the worker-thread pattern. Use `deadline_seconds` or `deadline_millis` for host numbers, and one-question `annotate` for bulk `choose`, `score`, and `tag`.
- Run the shared cases through the 0092 backend. Re-run the surface's error-index rows against the real engine, each with a planted bug that turns its test red. Report a skipped case as not run.
- Write any ruling that lived only in notes as an ADR (R2-29).

## PostgreSQL (ticket 0111, 2026-09-25)

The PostgreSQL binding follows the list above with these recorded differences. Ian can overturn each one.

- Lints. The lint tables equal the root's except `unsafe_code = "deny"` and `unexpected_cfgs` at `deny` with the root's one `check-cfg` name. `pgrx::pg_module_magic!()` expands its own `allow(unexpected_cfgs)`, and a forbid refuses it. `src/bin/pgrx_embed.rs` carries `#![allow(unsafe_code, reason = "…")]`, because `cargo pgrx package` injects generated `unsafe extern` blocks into it. Every other `unsafe` line lives in `src/ffi.rs` (item 5).
- Settings (decision 3). `thinkthen.throttle`, `max_requests`, `cache`, and `cache_bytes` are `Suset` settings, -1 or empty for unset. An unset setting calls no setter, so `EngineBuilder::from_env` keeps the environment's and the configuration file's values. A changed plan rebuilds the engine. The throttle holds for the whole process under 0077, so a rebuild passes it only while no throttle is active. `cache_bytes = 0` refuses with 22023.
- Saved answers (decision 4). The tag's in-backend answer map retires. `thinkthen_warm` fills the engine's disk cache, and a later decide reads it.
- Relate (decision 10). `thinkthen_relate` reads its query's rows through SPI and refuses the 256th row, a stricter cap than the unique-pair cap.
- The key (decision 12). `thinkthen.api_key` is a `Userset` setting that is never read. A set value refuses the next call with 22023, and the value never reaches the log.
- Deny. `databases/postgresql/deny.toml` is the root file plus an `ignore` for RUSTSEC-2021-0127 (`serde_cbor` under pgrx) and a Zlib exception for `foldhash`.
- Counters. Each engine counts its own sends, so `thinkthen_usage()` adds the counters of every engine the backend built.

## Consequences

- 0077's duplicate-image boundary follows item 5. Each loaded copy has its own cap, and each surface page says so.
- The ADR 0037 header moves from `contract/include/thinkthen.h` to `libraries/c/include/thinkthen.h` in ticket 0094.
- Ian can overturn every item. Items 5 and 8 touch his rulings most closely.
