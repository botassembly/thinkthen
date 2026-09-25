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
5. **Settled 2026-09-24: one throttle per loaded copy for 0.1.** Each native package links its own copy of the engine. ADR 0017 section 2, which Ian accepted, says "One width gate for the process". Two copies in one process, such as a Python wheel and a DuckDB extension, each carry their own engine and their own throttle. "One width gate for the process" therefore holds per loaded copy only. The owner accepts that for 0.1 and documents it. A shared cap across copies comes later only if a user needs it. The filed question is `notes/todos/2026-09-24-thinkthen-width-cap-with-two-library-copies.md`. Ian can overturn this item.
   - Amended 2026-09-25: the public name is `throttle`, under Ian's 2026-09-24 ruling and ADR 0017's amendment. The command keeps `--jobs`.
   - Each surface page describes the throttle in plain words: the limit on requests in flight at once, per loaded copy. A host that loads two copies can run up to twice the throttle.
   - Each surface README states that the cap is per loaded copy. `sdlc/planning/surfaces-port-guide.md` carries this as a builder note.
   - Options not taken for 0.1: refuse a second copy, which needs a process-global marker and gives a user who loads both packages an error; or share one cap through a process-global primitive such as a named semaphore, which needs new platform code outside Rust statics and its own fork story. The second option stays open for later.
   - Counters and fork state are per copy under every option. Durable cache coordination already crosses copies through operating-system file locks.
6. The C door is the crate `thinkthen-c` at `libraries/c`, library name `thinkthen_c`, crate types `cdylib` and `staticlib` only. The distinct name avoids a second `libthinkthen.rlib` beside its dependency. Its build sets the shared library's soname to `libthinkthen.so.0` (install name on macOS). The release step names the files `libthinkthen.so` and `libthinkthen.a`. It owns every exported symbol and the header. It never goes to crates.io. It ships in release archives, as ADR 0017 section 9 says.
7. No shared binding-helper crate. Host-neutral pieces live in the public API (ticket 0095: host deadline numbers, `ErrorKind::name`, result JSON). Each binding keeps one panic guard and one error-kind table at its own edge (error-index rows R1-31 and R2-31). Ticket 0094 is the reference for the FFI edge: the guard, the error table, and the one `unsafe` module.
8. The Polars surface at 0.1 is the Python Polars door, per the 2026-09-21 Polars ruling in ADR 0017. The branch's Rust Polars `Series` door is deferred past 0.1. A Rust user maps a `Series` of text into `decide_many`.
9. Relate from rows (all three databases): the binding dedupes rows by name and kind in first-seen order and calls `relate` once. It maps each edge back to every row with that pair. The engine's 255 cap counts unique pairs.

## Amended 2026-09-25: the Rust Polars door and the Polars column table

Ian ruled on 2026-09-24 that Rust Polars and Python Polars are both in 0.1 (`sdlc/planning/one-line-plan-2026-09-24.md`, afternoon rulings). That ruling overturns item 8's deferral of the Rust `Series` door. Ticket 0120 builds it as `thinkthen-polars` at `libraries/polars`, a binding like the others under items 1 to 7. Its `deny.toml` is the root file plus four named license exceptions, and `policy.py` checks that difference.

10. **The Polars column table.** Both Polars doors, 0120 in Rust and 0106 in Python, write a frame's new columns by this table. The issue `sdlc/issues/2026-09-25-public-library-api-gaps.md` holds its history.

| Question | Column | Widened cell when any row of that question failed |
| --- | --- | --- |
| decide | `Boolean`, null for not sure | `true` or `false` from `value_json`, null for not sure |
| choose | `String`, null for nothing fits | the plain label, null for nothing fits |
| score | `Float64` | the number text from `value_json` |
| tag | `String` holding the JSON array text | the array text from `value_json` |
| failed | | the marker from `value_json`, such as `{"failed":{"kind":"backend","cause":"missing_probability"}}` |

