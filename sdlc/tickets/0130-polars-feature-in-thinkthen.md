---
flow: build
priority: 130
opens: crates/thinkthen/Cargo.toml crates/thinkthen/src/public/mod.rs crates/thinkthen/src/public/frame.rs crates/thinkthen/src/public/frame crates/thinkthen/tests/polars Cargo.lock deny.toml libraries/polars databases/sqlite/deny.toml sdlc/surfaces.txt sdlc/scripts/surfaces sdlc/scripts/policy.py sdlc/scripts/test sdlc/scripts/lint sdlc/ratchet.json sdlc/planning/adr/0047-bindings-are-unpublished-crates-over-the-public-api.md sdlc/planning/adr/0017-libraries-over-one-bound-core.md sdlc/tickets/0084-freeze-the-public-rust-contract.md sdlc/planning/libraries/rust.md sdlc/issues/2026-09-25-release-and-install-for-0-1.md sdlc/issues/2026-09-25-public-library-api-gaps.md sdlc/records sdlc/tickets
---

# 0130: Move the Rust Polars door into `thinkthen` behind a `polars` feature

Status: in progress. Design accepted 2026-09-25 by the coordinator after its review, with the text edits below. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. The change adds a dependency to `thinkthen` and widens its public surface under a feature, so the code review names what it checked (repo `CLAUDE.md`).

## Outcome and authority

A Rust user who wants Polars writes `thinkthen = { version = "0.1", features = ["polars"] }` and gets the Series and frame door from the one `thinkthen` crate. By default `thinkthen` compiles no Polars, and a user who does not ask for the feature sees the same public API as today.

The root lock does name the Polars tree, so every machine that builds this repository keeps the Polars crates in its offline cache. The `install` rung fetches them with its `cargo fetch --locked`. The `lint` rung's root deny run and `policy.py`'s `cargo metadata` read them offline and fail without them. The spike's lock holds 90 packages the root lock does not, 8.1 MB of `.crate` files in cargo's cache.

Ian ruled on 2026-09-25: "No separate Rust Polars crate. The Rust Polars door moves into `thinkthen` behind an optional `polars` feature. This amends ADR 0047." The ruling is item 4 of "Ian's rulings, 2026-09-25" in `sdlc/planning/issue-backlog-2026-09-25.md`, and the release issue `sdlc/issues/2026-09-25-release-and-install-for-0-1.md` records it with his reason: Python ships `thinkthen[polars]`, and Rust should match. Ticket 0128 (release) left this move to its own ticket in its decision 15. This is that ticket.

Today the door is the crate `thinkthen-polars` at `libraries/polars` (ticket 0120), its own workspace with its own lock, `deny.toml`, and ratchet. It never shipped (`publish = false`).

## Is the Python Polars fix (backlog item A1) part of this ticket?

No. It gets its own ticket. Backlog item A1 is item 8 of `sdlc/issues/2026-09-25-public-library-api-gaps.md`: Python Polars writes widened cells with its own code, so a widened score of 1.0 prints as `1`. The reasons it stays out:

- It shares no file with this move. A1 changes `libraries/python/src/arrow/write.rs`, `libraries/python/src/engine.rs`, `libraries/python/src/frame.rs`, and a Python test. This ticket changes no Python file.
- It runs in another lane. A1 is proved by the Python surface's `check.sh`. This ticket is proved by the Rust Polars lane and the root gates.
- It is a live bug at the top of the backlog. It should not wait for a dependency review of the root lock and a new license exception list.
- This ticket keeps the Rust door's column text byte for byte. ADR 0047 item 10, the table both doors follow, does not change. So A1 can build beside this ticket, and its test can pin the table's text without building the Rust door.

The issue `sdlc/issues/2026-09-25-a-polars-frame-sends-before-a-question-name-clash-fails.md` is also Python Polars, in the same `frame.rs`. It fits A1's ticket better than this one. That grouping is the coordinator's call.

## Design

