# Surfaces branch: fourth review

Date: 2026-09-23. Source: external review of pushed tip `f942e06`. Companion to the three earlier reviews of 2026-09-22 and to the verification table appended to `2026-09-22-surfaces-branch-third-review-the-unheld-fixes.md` (`2b63c87`).

Six reviewers worked on a frozen export of `f942e06`. They built offline, including AddressSanitizer builds of DuckDB and the stand-in. PostgreSQL ran in one throwaway container from the pinned digest with no network, removed after and verified gone. Nothing ran on a Mac. The full gate did not run. Live `relate` and `recognize` cannot be tested because the stand-in replays them.

## Verdict

No sign-off. The verification table marks nearly every item closed by observation. Many rows do not hold when the reviewer's own probe is re-run. The lint rung is red at the tip. The TypeScript conformance test fails at the tip. Several tests behind PASS rows cannot fail. The wave added three new crashes and two new ways for one DuckDB database to read another's data.

## Process rules this review asks for

1. Every closing probe is shown failing on the old code and passing on the new code, with both outputs in the record.
2. The verifier runs the reviewer's probe. A lane's own test does not close an item.
3. The gate runs at the exact tip right before a closure note is written.

## Fixes confirmed holding

- DuckDB: the relate use-after-free is gone under AddressSanitizer (60 s, 12 threads, 1,859 load/close cycles). The token setting is gone (SET, SET GLOBAL and RESET refused). Relate refuses everything except SELECT. The lock is per database. The reaper uses 1% of a core at 500 databases. The host's SIGINT disposition is kept, and an SA_SIGINFO handler gets valid info. One file open per query. `package.sh` follows the host architecture.
- PostgreSQL: `/dev/zero`, `/dev/urandom`, a 1 TB sparse file and a directory refuse and the backend lives. The `pg_read_server_files` gate and the configured directory work, and only a superuser sets the directory. The event trigger keeps a deliberate PUBLIC grant. The README grant is narrowed. A two-record batch takes 2 ms. Warm on 20,000 short rows takes 2.3 s.
- SQLite: `@file` reads are capped and refuse devices, fifos and directories. Warm is keyed by question. The shared watcher landed 60 of 60 interrupts across 8 connections, and 20,000 uncached calls went from 31.9 s to 0.17 s.
- Python and Node: a pre-cancelled token sends 0 requests across 18 verb forms. `-1` means no deadline. Node refuses `true`, `"5"`, `[]` and `NaN`. The Python builder refuses stray arguments. The `.node` has no home paths. The addon has `build.rs`.
- R: NUL is refused at every entry point tried. `deadline=NA` is refused. Annotate refuses to shadow an input column. Interrupt lands in about 0.15 s and the error hook no longer runs on it.
- Ruby: `nil` records refuse. `with_tick` is per-thread (83 and 83 ticks). The round-three `Thread#raise` bulk stress ran 36 of 36 clean. The gemspec says `>= 3.4`. The builder no longer mounts the host `~/.rustup`.
- Stand-in: `THINKTHEN_NULL=0` and `=false` mean off. Width 1 holds under churn. Retired pools close after the grace when a later sweep runs.
- Process: no agent attribution on the 35 commits. The duplicate record 0069 is gone. Python requirements carry hashes. The merged tree builds when `crates/thinkthen-core` is restored.

## Crashes and memory safety

1. HIGH, confirmed. Stand-in state table use-after-free. Eviction and retraction free the slot box at `standin/src/lib.rs:1455,1540,1617` while other threads read it at `:1487,1526,1652,1666`. The grace protects the `Arc<Inner>` and not the box. AddressSanitizer: heap-use-after-free in 5 of 6 runs. Release build: SIGSEGV with more than 64 settings values and more than one thread. Through the public C door, 70 engines with distinct timeouts and 32 threads crashed 4 of 4 (`lens1r4/c/churn.c`). Safe Rust reaches it through `from_settings`. The comment at `:1665` ("a stored pointer is never freed") is false. The compare-and-swap at `:1605` also has an address-reuse hazard (likely).
2. HIGH, confirmed. Ruby VM crash or freeze. A trap handler that raises, a signal every 0.2 ms and a 200k-record `decide_many` crashed in `coroutine_transfer` four runs in a row. Other runs died with `[BUG] unexpected situation` or froze. Pure Ruby under the same flood ran 100 of 100 clean. `libraries/ruby/src/lib.rs:385-387` wraps `rb_thread_call_with_gvl` in `rb_protect`, which catches the raise while the VM lock is held.
3. HIGH, confirmed. Python Arrow input still crashes the process (exit 139). The fix caps sizes at 64 MiB (`libraries/python/src/arrow.rs:166`) without bounding them by the buffers. Crashing inputs: a string view at offset 60,000,000 over a 100-byte buffer; Utf8 offsets past the buffer; a null offsets pointer; a null buffers pointer; a null children pointer in a frame root (`:405`). Length 2^40 aborts in `Vec::with_capacity` (`:197`). The `n_buffers - 2` off-by-one at `:207` remains; a view pointing at the sizes buffer is accepted. The new unit tests build views without the sizes buffer.