A column widens only when the same reply also holds a usable answer. The engine refuses a reply with no usable answer (`specification/annotate.md`), so a one-member set, a series call, or a request chunk whose only answer failed ends the call with a `Backend` error. A widened cell takes its text unchanged from `AnnotatedRecord::value_json`, the engine's one serializer. No door keeps its own table of failure causes. Ian can overturn the table.

## Why this is consistent with ADR 0017 and ADR 0037

ADR 0017 bans publishing a split of the engine's layers. A binding crate adds no layer and holds no engine logic, and it is never published to crates.io, so the one Rust package stays `thinkthen`. ADR 0017 section 8 step 2 already puts every exported C name in a C binding outside the engine, and 0084 keeps C symbols out of `thinkthen`. ADR 0037 asks for everything through C. A Cargo feature on `thinkthen` cannot add a `cdylib` crate type. It would also put `unsafe` and exported symbols into the published package. So the C door is a separate crate.

## Every surface ticket

Each surface ticket follows ticket 0093's pattern and changes only host code. `libraries/rust` is the reference copy of every file below.

1. Folder. The binding lives at the folder its line in `sdlc/surfaces.txt` names. Change that line from `planned` to `landed` in the same commit. `lint` refuses a binding folder that is not a landed line. A crate that cannot sit at the binding folder, such as R's inside its package, gets an optional third field naming its crate folder relative to the binding folder. The field may not start with `/` or hold a `..` part. The registry and `policy.py` read the manifest there. The binding's `deny.toml` and ratchet files stay at the binding folder.
2. `Cargo.toml`. Give it an empty `[workspace]` table, so the binding is its own workspace. Set `publish = false`, and use the root `edition` and `rust-version`. Depend on `thinkthen = { path = "../../crates/thinkthen", default-features = false }` once, in `[dependencies]`. Depend on no other binding. Copy the root `[profile.release]`. Copy the root lint tables into `[lints.rust]` and `[lints.clippy]`, with `unsafe_code = "deny"`. Tests may start the loopback backend through a dev-dependency on `conformance/backend`.
3. `clippy.toml`. Copy `libraries/rust/clippy.toml`, which holds the shared thresholds and test allowances.
4. `Cargo.lock`. Start from a copy of the root lock, then let Cargo prune it offline. `thinkthen`'s own tree must resolve to the root lock's versions.
5. `unsafe`. Only a file named `ffi.rs` holds it, under `#[allow(unsafe_code, reason = "…")]`.
6. Tests. A bare `cargo test` runs every Rust test. No test is `#[ignore]`d, and none returns before its first assertion (R2-28). The check reads `#[test]` functions only. It misses `cfg_attr(…, ignore)`, a skip branch that never returns, and other test macros. A host test framework tightens it when it arrives.
7. `ratchet.json`. Name the source folders under `directory`, never the folder itself, so `target` stays uncounted. Set `max` to the measured count. Add one `ratchet.<ext>.json` per host language beside it.
8. `check.sh`. The surface rung passes the loopback port as `$1`. The check runs offline and exits 0 on a pass. It exits 77 when its host toolchain is missing, and the rung reports "not run", never "pass" (R6-2). A Rust binding's check runs `cargo fmt --check`, Clippy with warnings denied, and `cargo test`.
9. The rung. `sdlc/scripts/surfaces` is the fifth ladder step, after `spec`. It runs every landed surface's `check.sh`, including heavy Docker checks, and never runs from `test`. The CI workflow runs the first four rungs only, and `lint` there still runs the registry, ratchet, and deny checks. `lint` runs `surfaces --registry`. That check refuses a landed surface with no `check.sh`, runs every `ratchet*.json` file of each landed surface, and runs `cargo deny` over each landed lock with `advisories bans licenses sources`. A binding's own `deny.toml` equals the root file plus the entries `policy.py` names for it.
10. `policy.py` checks items 2 through 6 for every `libraries/*/Cargo.toml` and `databases/*/Cargo.toml`, registered or not.
11. `README.md`. State that the throttle holds per loaded copy (item 5).