The door's code and tests move into `crates/thinkthen`. Its behavior does not change. The tests that proved it move with it and run in one lane.

1. **The feature.** `crates/thinkthen/Cargo.toml` gains `polars = { version = "0.55.2", default-features = false, optional = true }` and the feature `polars = ["dep:polars"]`. `default` stays `["cli"]`. The lock keeps 0.55.2, and this ticket moves no version.
   - The requirement becomes a caret, `0.55.2`, which Cargo reads as at least 0.55.2 and below 0.56. 0120 pinned `=0.55.2`. Its record gives two reasons: one door release supports one Polars minor, and `policy.py` held the `polars` and `polars-core` pins equal (`sdlc/records/0120-build-rust-polars.md` line 13, 0120 amendment item 3). The caret keeps the first, because Polars breaks its API only across minors. The second retires with the `polars-core` dev-dependency (decision 6). Neither spike 257 nor the design review recorded another reason. A published crate with an exact pin would refuse any user whose tree needs 0.55.3, so the caret is the right requirement once the crate ships.
2. **The code.** `libraries/polars/src/lib.rs` and `column.rs` move to `crates/thinkthen/src/public/frame.rs` and `crates/thinkthen/src/public/frame/column.rs`, behind `#[cfg(feature = "polars")] mod frame;` in `public/mod.rs`. The module is private. The name `frame` avoids a clash with the `polars` crate path.
3. **The public names.** With the feature on, the crate root gains two names and no module of its own:
   - `thinkthen::PolarsEngine`, the trait with the five methods of 0120 decision 3, implemented for `Engine`.
   - `thinkthen::polars`, a re-export of the `polars` crate. A user names the exact Polars version the door takes through it, as `thinkthen_polars::polars` did.
   With the feature off, neither name exists. `thinkthen_polars::thinkthen` goes, because the door now sits in `thinkthen`.
4. **One error type.** The door's own `Error { Engine, Usage, Defect }` retires. It existed only because 0084 gives an outside crate no way to build a `thinkthen::Error` (0120 decision 3). Inside the crate, each method returns `thinkthen::Error`. A refusal is `Error::usage(sentence)` and a fault is `Error::defect(message)`, through the constructors that already exist in `public/error.rs`. This ticket does not edit `public/error.rs`. Each refusal sentence stays the same text. A defect message gains the crate's usual `defect: ` prefix.
5. **The tests.** `libraries/polars/tests/*.rs` move to `crates/thinkthen/tests/polars/`, with four `[[test]]` tables named `polars_door`, `polars_cases`, `polars_deadline`, and `polars_throttle`, each with `required-features = ["polars"]`. Cargo skips them when the feature is off. The two throttle tests keep one test per file, so each runs in its own process (0120, "One process for each throttle test"). The helper `tests/polars/common/mod.rs` keeps its guard: it refuses to build an engine unless the environment holds the lane's fake key and a loopback address. Paths to `conformance/cases.json` read from `CARGO_MANIFEST_DIR`.
6. **The `polars-core` dev-dependency goes.** Cargo has no optional dev-dependency. A `polars-core` dev-dependency with `dtype-categorical` and `dtype-struct` would compile most of Polars in every root test build. Only one test needed it: the R1-4 test, which built `Categorical`, `Enum`, and struct columns (spike 257 finding 2). That test now builds `Int64`, `Boolean`, and `List(String)` columns beside the text column. Its plant becomes "cast every caller column that is not text to `String`". The refusal test's `Categorical` row becomes a `Boolean` row. Its `Int64` row stays. Both reach the same code path: the door refuses any dtype but `String` in one check, and it never touches a column other than `on`. The equal-pin check in `policy.py` retires with the dev-dependency.
7. **One lane builds Polars.** The `test` and `lint` rungs drop `--all-features`. Today `--all-features` equals the default set, because `cli` is the only feature. After this ticket it would add Polars to every test build and every Clippy run. Only the Rust Polars lane compiles Polars:
   - `libraries/polars/check.sh` stays in `libraries/polars` as the lane. It runs from the repository root, where the manifest now is, and uses its own target folder, `target/polars`, so its builds never evict the other rungs' builds.
   - It keeps 0120's steps in order. The probe `cargo fetch --locked --offline` exits 77 with the same two lines when the cache lacks a locked crate. The environment step unsets `THINKTHEN_API_KEY`, sets the fake key, a closed loopback port, and scratch cache folders.
   - Then Clippy with warnings denied over the library, the binaries, and `--test 'polars_*'`, with the default features plus `polars`. That is the shape a user who adds the feature builds.
   - Then one Clippy run over the library alone with `--no-default-features --features polars`, the library-only user's shape. It reuses the dependency builds of the first run.
   - Then `cargo test --test 'polars_*'` and `cargo test --doc` filtered to `PolarsEngine`, both with the default features plus `polars`. The whole lane sets `RUSTFLAGS` and `RUSTDOCFLAGS` to `--cfg thinkthen_internal_doctest`, as the `test` rung does for doctests, so one flag set serves every step and nothing rebuilds between them.
   - `cargo fmt --check` leaves the lane. The `lint` rung's `cargo fmt --all` already reaches the moved files: rustfmt ignores `cfg`, and `--all` reads every target, `[[test]]` paths included. A plant proves it (Proof row 8).
   - A `policy.py` check refuses `--all-features` in any rung script: `install`, `lint`, `test`, `spec`, `surfaces`, and `package`.