## Security

4. HIGH, confirmed. DuckDB routes a caller to the wrong database (`databases/duckdb/src/connections.rs:240`). A closes all its connections while the host keeps it open, the reaper drops A, B loads, A reconnects. A's relate then uses B's kept connection and read B's table and `/etc/hostname` with A's external access off (`lens5r4/fastpath.py`). A connection pool triggers it.
5. HIGH, confirmed. DuckDB identity probe can be forged. A read-only database gets no probe and falls back to its name. Another database detaches its own probe and attaches `:memory:` under that name, then reads the first one's tables and files (`lens5r4/forge3.py`). A known probe name forges a writable database too (`forge2.py`). Probe names are clock, counter and PID, visible in `SHOW DATABASES`. The comment at `connections.rs:95` says 128-bit random.
6. MED-HIGH, confirmed. DuckDB file-access bypass during reaping. The reaper marks up to 8 live databases retired per pass (`connections.rs:712`). `file_read_refusal` then skips them (`:330`) and the read falls back to `std::fs::read_to_string` (`:505`). 53 of 342,636 `@file` calls read the file with external access off. The same window fails 46 of 23,570 live relate calls.
7. MED, confirmed. `@file` check-then-open race in PostgreSQL (`databases/postgresql/src/lib.rs:394,402,410`) and SQLite (`databases/sqlite/src/lib.rs:212,221`). A symlink swap escaped the PostgreSQL directory in 26 of 20,000 reads and the error leaked the outside file's JSON key names. A fifo swapped in hung a PostgreSQL backend in `open()` past `pg_cancel_backend` and `pg_terminate_backend`. SQLite hung at iteration 59. Fix: open once with `O_NONBLOCK | O_NOFOLLOW` and check the opened handle.
8. MED, confirmed. DuckDB row cap is bypassable. The count and the real run execute separately. `range(3 + (random()<0.5)::INT*8000000)` passed the count and then held 8M rows (57 to 985 MB) before the check at `src/relate.rs:669`.

## Wrong answers and cancel