The host work follows the same list for every surface.

- Map types per section 3.1 of `sdlc/planning/surfaces-port-guide.md`. A binding may use `CallOptions::interrupt` for its host's interrupt channel, or keep the worker-thread pattern. Use `deadline_seconds` or `deadline_millis` for host numbers, and one-question `annotate` for bulk `choose`, `score`, and `tag`.
- Run the shared cases through the 0092 backend. Re-run the surface's error-index rows against the real engine, each with a planted bug that turns its test red. Report a skipped case as not run.
- Write any ruling that lived only in notes as an ADR (R2-29).

## The TypeScript surface (ticket 0107)

These rulings lived only in the tag's notes (R2-29). Ian can overturn each one.

- One error class, `ThinkThenError`, carries `kind` and `retryable`.
- An `AbortSignal` is the cancel gesture. The promise rejects at once, and the binding fires the call's `CancelToken`.
- `deadlineMs` counts milliseconds and converts through `CallOptions::deadline_millis`.
- A question value is a frozen function that carries its spec.
- `new tt.Engine(options)` exposes ADR 0017 section 5's settings. The first explicit throttle holds for the process.
- Each call runs on a detachable worker thread of its own. The tag's calls on the libuv pool retire. `detach()` fires the token and closes the threadsafe function, so Node may exit after any settle.
- The napi module `src/node.rs` holds the macro output under one `allow(unsafe_code)`. No hand-written `unsafe` exists, so item 5's `ffi.rs` rule holds.
- The package ships its own `index.js`, `index.mjs`, `index.d.ts`, and a nine-line `loader.js`. No generated loader remains.

## The SQLite binding (ticket 0109)

These rulings lived only in the tag's `NOTES.md` (R2-29). Ian can overturn each.

- The floor is SQLite 3.50.0. Below it a CHECK constraint in an untrusted database reaches a volatile function, so a schema could spend money or read files. The load refuses below the floor and names the host's version.
- Every function and both table-valued modules are direct-only, and none is deterministic. A view, trigger, DEFAULT, CHECK constraint, generated column, or index in a database file cannot call them, whatever `trusted_schema` says.
- `'@name'` resolves against the process working directory. The extension follows symlinks and confines nothing. The file must be a regular file of at most 1 MiB, opened once without blocking.
- `thinkthen_warm` takes decide questions only.
- `thinkthen_max_requests(n)` caps each engine call, not a statement, because each scalar row is a one-record call. Ian ruled on 2026-09-25 (`sdlc/planning/one-line-plan-2026-09-25.md`) for a process total, `thinkthen_max_requests_total(n)`, unset by default. Before each call the extension reads the requests its engine has sent. A spent total refuses as `usage` with no send. Otherwise a warm flush is cut to the remaining total, judged, and then refused. It holds to within one call's retries for the scalars and `thinkthen_warm`. A `thinkthen_recognize` or `thinkthen_relate` call counts as one record but may send several requests, so it can pass the total by that call's own requests as well. Calls running at the same time can each spend what remains, so the total can be exceeded by one call per thread in flight, plus retries. A forked child starts from zero.
- `thinkthen_relate` reads entities from a named table's id, name, and kind columns (item 9).
- Every call that can send runs on a detachable worker. On an interrupt the calling thread returns at once, and the detached worker finishes the requests it already sent. That worker is the known exception to ADR 0017's rule that no thread outlives a call. It lives at most the 30-second request timeout and holds its throttle permits until then.
- `rusqlite` brings `foldhash` 0.2.0 under the Zlib license. The root `deny.toml` does not allow Zlib and cannot, because its tree never uses it. `databases/sqlite/deny.toml` equals the root file plus that one exception. Zlib is permissive and has no copyleft term.