8. **The registry gains a `feature` state.** `sdlc/surfaces.txt` changes the line `libraries/polars landed` to `libraries/polars feature`. The header comment defines it: a surface that is a Cargo feature of `thinkthen`. Its folder holds only `check.sh` and `README.md`. The root ratchet and the root `deny.toml` cover its code.
   - The `surfaces` rung runs a `feature` line's `check.sh` the same way it runs a landed one. It prints pass, not run, or FAIL, so ticket 0128's `--release` form covers it with no extra line.
   - `surfaces --registry` requires `check.sh` in a `feature` folder and refuses any other file there but `README.md`. So no code under `libraries/` escapes a ceiling (ADR 0047 item 4). It runs no ratchet and no deny for a `feature` line. It gains one plant: the line `libraries/rust feature` is refused, because that folder holds a `Cargo.toml`.
   - `policy.py`'s binding walk skips a folder whose line says `feature`. It has no `Cargo.toml` to read.
   The other choice was to delete `libraries/polars` and call a script from the `surfaces` rung. That drops the Rust Polars surface from the registry, its user page, and the per-surface report. Ten surfaces stay ten.
9. **The user page.** `libraries/polars/README.md` becomes a short page: the `Cargo.toml` line with the feature, the Polars version, the five-method table, the frame column rules, the refusal sentences, and the lane. The example moves into the rustdoc of `PolarsEngine` as a `no_run` doctest, so the lane compiles it. It sits in the crate, and a README outside the crate cannot enter `cargo package`. The four license reasons move to the root `deny.toml`.
10. **Licenses.** The root `deny.toml` reads `[graph] all-features = true`, so its run now meets the Polars tree. It gains 0120's four per-crate exceptions, each with its reason as a comment: `foldhash` (Zlib), `slotmap` (Zlib), `xxhash-rust` (BSL-1.0), and `ar_archive_writer` (Apache-2.0 WITH LLVM-exception). The `allow` list does not change, and `unused-allowed-license = "deny"` still holds. Each exception names its crate, so a new crate under the same license still fails. The header gains the sentence 0120 put in its binding file: each of these licenses lets anyone use, change, and ship the code with a notice kept, and none requires shared source.
    - `policy.py`'s `check_dependencies` reads `cargo metadata --all-features`, so its license walk sees the tree deny sees. `LICENSE_EXCEPTIONS` gains the same four entries, and `ACCEPTED_DEPENDENCIES["thinkthen"]` gains `polars`.
    - The exceptions are TOML comments above each entry, not `reason` fields, as in 0120's binding file.
    - Three lanes run deny against the root file directly: the Ruby and TypeScript surfaces, whose registry deny runs take the root `deny.toml` because they hold none of their own, and the `surfaces --registry` loop, which uses the root file for every landed binding without its own (C and the Rust examples too). Their trees never meet the four crates, so the exceptions go unused there. Stop rule 2 covers the case where cargo-deny fails on that.
    - A binding may leave out a root entry its tree never meets (`deny_failures`). So the Python, R, and PostgreSQL copies need no new entries. DuckDB's own `tools/source_checks.py` (R5-25) needs its file to equal the root text with one exception added. Its copy takes the root file whole and adds the `zlib-rs` entry as the last list item. That check and `check.sh`'s removal plant change to match. The build found this in the surfaces rung. `databases/sqlite/deny.toml` held only the `foldhash` exception, which the root now holds, so the file goes and its `BINDING_DENY` row goes. The PostgreSQL row drops `FOLDHASH` and keeps its advisory ignore. Its file does not change.
    - `libraries/polars/deny.toml`, its `BINDING_DENY` row, `BINDING_WASM_ONLY`, `check_polars_binding`, and `polars_failures` go.