9. HIGH, confirmed regression. DuckDB details on decide always reads NULL probability and sends (round three: `0.97|1`). `src/lib.rs:1293,1353` set every row NULL and never clear it. Choose and tag give `model ''` where the README promises NULL.
10. HIGH, confirmed. R scripts continue past errors. With any `options(error=...)` hook set, an uncaught thinkthen error does not stop an Rscript, the hook does not run, and it exits 0. `libraries/r/thinkthen/R/thinkthen.R:36-40` restores the hook before R decides to stop.
11. MED, confirmed. Ruby single calls resend on a signal. One trapped USR1 during a single `decide` sent the paid request twice (two in flight). A signal every 200 ms fails the call with `Interrupted system call`. SIGINT raises 4.0 s late.
12. MED, confirmed regression. SQLite interrupt lost in a forked child. The child inherits the started flag without the thread (`databases/sqlite/src/lib.rs:342-351`). An interrupted call ran 3.00 s to a backend failure where round three cancelled in 0.10 s. A fork during the watcher's lock would deadlock the child (risk).
13. MED. DuckDB cancel. One Ctrl-C with two running queries stopped both in 2 of 5 trials and neither in 2. With four, 1 of 4 stopped. Neither Ctrl-C nor `con.interrupt()` stops a slow relate query, and relate runs its query twice.
14. MED. Python Ctrl-C. A single `decide` retries the interrupted request and raises at 4.3 s. A Polars score column runs all 8 rows first. `step()` has no signal polling (`libraries/python/src/lib.rs:720-735`).
15. MED. Other wrong answers:
    - Ruby annotate checks only the first record's keys (`libraries/ruby/lib/thinkthen.rb:203`); a second record's column was overwritten.
    - R under `LC_ALL=C` still sends `caf<c3><a9>` for native-marked valid UTF-8 (`libraries/r/thinkthen/src/rust/src/lib.rs:169-185`).
    - PostgreSQL warm saves nothing: warm then decide on 20,000 pairs used 40,000 requests. With 2 KB evidence it stays quadratic: 2,000 / 4,000 / 8,000 rows took 1.3 / 14.3 / 37.5 s.
    - SQLite `@q.json` cache stays stale after the file is deleted or rewritten.
    - SQLite warm with interleaved questions degrades to one round per row (`databases/sqlite/src/lib.rs:731`).
    - Pandas docstring advice gives all NaN on any non-default index (`libraries/python/thinkthen/__init__.py:116,145`).
    - `question(decide=..., choose=...)` silently drops the second verb (`libraries/python/src/lib.rs:538`).
    - Stand-in `request_digest` reads the base URL from the environment (`standin/src/lib.rs:784`), so an engine with an explicit address reports the wrong digest.

## Gate honesty

16. The surfaces ratchet is red at `f942e06`: 23,973 lines against a ceiling of 23,974, exit 1. The tip commit removed a line after `4238ea5` set the ceiling.
17. TypeScript conformance fails at the tip: `25-find-none-fits` fails because the runner never passes the `none` facet to the skip reader. HANDOFF admits it and the verification table says "green over all 84". Separately, corrupting cases 15 and 19 still reports pass: mismatches `return note(...)` at `libraries/typescript/tests/conformance.test.mjs:111,115,217,219`. The standing probe corrupts only case 13. The runner's `default:` arm counts an unknown verb as a pass.
18. Tests behind PASS rows cannot fail. `standin/tests/settings_state.rs` is byte-identical to `37240fd` and passes on the leaking code; it uses two settings values. At 200 values, open files went 4 to 194 and RSS 2 to 55 MB. Correction 4 made `deadline_fast.rs:27` a tautology. The Ruby GC test cannot fail because the tick stays rooted in `Thread.current[:thinkthen_tick]`. `the_testkit_reports_a_visible_skip` leaks `THINKTHEN_BASE_URL`, and the digest test fails 4 of 50 runs. `settings_state.rs` itself is flaky under load (2 of 50) because it counts connections on the server side.
19. Lints are not strict. `sdlc/scripts/lint-workspaces` runs clippy without `-D warnings` or `--all-features`; with them 10 of 11 buildable workspaces fail (DuckDB 46 errors, R 25). A workspace whose runtime is missing is skipped without failing. Ruby and DuckDB have no `[lints]` table. `cargo deny` covers 12 of 13 lockfiles.
20. `--locked` counts files, not calls. Still unlocked: `libraries/rust/check.sh:29`, `databases/postgresql/check.sh:74` (`pgrx package`), `maturin develop`, R's Makevars, `napi build`, every `package.sh`.
21. Skips. Python, C and Rust keep their own readers. `skiptable.py lookup` always passes `wire=False`, so R, Ruby and DuckDB ignore `unless: wire`. DuckDB treats a failed lookup as run. Local skip paths remain in Ruby, R, C, Python, DuckDB, SQLite and PostgreSQL runners. The cancel reason is still false: case 18 expects the cancelled error and the real gap is a missing cancel path. `kind: local` claims no file door on surfaces that have one. `scripts/check_surfaces.sh:42` counts node `# SKIP` lines as green, and covered cases never reach the totals. `skiptable.py validate` is not in the gate.
22. Bare `cargo test` in `libraries/rust` shows 15 passed and no skip lines; skip notes go to hidden stderr, and 18 tests skip. Under `THINKTHEN_NULL=1`, one verb test still returns early, and 8 C tests fail on the old `ENGINE_NULL` literal (`tests/cancel.rs:32`, `concurrency.rs:53`, `deadline_fast.rs:36`, `error_threads.rs:23`).
23. Ceiling raises break `CLAUDE.md`. `e11bb57` and `4238ea5` have empty bodies. None names a second-agent review. 29 of 35 commits have no body.