## PostgreSQL (ticket 0111, 2026-09-25)

The PostgreSQL binding follows the list above with these recorded differences. Ian can overturn each one.

- Lints. The lint tables equal the root's except `unsafe_code = "deny"` and `unexpected_cfgs` at `deny` with the root's one `check-cfg` name. `pgrx::pg_module_magic!()` expands its own `allow(unexpected_cfgs)`, and a forbid refuses it. `src/bin/pgrx_embed.rs` carries `#![allow(unsafe_code, reason = "…")]`, because `cargo pgrx package` injects generated `unsafe extern` blocks into it. Every other `unsafe` line lives in `src/ffi.rs` (item 5).
- Settings (decision 3). `thinkthen.throttle`, `max_requests`, `cache`, and `cache_bytes` are `Suset` settings, -1 or empty for unset. An unset setting calls no setter, so `EngineBuilder::from_env` keeps the environment's and the configuration file's values. A changed plan rebuilds the engine. The throttle holds for the whole process under 0077, so a rebuild passes it only while no throttle is active. `cache_bytes = 0` refuses with 22023.
- Saved answers (decision 4). The tag's in-backend answer map retires. `thinkthen_warm` fills the engine's disk cache, and a later decide reads it.
- Relate (decision 10). `thinkthen_relate` reads its query's rows through SPI and refuses the 256th row, a stricter cap than the unique-pair cap.
- The key (decision 12). `thinkthen.api_key` is a `Userset` setting that is never read. A set value refuses the next call with 22023, and the value never reaches the log.
- Deny. `databases/postgresql/deny.toml` is the root file plus an `ignore` for RUSTSEC-2021-0127 (`serde_cbor` under pgrx) and a Zlib exception for `foldhash`.
- Counters. Each engine counts its own sends, so `thinkthen_usage()` adds the counters of every engine the backend built. The binding keeps one engine per settings plan and records it right after the build, so a cancelled call's sends still count.
- The request total. Ian ruled on 2026-09-25 that every SQL surface caps requests per process (`sdlc/planning/one-line-plan-2026-09-25.md`). `thinkthen.max_requests_total` is a `Suset` setting, -1 for unset. Each call reads what remains once. None left refuses with 22023 and no send, and a batch sends only what remains before it refuses. A new backend starts from zero.
- Recognize. `thinkthen_recognize` takes a kinds array or a version-one spec, `'@names.json'`. The spec form carries the kinds' meanings and thresholds that the shared cases need.

## Consequences

- 0077's duplicate-image boundary follows item 5. Each loaded copy has its own cap, and each surface page says so.
- The ADR 0037 header moves from `contract/include/thinkthen.h` to `libraries/c/include/thinkthen.h` in ticket 0094.
- Ian can overturn every item. Items 5 and 8 touch his rulings most closely.

## Ruby rulings (ticket 0112, R2-29)

These rulings lived only in the tag's Ruby `NOTES.md`. Ian can overturn each one.

- The crossing never takes the VM lock beneath the engine call. Each call runs on its own worker thread with asynchronous signals blocked. The Ruby thread waits in 50 ms slices with the lock released, through `rb_thread_call_without_gvl` and an unblock function that only wakes the wait. Between slices, with the lock held, it reads pending interrupts under `rb_protect`, then the caller's token and the call's own.
- The return is prompt. Any stop fires the call's own token and returns within one slice. The worker finishes its sent requests alone, and its answer is dropped.
- Ctrl-C raises `ThinkThen::CancelledError`, with the `Interrupt` as its cause. Every other raise, such as `Thread#raise` or a trap's error, passes through unchanged.
- The detach cost of decision 5: a stopped call's worker lives until its sent requests end. A stopped batch can hold up to the throttle in flight after the raise. It never sends a new request, because its own token is fired.
- The watchdog runs ticks every 0.1 s on one Ruby thread. A tick belongs to the thread that set it. A tick that raises fires its call's own token, and the call raises the tick's error.
- A `nil` record refuses with `UsageError` naming its index. A record that is not a `String` crosses as its JSON text.