11. **The manifest check.** `policy.py`'s `check_crates` holds the new table: the optional set is `clap`, `csv-core`, `signal-hook`, and `polars`. The features table is exactly `default = ["cli"]`, `cli` as today, and `polars = ["dep:polars"]`. `polars` has `optional = true` and `default-features = false`. The dev-dependency set does not change.
12. **R2-28 keeps its scan.** `policy.py`'s `binding_test_failures` (no `#[ignore]`, no return before the first assertion) runs over `crates/thinkthen/tests/polars/`, as it ran over the binding's tests.
13. **The ratchet.** `libraries/polars/ratchet.json` goes. The moved Rust counts under the root ceiling, whose `directory` already names `crates`. The root ceiling rises by the measured count of the moved files. The door loses its error type, so the repository's total Rust falls. The commit says what grew, why, and what it deleted first.
14. **The records.** ADR 0047 gains an amendment: the Rust Polars door is no longer a binding. It is the `polars` feature of `thinkthen`, and its line in `sdlc/surfaces.txt` is a `feature` line. Item 10's table does not change. The SQLite section's `deny.toml` sentence and the Polars amendment's `deny.toml` sentence point at the root file. ADR 0017 section 1 gains one sentence: a second optional feature, `polars`, off by default, carries the Rust Polars door. Ticket 0084 gains a dated two-sentence amendment: the feature adds two root names only when it is on, and the frozen inventory, built with `--no-default-features`, does not change. 0120 decision 1 had cited 0084's "no feature" as a reason against this landing zone. `sdlc/planning/libraries/rust.md` rewrites its Polars paragraph.
15. **The issues.** This ticket settles no whole issue. The landing commit adds "Settled by ticket 0130" under ruling 4 in the release issue and strikes `libraries/polars` from its item 12 list. It rewrites the Rust door paths in item 8 of the API gaps issue from `libraries/polars/src/column.rs` to `crates/thinkthen/src/public/frame/column.rs`.

## What 0128 (release) depends on