## Build, packaging and records

- HIGH, confirmed. R does not install from a fresh checkout. `--offline` was added while `Makevars.in:13-16` points `CARGO_HOME` at an empty, gitignored `src/.cargo`: `no matching package named serde`. The install still builds the `document` binary in debug.
- Ruby's builder image lacks clippy and rustfmt that `rust-toolchain.toml` requires, so every build downloads. `build.sh` never rebuilds an existing image, and its header still describes the host mount.
- Ruby and R `check.sh` restore the production build only on success.
- Release C and SQLite libraries carry the builder's home path (141 and 152 strings). `sdlc/records/surfaces-notes/NOTES-gate-process.md:92-94` names the private repo and two developer home paths.
- Mac-labelled scripts keep `ss`, `/usr/bin/pg_config`, `timeout`, GNU `sed -i` (also in `lint-workspaces`), hard-coded `.so` names and a `mktemp` template with a suffix. The README cites `scripts/gate-macos.md`, which does not exist. The committed Mac `ruby.log` was edited after capture without a note.
- `databases/sqlite/package.sh` installs Ubuntu 24.04's SQLite 3.45.1, which the 3.50 floor refuses. The floor excludes every stock Linux SQLite known to the reviewers; the docs name only Ubuntu 24.04.
- DuckDB's build still installs `duckdb` unpinned and a `git+https` package; `scripts/gate-hermeticity.md` omits both. Two DuckDB suites still need the gitignored `null-cut.json`. The Makefile keeps its own version pin.
- The merge conflicts in two `sdlc/issues` files and the merged contract fails cargo check. Option (a) is a recommendation, not a recorded decision. MERGE-NOTE cites stale lines and says 11 lockfiles.
- The deadline ruling is an "ADR draft" inside a notes file. No ADR landed.
- HANDOFF shows `failed=5` beside "exit 0", which the script cannot produce.

## Leftovers still open

- C failure table grows per thread: 200,000 threads took 36 MB, 28 MB left after `engine_free` (`libraries/c/src/lib.rs:101`). Missing from the verification table.
- Width has no upper bound (width 100,000 panics to rc 6 at 322 MB). The JSON door acts on key presence (`"rank":false`). `Options::deadline_in` has no checked add. `from_settings` calls `.expect`. DNS and TLS errors retry.
- `index.d.ts:227` rejects the new score form. Python and Node docs still say `-1` is refused. Wrong-question-type calls report defect or backend instead of usage.
- The Python review-3 suites are not in `check.sh`. No `.pyi` or `py.typed`. The wheel is tagged `manylinux_2_39`. `npm pack` ships no LICENSE.
- Moved output children have a no-op release (`arrow.rs:838`); output offsets can wrap past 2 GiB; `on=` drops dtype metadata.
- DuckDB leaks logical types (`warm.rs:165-169`, `usage.rs:36-38`), uses the deprecated result API 9 times, builds with 22 warnings, ships `ENGINE_TEST_PANIC` in release, and locks a mutex in its signal handler (`lib.rs:292`). 20,000 rows still make 20,000 `statx` and `readlink` calls.
- SQLite `recognize` and `relate` pass empty options (`lib.rs:1070,1326`). Its question cache has no bound (300,000 questions, 422 MB, not freed on close). Six tests hard-code `libthinkthen0.so`.
- R `tt_raise_interrupt` calls `R_CheckUserInterrupt` across Rust frames (`lib.rs:271-274`). `lib.rs:599` never substitutes `ERROR_SEP`. Ruby `>= 3.4` admits 4.x with no ABI check.

## Fix order

1. Stand-in use-after-free, Ruby `rb_protect` crash, Python Arrow bounds.
2. DuckDB fast-path routing, probe forgery, reaper file-access window; `@file` open-once in PostgreSQL and SQLite.
3. DuckDB details NULL, R error hook, R fresh install.
4. Gate: ratchet at the tip, TypeScript mismatches fail, tests that can fail, `-D warnings`, one skip reader with the `wire` and `none` facets, skips in totals.
5. The rest.