## R section, 2026-09-25 (ticket 0108)

These rulings lived only in the R surface's notes (R2-29). Ian can overturn each.

- **Layout.** R's crate sits at `libraries/r/thinkthen/src/rust`, inside the R package, because `R CMD INSTALL` builds from the package folder. Its lock and `clippy.toml` sit there too. `libraries/r/deny.toml` and the two ratchet files sit at `libraries/r`. R's line in `sdlc/surfaces.txt` names the crate folder `thinkthen/src/rust` as a third field, and the registry check and `policy.py` read it (`sdlc/records/surface-batch-integration.md`).
- **`I()` over a number.** A deadline may carry the `AsIs` class. Any other class, such as `factor` or `difftime`, is `usage`.
- **Deadline numbers.** `NULL`, `-1`, and `-1L` mean no deadline. The engine's `deadline_seconds` rules every other number.
- **Bulk choose, score, and tag.** A column crosses as one `annotate` of a one-question set. This closes R2-23.
- **jsonlite stays** as the one import, at the tested 2.0.0. Its archive is pinned by sha256.
- **relate takes a frame** with `name` and `kind` columns and dedupes it by item 9.
- **The ratchets.** `ratchet.json` counts the crate's Rust. `ratchet.R.json` counts `.R` files under the package's `R` and `tools` folders, `tests`, and `examples`.
- **The `paste` advisory.** `extendr-api` 0.8.2 depends on `paste`, which RUSTSEC-2024-0436 reports unmaintained. The binding's own `deny.toml` equals the root one plus that one ignore entry with its reason.

## Python (ticket 0105)

These rulings lived only in the tag's Python notes (R2-29). Ticket 0105 records them here. Ian can overturn each one.

- `Cancelled` subclasses both `KeyboardInterrupt` and `ThinkThenError`. Code that catches Ctrl-C and code that catches every thinkthen failure both see a stop.
- Only a `KeyboardInterrupt` from a signal handler becomes `Cancelled`. Any other error a handler raises, such as `SystemExit`, passes through unchanged.
- A pandas object is refused with a `UsageError` that says the Python data frame is Polars (the 2026-09-21 ruling in ADR 0017). The check reads the type's top-level module before any Arrow check, because pandas objects also expose `__arrow_c_stream__`.
- Every call runs on a detachable worker thread, so Ctrl-C and a caller's token stop a single send and a batch within one 50 ms tick. A detached send ends on its own, and no new request starts.
- The Rust unit tests link libpython, so they run as `cargo test --no-default-features --lib` from `check.sh`, which sets the library path. A bare `cargo test` builds the extension-module form, which does not link libpython (item 6).

## Python Polars door (ticket 0106)

Ticket 0106 records these rulings. Ian can overturn each one.

- Item 3 names one FFI module. `policy.py` admits `unsafe` only in files named `ffi.rs`, and one file would exceed the ticket's 500-line cap. So the Python door keeps its raw memory code in three `ffi.rs` files under `libraries/python/src/arrow/`: one reads a producer, one hands answers back, and one holds the probe build's test producer. Each carries its own `#[allow(unsafe_code, reason = "…")]`.
- The worker releases a producer's batches attached to the interpreter, behind an `atexit` exit gate. A release that finds exit under way leaks the batches on purpose (spike 255). A release never waits on the gate: it tries the read side and leaks when the exit hook holds or waits for the write side, so a release on a thread that holds the interpreter cannot deadlock the hook.
- Waived at 0.1 (R2-29): an extent that runs into another readable allocation, and a guard page off Linux, where `mincore` sees only unmapped pages. Memory a producer unmaps during a call stays a recorded risk. The lever for each is a producer that lies about its buffers.