- **The crate that gets published is `thinkthen` at `crates/thinkthen`.** It carries the optional `polars` feature. No `thinkthen-polars` crate is published, and no second name is claimed. The release issue's item 4 recommendation of `thinkthen-polars` retires.
- **The version check.** `libraries/polars/Cargo.toml` and its lock leave. Nine binding manifests remain under `libraries/` and `databases/`, where 0128 item 1 counts ten. The door has no version of its own. It rides `crates/thinkthen/Cargo.toml`. The `polars = "0.55.2"` requirement is a dependency requirement, and the version check must not read it as a version place.
- **The package.** `cargo package --package thinkthen` now carries `src/public/frame*` and `tests/polars/`. The `package` script's file list already admits `src/` and `tests/`. The packaged lock carries the Polars tree.
- **docs.rs.** The docs.rs page shows the door only with `[package.metadata.docs.rs] features = ["polars"]`. That line belongs with 0128's publish metadata, item 5.
- **Not run at release.** `surfaces --release` counts the `feature` line's exit 77 as a failure, as it does for every surface.
- `cargo install thinkthen` builds the default features and never compiles Polars.
- **The requirement.** The published manifest reads `polars = "0.55.2"` as a caret with default features off (decision 1). The release job ships it as written.

0128's decision 15 and its Defers line become stale once this ticket lands. Whichever ticket lands second updates them.

## Edge cases

### Build shapes

| Build | Compiles Polars | Public names added | Who builds it |
| --- | --- | --- | --- |
| `cargo build`, default features (`cli`) | No | None | The `test` and `lint` rungs, `cargo install thinkthen` |
| `--no-default-features` | No | None | `package`, `inventory` |
| `--features polars` | Yes | `PolarsEngine`, `polars` | The Rust Polars lane, and a user who asks |
| `--no-default-features --features polars` | Yes | `PolarsEngine`, `polars` | The lane's library-only Clippy run |
| `--all-features` | Yes | The same | No rung. `policy.py` refuses it in a rung script |

### The lane

| State | What the lane prints | The rung reports |
| --- | --- | --- |
| Cargo's cache lacks a locked crate | `polars: not run; the crate cache lacks the locked crates.` and the fetch line, exit 77 | not run |
| Every test passes | Cargo's output, exit 0 | pass |
| A test fails | Cargo's failure, exit 101 | FAIL, never not run |
| The cache already holds the root lock's crates, after `install` | The lane builds offline | pass or FAIL |
| A fresh machine before `install` | `lint`'s deny and `policy.py` fail offline with cargo's missing-crate error. `install` fetches 8.1 MB of Polars crates once | not run for the lane; `lint` fails until `install` runs |
| The caller's shell holds a real key | The lane unsets it and sets the fake key. The helper sees only the fake key | pass |
| `cargo test --features polars` run by hand, outside the lane | The helper's sentence: run the tests through `check.sh` | fails, and sends nothing |

### Refusals, each before any request and each `Error::Usage`

| Input | Sentence | Change |
| --- | --- | --- |
| An `Int64` column | `the column counts is i64, not text` | Kept |
| A `Boolean` column | `the column flags is bool, not text` | The test's `Categorical` row becomes this row |
| A column with a null row | `the column holds nulls; the engine needs text, and NA rows are the caller's to drop` | Kept |
| A frame with no column `on` | `the frame holds no column {on}` | Kept |
| A question name that names a frame column | `the frame already holds a column named {name}` | Kept |
| A decide question to `score_series` | `score_series needs a score question, and this one is a decide question` | Kept |
| A score question to `tag_series` | `tag_series needs a tag question, and this one is a score question` | Kept |
| An empty column | An empty column of the method's type, 0 sends | Kept |

### Licenses in the root deny run

| Crate | License | Root deny today | After |
| --- | --- | --- | --- |
| `foldhash` 0.2.0 | Zlib | Not in the tree | Passes by its exception |
| `slotmap` | Zlib | Not in the tree | Passes by its exception |
| `xxhash-rust` | BSL-1.0 | Not in the tree | Passes by its exception |
| `ar_archive_writer` | Apache-2.0 WITH LLVM-exception | Not in the tree | Passes by its exception |
| A new Polars crate under Zlib | Zlib | Not in the tree | Fails: the exception names a crate, not the license |

## Proof

The lane is `libraries/polars/check.sh` in the `surfaces` rung. Each 0120 test keeps its assertion and its plant, and the build re-runs every plant against the moved code. The record shows each red, then green after the restore. Counts are the loopback backend's.

