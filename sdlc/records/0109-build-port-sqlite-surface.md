# 0109 build: the SQLite surface on the public Rust API

Built 2026-09-25 by Claude (Opus) on `ticket/0109-port-sqlite-surface`, from `9efb9abc`. It brings `databases/sqlite` over from tag `surfaces-wave7-frozen-2026-09-24b` and rebuilds it on the public `thinkthen` API. Code review: pending, by a fresh Claude session.

## What landed

- `databases/sqlite` is the unpublished crate `thinkthen-sqlite`, library `thinkthen0`, `cdylib` only, in its own workspace (ADR 0047). It depends on `thinkthen` by path with default features off, `rusqlite` 0.40.2, `libc` 0.2.189, and `serde_json` 1.0.151. The lock resolves `thinkthen`'s tree to the root lock's versions. It adds the crates the ticket lists and no others.
- Seven source files: `lib.rs` (the failure type, the kind table, the one panic guard), `ffi.rs` (every `unsafe` block, the entry point, the API-table tail, the pin, the virtual-table glue), `settings.rs` (the process engine and the four settings), `question.rs` (the file door and the parse caches), `worker.rs` (the detachable worker), `scalars.rs` (the scalars and the warm aggregate), and `tables.rs` (recognize and relate).
- Retired from the tag: the saved-answer map, the hub, the watcher thread, the fork generation, the stand-in imports, `package.sh`, `tools/`, the wire-stub tests, and `ENGINE_NULL`, `ENGINE_WIDTH`, and `THINKTHEN_NULL`. The width setting is now `thinkthen_throttle`.
- Eight Python test files beside `tests/helper.py`, plus `tests/conformance.py` and its planted-failure test. `tests/slide.sql` is unchanged. `examples.json` is re-derived on the generic arm.
- `check.sh`, `setup.sh`, `tests/host_sqlite.sh`, and `amalgamation.sha256` for the SQLite 3.50.0 toolchain under `~/.cache/thinkthen-toolchains/`.
- `README.md`, a new `NOTES.md`, an "As built" section in `sdlc/planning/databases/sqlite.md`, and a SQLite section in ADR 0047.
- `sdlc/surfaces.txt` lists `databases/sqlite` as landed.

## Proof

- `check.sh` exits 0 under the heavy lock: fmt, Clippy with warnings denied, 7 unit tests, the remapped release build with no home path, one exported symbol, one `catch_unwind` call, and 38 Python tests across nine files.
- Conformance: 38 pass, 0 FAIL, 16 not run, 54 of 54. The not-run cases are `rank` (3), `find` (2), the `on` case, the defect injection, and the nine recognize cases with relations (decision 10).
- `python3 sdlc/scripts/policy.py` passes with the binding in place.
- `cargo deny` with `databases/sqlite/deny.toml` passes offline. With the root `deny.toml` it rejects `foldhash` 0.2.0 on Zlib.
- The toolchain came from the existing cache. Nothing was downloaded. `tests/host_sqlite.sh` rebuilt the host library and CLI once to write its hash stamp.

## Plants

Each plant was applied to the source, rebuilt, and run against its test. Every one turned red, and the restored tree passed `check.sh`.

| Row | Plant | Red result |
|---|---|---|
| R1-14 | read the interrupt through the first connection's handle | the second connection's call is not cancelled |
| R1-17 | `thinkthen_score` without `SQLITE_DIRECTONLY` | the function list shows a registration without the flag |
| R2-1 | floor lowered to 3.45.0 | the stock 3.45.1 host loads |
| R3-2b | size cap dropped | the over-cap file fails at parse with another sentence |
| R4-6 | open without `O_NONBLOCK` | the fifo child runs past 5 s |
| R5-20 | open with `O_NOFOLLOW` | the symlink refuses |
| R3-22 | a full tick before each read | 200 cached calls take 10.1 s |
| R4-17 recognize | the call listens on no connection | no cancel within 100 ms |
| R4-17 relate | worker options without the token | 2 sends after the release |
| case 18 | the same plant, on the warm | 48 sends after the release |
| R2-24 | single calls wait on the calling thread | no cancel within 100 ms |
| R2-24 | ` (retryable)` dropped | the pinned 503 sentence differs |
| R2-10 | any negative deadline as none | `-2` answers |
| R2-10 | worker options without `deadline_millis` | the held call answers after the release |
| R1-11 | a REAL through `Duration::from_secs_f64` | `1e300` panics into `defect` |
| R2-22 | warm groups keyed by text alone | warm counts 1 |
| R5-18 | groups found by linear search | the unit test ran 556 s before it was stopped |
| R4-17 | parse-cache eviction dropped | the unit test holds 5,000 |
| R1-31 | `Deadline` mapped to `backend` | the unit test fails |
| R1-10 | the guard removed | the unit test fails |
| R2-31 | a second `catch_unwind` | the guard count reads 2 |
| symbol | a second `#[unsafe(no_mangle)]` function | `nm` lists two symbols |
| R1-28 | an `unwrap()` in production | Clippy fails |
| R3-28 | one misformatted line | `cargo fmt --check` fails |
| R3-28 | the `foldhash` exception dropped | deny fails on Zlib |
| R1-30 | the build without the remap | `strings` finds 175 home paths |
| R3-29 | `--offline` dropped from the build | the flag step names the line |
| decision 16 | the helper passes the caller's key through | the child reads the sentinel |
| seed | the engine from `Engine::builder()` | 1 send, and nothing lands in `THINKTHEN_CACHE` |
| throttle | the setter dropped | the held warm reaches 4 |
| max_requests | the setter dropped | the warm answers 3 |
| cache | the folder ignored | the second child sends again |
| cache_bytes | stored without the check | `0` returns 0 |
| settings | a setting after the build applies silently | the call returns 8 |
| R3-30 | the runner skips one case | the counts sum to 53 of 54 |
| R5-32 | a mismatch reported as not run | the planted-failure test fails |

Not planted: R4-21 and R6-9. Their plant starts a helper thread in the parent, and the worker design has no such thread to plant. Both fork tests pass. R6-14's plant adds an answer map the design does not hold. Its test passes against the engine cache.

## Departures from the ticket's text

`databases/sqlite/NOTES.md` lists each one: the fixed loopback key, the 2 sends under `max_requests`, the retry count's source, `thinkthen_usage()` building no engine, case 41 for "case 68", the three added not-run forms, the cancelled case's 1 send, and the schema-load sentences.

## Findings for landing

- `sdlc/scripts/surfaces --registry` runs `cargo deny` with the root `deny.toml` for each landed binding. With this surface landed, `lint` fails on `foldhash` until that call uses `$surface/deny.toml` when present. The ticket's lint check comparing the two deny files is also not written. The port's brief forbade ladder-script changes, so both wait for the landing agent.
- The full ladder was not run, as the brief directed.

## Budget

- Production Rust: 1,392 nonblank lines in seven files, each under 500. Unit tests: 158. `ratchet.json` holds 1,550.
- Python: 896 lines in eight test files and the helper. The runner and its planted-failure test: 225. `ratchet.py.json` holds 1,121. `ratchet.sql.json` holds 18.
- Scripts: 99 nonblank lines. Documentation: 89 nonblank lines added.
- No change under `crates/thinkthen`, no backend arm, and no dependency beyond the ticket's list.