| # | Test and where it runs | What it proves | Planted fault that turns it red |
| --- | --- | --- | --- |
| 1 | `polars_door` `every_refusal_is_pinned_and_sends_nothing`, lane | Each refusal sentence in the table, whole, with kind `Usage` and 0 sends. The `Debug` assertion changes with the error type. It checks that `format!("{error:?}")` equals exactly `Usage(ErrorDetail { message: "<sentence>", retryable: false })` with the pinned sentence. So the `Debug` line holds the column name, dtype, or question name the sentence holds, and no row text | Drop the null check. Separately, drop the kind check in `score_series` and pass it a decide question. Each call sends |
| 2 | `polars_door` `the_callers_columns_come_back_unchanged`, lane | `Int64`, `Boolean`, and `List(String)` columns come back with their dtypes and values | Cast every caller column that is not text to `String` |
| 3 | `polars_door` failed marker, failed row, chunked and sliced, empty column, lane | 0120's rows R1-3 and the failed-marker text, pinned as literals | 0120's plants: write `null` for a failed cell; write a failed row as null and go on; read `chunks()[0]` for every chunk |
| 4 | `polars_cases`, lane | The shared cases 0120 names give equal values through the slice form and the door | Map a not-sure answer to `false` in `decide_series` |
| 5 | `polars_deadline`, lane | R1-24: a 200-row score column at throttle 8 on the 100 ms delay arm stops near 1 s with at most 96 counted | Pass `CallOptions::new()` to the engine in place of the caller's options |
| 6 | `polars_throttle`, lane | A Series runs at the throttle as a slice does, within 5 percent, and holds exactly 8 in flight | Loop `decide` per row in `decide_series` |
| 7 | The lane's probe and helper | R2-28 and the paid-backend guard | Run the lane with `CARGO_HOME` at an empty scratch folder: it prints not run, and the rung counts no pass. Add a failing test: FAIL, not "not run". Delete the `unset` and fake-key lines and set a sentinel key: every engine test fails in the helper before an engine is built |
| 8 | `lint`'s `cargo fmt --all -- --check` | rustfmt reaches the `cfg` module and the `[[test]]` paths | Add a misformatted line to `public/frame/column.rs` and to `tests/polars/door.rs`. Each fails `lint` |
| 9 | `inventory`, in `lint` | The public API with the feature off equals the frozen text, unchanged | Make the Polars-free helper `kind_word` `pub` and re-export it from `public/mod.rs` with no `cfg`. `inventory` names the added export |
| 10 | `policy.py` manifest check | Decision 11's table | In memory: `default = ["cli", "polars"]`; drop `optional = true`; add `polars-core` to the dev-dependencies. Each is refused with its sentence |
| 11 | `policy.py` rung check | No rung script passes `--all-features` | In memory: add `--all-features` to the `test` rung's text. Refused |
| 12 | `lint`'s root `cargo deny` and `policy.py`'s license walk | The four exceptions, and nothing wider | Remove the `xxhash-rust` exception from `deny.toml`: deny fails licenses. Remove `slotmap` from `LICENSE_EXCEPTIONS`: policy names the disallowed license |
| 13 | `surfaces --registry` | A `feature` folder holds only `check.sh` and `README.md` | The planted line `libraries/rust feature` is refused |
| 14 | `policy.py`'s binding test scan | R2-28 over the moved tests | In memory: add an `#[ignore]` test to a copy of `tests/polars/door.rs`. Refused |

The four questions for each new check. Rows 1 to 9 are moved or existing tests and add no new test.

- **Row 10.** It protects the promise that `thinkthen` compiles no Polars by default. A credible regression is a helper who adds `polars` to `default` "for convenience", or a test that needs a Polars type through a dev-dependency. No existing check reads the optional set with `polars` in it: today's check would refuse the new manifest outright, so it must change anyway. No test-only hook: it reads the manifest.
- **Row 11.** It protects the one-lane build. A credible regression is a rung edit that restores `--all-features` and adds Polars to every test build on the gate host. Nothing checks rung flags today. No hook.
- **Row 12.** It protects per-crate license review. A credible regression is a Polars bump that adds a crate under an admitted license. Today's root checks never meet the Polars tree. No hook.
- **Row 13.** It protects ADR 0047 item 4: no code under `libraries/` escapes a ceiling. A credible regression is Rust left in `libraries/polars` after the move. The registry has no `feature` state today. No hook: the plant edits the registry's own input, as its other plants do.
- **Row 14.** It protects R2-28 over tests that left a binding folder. A credible regression is a skip that returns early. The scan reads binding folders only today. No hook.

## Budgets, in nonblank lines

- Production Rust in `crates/thinkthen/src/public/frame.rs` and `frame/column.rs`: at most 2 files and 355 lines. The door measures 399 today: `lib.rs` 203, `column.rs` 148, and `error.rs` 48. The error type goes.
- Tests in `crates/thinkthen/tests/polars/`: at most 5 files and 770 lines. They measure 753 today.
- `crates/thinkthen/Cargo.toml`: at most 22 lines added.
- `libraries/polars/check.sh`: at most 35. `libraries/polars/README.md`: at most 45.
- Root `deny.toml`: at most 16 added.
- `policy.py`: at most 45 added. It deletes at least the 30 lines of the Polars binding checks.
- `sdlc/scripts/surfaces`: at most 15 added. `test` and `lint`: at most 3 changed lines each.
- Documentation: at most 40 across the ADR 0047 amendment, ADR 0017, 0084, `rust.md`, the `surfaces.txt` header, and the two issue edits.
- The root ratchet rises by at most 1,125, equal to the measured count. `libraries/polars/ratchet.json` (1,152) goes. The repository's Rust therefore falls by at least 27 lines: 355 production plus 770 test is 1,125, against the 1,152 removed.
- The root `Cargo.lock` gains at most 100 packages. It changes the version of no package it holds today.
- The offline cache on every machine grows by at most 10 MB of `.crate` files. The spike's lock measures 8.1 MB over 90 packages the root lock lacks. The record gives the root lock's own figure.
- Time on the gate host, under the heavy lock, measured before and after. The lane's warm run takes at most 30 s more than today's `libraries/polars/check.sh` warm run. Its cold run takes at most 60 s more. The warm `test` and `lint` rungs grow by at most 5 percent.

## Stop rules

Stop and bring it back for a re-score when any of these happens.

1. Adding Polars changes the version of any package the root lock holds today.
2. A cargo-deny run that uses the root file, or a binding's copy, fails on a root exception its tree never meets.
3. The door needs a root lint, `crates/thinkthen/clippy.toml`, or a `policy.py` table weakened, beyond the tables decision 11 names.
4. `cargo metadata --all-features` meets a license outside the four exceptions.
5. Any gate beyond the `install` rung's `cargo fetch` needs the network.
6. rustfmt does not reach the moved files (Proof row 8 stays green under its plant).
7. The door needs an edit to `public/error.rs` or to any file another in-flight ticket owns, beyond the overlaps named below.
8. A budget would be crossed.
9. A moved test needs a test-only hook.

## Overlap with in-flight tickets

The rule: whichever ticket lands second merges the other's changes.

| Ticket | Shared | Note |
| --- | --- | --- |
| 0123 relate | `sdlc/ratchet.json` | Re-measure the ceiling. 0123 also opens `public/error.rs`, which this ticket does not edit |
| 0124 answer cache | `sdlc/ratchet.json` | Re-measure. Same note on `public/error.rs` |
| 0125 audit | `sdlc/scripts/policy.py`, `sdlc/ratchet.json` | Different tables in `policy.py`. Merge by hand |
| 0126 wording | `crates/thinkthen/tests`, opened whole, and `sdlc/ratchet.json` | This ticket adds only the new folder `tests/polars/`. 0126 also opens `README.md`, which this ticket does not touch |
| 0127 test harness | `sdlc/scripts/lint`, `sdlc/scripts/test`, `sdlc/ratchet.json` | One flag leaves each rung line. 0127's `children` scan will read `tests/polars/`, which spawns no child |
| 0128 release | `sdlc/scripts` opened whole (`surfaces`, `policy.py`, `test`, `lint`), `crates/thinkthen/Cargo.toml`, `libraries` and `databases` opened whole (`libraries/polars`, `databases/sqlite/deny.toml`), and the release issue | See "What 0128 depends on". Its `--release` form and this ticket's `feature` state touch the same loop in `surfaces` |
| 0129 warm | None | 0129 opens `databases/sqlite` source, tests, and README, not `deny.toml` |

## Evidence

- Starts from: ticket 0120 and its build record `sdlc/records/0120-build-rust-polars.md`: the landed door, its eight test plants, a lock of 163 packages, and the finding that only Polars' wasm target adds packages under `getrandom` 0.2. Spike 257 (`~/workspace/experiments/257-thinkthen-rust-polars-spike/`): the four license exceptions and their chains, the facade needing no Polars feature, only the R1-4 test needing `polars-core`'s dtype features, and a 59 s cold build at `-j 4`. Ian's ruling 4 of 2026-09-25 and 0128's decision 15. `policy.py`'s `deny_failures`, which lets a binding leave out a root entry its tree never meets.
- Keeps: the five methods and their signatures, each refusal sentence, the column table of ADR 0047 item 10, one engine call per column at the throttle, the caller's `CallOptions` passed whole, Polars 0.55.2 in the lock, the lane's not-run probe, the fake-key guard, and every 0120 test with its plant. The public API with the feature off. The root toolchain.
- Changes: the door lives in `thinkthen` behind `polars` and returns `thinkthen::Error`. Its tests run from `crates/thinkthen/tests/polars/` in the lane and nowhere else. The R1-4 and refusal tests use `Int64` and `Boolean` in place of `Categorical`. The requirement is a caret, `0.55.2`. The root lock and root `deny.toml` take the Polars tree and its four exceptions. `test` and `lint` drop `--all-features`. The registry gains the `feature` state. SQLite's `deny.toml` and the binding's own lock, `deny.toml`, ratchet, and `policy.py` checks go.
- Proof: the fourteen rows above, each with its plant, and the lane, `lint`, `test`, and `surfaces` rungs run once each after a merge of `origin/main`. The record gives the lock count, the root ratchet change, and the before and after times.
- Defers: a frozen inventory of the feature-on API. The tests pin it by calling every method from outside the crate. `Categorical`, `Enum`, and struct column coverage, which needs Polars dtype features in a test build. The docs.rs metadata line, to 0128. Python Polars item A1 and the name-clash issue, to their own ticket. The Mac lane time. 0120's own defers: a lazy `Expr` door, the other verbs over a column, and more than one Polars minor version.

## What Ian can overturn

- This reading of ruling 4, which the release issue already marks as his to overturn.
- A1 in its own ticket, not this one.
- The `feature` state that keeps `libraries/polars` in the registry, in place of deleting the folder.
- The root names `thinkthen::PolarsEngine` and `thinkthen::polars`, in place of a public `thinkthen::frame` module.
- One error type: the door's `Error` retires for `thinkthen::Error`.
- `Int64` and `Boolean` in place of `Categorical` in two tests.
- The caret requirement `0.55.2` in place of 0120's exact `=0.55.2` (decision 1).
- The Polars tree in every machine's offline cache, 8.1 MB, which the root lock now names.
- The four exceptions in the root `deny.toml`, which every root deny run now carries.
- The lane in the `surfaces` rung, not the `test` rung.

## Complexity

Contract 2; state and timing 0; reach 3; proof 2; cost of error 2; total 9. Final level: 3. No behavior changes. The risk sits in the root lock, the license list, and the gate scripts that three other tickets also open.
