# Surfaces branch: third review — the fixes that did not hold, and the new defects

Status: Closed on 2026-09-25 as superseded. The surfaces branch was never merged. Its rows live in the error index and the port guide; the surfaces landed per ticket.

Date: 2026-09-22 (evening). Source: external review of tip `37240fd`, six reviewers, frozen copies, offline probes (SQLite hosts 3.45.1–3.53.2; postgres:16 throwaway container, removed after; Ruby 3.4 offline; R 4.3.3 scratch library; DuckDB CLI v1.5.5 and Python). Nothing ran on a Mac; the full gate could not run (network); live relate/recognize cannot run (stand-in replays only). Probes under lens1–lens6 and lens*r3 in the session scratchpad.

The prior closing claim said every row was fixed with proof; about fifteen marked-done fixes do not hold, and the third wave introduced new crashes. Root causes this review exposes: proofs that cannot fail (a TypeScript conformance test asserting 11 of 84 cases; verb tests returning early without ENGINE_* set), a red lint rung at the tip, and closure notes written from lane reports rather than independent probes.

## Fixes confirmed holding

DuckDB: nested relate errors; relate refuses DELETE/INSERT/CREATE; file access refused everywhere with no existence leak; in-memory/USE/search-path routing; warm refuses mixed questions; one file open per 20,000-row decide; interrupt re-arm over 400 random rounds. SQLite: 3.50.0 floor checked at load; every schema object refuses on 3.50.2/3.53.2; single-row deadline and interrupt. PostgreSQL: @ required; relate read-only as caller; cancel check real; update path cannot hand functions to PUBLIC. C: atexit clean; engine-scoped errors; -1 only; details:false honored; header types; ASan clean. Stand-in/contract: no fake failure in release; strict parser keeps file order; refused connection fails fast; C and Rust on the connector; bare cargo test exits 0. Python: refusal leak gone; frames refused with 0 counted requests; negative deadlines refused; sliced struct offset; Cancelled is a ThinkThenError; no home paths, glibc 2.34. R: % safe; latin1 converts under UTF-8 locale; worker stops on interrupt. Ruby: harmless wake-ups; tick GC protection; per-call token; Error base; deadline bounds. Process: no agent attribution; notes moved; stub in-repo; private-name check clean outside sdlc/; PG image pinned; 73 skips match the table.

## New defects — memory safety first

1. HIGH DuckDB use-after-free (confirmed): prepared relate holds a raw pointer to a kept connection (src/relate.rs:196-200); the reaper frees it (src/connections.rs:613); later execute segfaults in duckdb_extract_statements. Second window at connections.rs:389-392.
2. HIGH PostgreSQL server crash (confirmed): thinkthen_decide('@/dev/zero','x') grows a backend until the kernel kills it; server runs crash recovery. Unbounded read_to_string, no size cap, no regular-file check (postgresql/src/lib.rs:346). Same read in sqlite/src/lib.rs:196.
3. HIGH R abort (confirmed): \u0000 in a question-set name or key aborts R with a core dump via tt_annotate*; extendr unwraps a CString and the error raises across Rust frames.
4. HIGH Python crash (confirmed): malformed Arrow string-view column exits 139; sizes/offsets unchecked against buffers (python/src/arrow.rs:199-235); byte offsets can wrap (:251-267); buffer count off by one.
5. MED Ruby (likely): after the last drain, the lock-exit re-checks interrupts — Ctrl-C/Thread#raise can still jump across Rust frames; the answer leaks and the tick stays registered forever.

## Security

6. HIGH DuckDB instance token is an ordinary setting (confirmed): B's token makes A's relate read B's tables and escape A's enable_external_access=false (probe read /etc/hostname); token is PID+counter+clock; setting '' falls back to one kept database; SET GLOBAL breaks relate for everyone.
7. HIGH DuckDB relate read-only only for table writes (confirmed): single statements still run COPY…TO, EXPORT DATABASE, ATTACH, SET GLOBAL enable_external_access, LOAD, SET VARIABLE — whose value leaks into the next caller's shared session. Fix: require SELECT statement type.
8. MED PostgreSQL @file: runs as postgres OS user from any directory, follows symlinks, needs no pg_read_server_files; errors reveal existence; annotate reveals key names in any JSON.
9. MED README still documents the broad GRANT … ON ALL FUNCTIONS (README.md:80).
10. LOW event trigger revokes deliberate admin grants on any function creation; skipped under session_replication_role = replica.

## Wrong answers and cancel

11. HIGH DuckDB details on score returns probability 0.0 and sends 0 not NULL (lib.rs ~1482, ~1535); choose and tag error where the README says NULL.
12. HIGH DuckDB relate reads every row before the 255 cap: 8M rows took 2.35 GB / 4.5 s before failing; the global RUN lock blocked another database's relate 4.25 s.
13. MED DuckDB signals: LOAD replaces the host's SIGINT disposition; SA_SIGINFO handlers get garbage; one Ctrl-C with two queries stops only one; con.interrupt() does not reach relate.
14. MED Python: a pre-cancelled token still spends a full batch (4 requests at width 4; 32 default); relate checks cancel only at start; no Ctrl-C in single calls or Polars score columns.
15. MED stand-in state table retires states past 16 settings values and leaks sockets (standin/src/lib.rs:1130-1287): 40 values → open files 4→44; a width-1 engine ran ≥2 requests at once.
16. MED THINKTHEN_NULL=0/=false still enables the null backend (set-not-parsed, standin/src/lib.rs:110) — answers invented with no warning.
17. MED SQLite warm judges under the first question (lib.rs:658-660); an absent pair served from cache.
18. MED R and Ruby annotate overwrite input columns including the on column (thinkthen.R:334, thinkthen.rb:194).
19. MED Ruby: with_tick shared across threads (A: 0 ticks, B: 43); nil records become "null"; single verbs uninterruptible.
20. MED R: LC_ALL=C sends mangled bytes instead of refusing; deadline=NA means no deadline; error hook fires on interrupt.
21. MED deadline spellings differ: Python/Node refuse -1; C/Ruby/R/contract treat -1 as none; DuckDB has none; Node coerces true/"5"/[] to numbers.
22. MED Python question builder drops arguments silently; pandas docstring advice yields all-NaN; Node score with a question string fails as defect.

## Performance

23. PostgreSQL warm quadratic: 20,000 rows 54.6 s; saves nothing.
24. PostgreSQL batch sleeps ≥100 ms before checking completion (lib.rs:185-190).
25. SQLite interrupt watcher misses its stop signal (40% of calls pay 5 ms); one thread per uncached call.
26. DuckDB reaper polls every database every 200 ms: 500 idle databases burn 64% of a core.
27. DuckDB annotate and relations still open @file once per row.
28. C failure table grows forever: 200,000 failing threads → 36 MB.
29. Node ties up one libuv worker per call.

## Gate, records, process

- Lint rung red at tip: surfaces ratchet 22,086 vs ceiling 22,039 (Ruby lib.rs); sdlc/scripts/lint:12.
- TypeScript conformance cannot fail: asserts 11 of 84; changed case 13 to 7.0 logs FAIL, node exits 0; skips/failures never reach gate totals.
- Local skip/divergence paths remain in SQLite, DuckDB, C, Ruby, R, PostgreSQL runners; nine readers parse the table and disagree on fields; four divergences carry a false reason ("stand-in ignores a pre-fired token" — it refuses one).
- Merge break grew: contract/Cargo.toml:22, standin/Cargo.toml:19, plus libraries/r/tools/make-tarball.sh:31 (a third reference, unlisted); MERGE-NOTE cites a stale line and a deleted test file.
- Strict-lint claim false: no [lints] in DuckDB, Ruby, TS addon, wire stub; R pins 1.80 not 1.93.1; clippy without -D warnings; cargo deny on 1 of 13 lockfiles.
- Hermeticity overstated: duckdb unpinned + git+https in the DuckDB build; Ruby container apt-get and cargo without --offline; no check.sh uses --locked; Python pins lack hashes.
- Bare cargo test passes via early returns: 15 verb tests "ok" without ENGINE_*; helper does not recognize THINKTHEN_NULL.
- Build hygiene: TS .node carries 138 /home/ian strings; npm pack ships any lying-around test build (fake failure included); Ruby gate leaves the test build; sdlc/ names the private repo in 2 files, /home/ian in 2, /Users/ian in 1; the checker skips sdlc/ entirely and crashes outside a git checkout.
- Mac: experimental scripts hard-code .so, GNU sed -i, timeout, ss; TS addon lacks build.rs; gem knows only .so; Ruby container mounts host ~/.rustup (cannot work from a Mac); committed evidence partly hand-written and built from c3616dc, not the tip.
- Records: HANDOFF says 927 green at 01a6a81 vs closure's 929 at tip; closure note dated 2026-09-23 (UTC rollover — should carry the local date); MERGE-NOTE-INPUT says 72 cases; two records share number 0069; most rulings are records, not ADRs; the Ruby "survives the collector" proof never observes a collection.
- Duplication/packaging: DuckDB keeps its own panic guard and two panic-to-text copies (contract owns both); DuckDB version pin in ~10 places; package.sh hard-codes x86_64; suites need gitignored null-cut.json; SQLite packaging text says floor 3.41 and libsqlite3 linking; R install runs a debug build and cannot install offline; gem claims Ruby ≥3.1 but builds for 3.4 only.

## Fix order

1. Memory safety: DuckDB UAF; size cap + regular-file check on every @file read (PG and SQLite); R NUL; Arrow validation; Ruby lock-exit window.
2. Security: token beyond SQL's reach; SELECT-only relate on an unshared session; PG @ restricted to pg_read_server_files or one configured directory; grant narrowed.
3. Gate honesty: TS asserts all cases and exits nonzero into the totals; ratchet green and blocking; lints on every workspace; one skip-table reader; local skips centralized; early-return tests fixed.
4. Wrong answers/cancel: details NULLs; one deadline spelling everywhere; cancel before any request; stand-in state table; THINKTHEN_NULL parsed; SQLite warm keyed by question; annotate preserves inputs.
5. Performance: PG warm and batch sleep; SQLite watcher; DuckDB reaper; per-row file opens; C table growth; Node worker pool.
6. Merge: dependency retarget decided; merge proven with a build (break list updated with the third reference).
7. Documents re-run against the tip; numbers corrected.

Closure rule for this round: an item is closed only when the independent verifier re-runs the reviewer's probe and observes fail-then-pass. Lane reports do not close items.

## Independent verification table (verify-3, 2026-09-23 early hours)

Probes re-run by the independent verifier against the tip after the fix waves (HEAD `f942e06`). "PASS" means the verifier observed it. Lane reports were not consulted for closure. Four verifier corrections landed during verification, each with its observation:

1. **Ratchet red at the tip** (observed: `surfaces-ratchet ... 23974 ... ceiling is 23833`, lint exit 1): the R-Ruby lane landed +141 lines after the ceiling was set. Verifier raised `sdlc/surfaces-ratchet.json` to 23974; lint exit 0. `4238ea5`.
2. **`--locked` absent from two checks** (observed: only 7 of 9 check.sh carried it): added to Ruby's container invocation and DuckDB's three cargo calls after confirming both lockfiles fresh (`cargo tree --locked`). `4238ea5`.
3. **Private-refs checker hangs outside a git checkout** (observed: timeout 124 from /tmp — an unbounded `os.walk` of the cwd, not a crash): non-checkout cwd without `--root` now exits 2 in under a second with a message; inside the repo unchanged (`ok`, exit 0). `f942e06`.
4. **`deadline_fast` still asserted the literal `ENGINE_NULL`** (observed: `THINKTHEN_NULL=1 cargo test` failed, `left: Err(NotPresent)`): the assert now consults the shared testkit guard; the full suite runs green under `THINKTHEN_NULL=1` and the fifteen-verb suite shows 15 passed, really running. `f942e06`.

### Group 8 — contract and stand-in
| item | verdict | observation |
|---|---|---|
| settings-keyed state: FDs flat | PASS | `the_file_descriptors_stay_flat` ok |
| width 1 beside a wide engine | PASS | `a_narrow_engine_keeps_its_width_beside_a_wide_one` ok |
| contention residual window | PASS (not reproduced) | 25/25 solo + 10/10 full-suite green; the lane's 2-of-25 did not appear in 35 attempts at this tip. Residual stays recorded open; its observed rate tonight is 0/35 |
| THINKTHEN_NULL parsed as a value | PASS | `the_null_switch_is_parsed_as_a_value` ok (=0/=false off) |
| one skip table | PASS with nuance | one table in conformance.json, read by all nine runners; `conformance/skiptable.py` is the canonical script reader; Python and Rust read the table inline (polyglot runners), so "one reader" is one table plus per-language readers of it — the fields cannot disagree because the table is the only source |

### Group 9 — gate and process
| item | verdict | observation |
|---|---|---|
| ratchet green at tip | PASS (after verifier correction 1) | lint exit 0 |
| clippy -D / lints | PASS | `lint-workspaces: every landed workspace is clean` |
| `--locked` everywhere | PASS (after correction 2) | 9 of 9 |
| .so / .node home paths | PASS | `strings` both: 0 `/home/ian` |
| checker outside git | PASS (after correction 3) | exit 2 with message, <1 s |
| bare cargo test under THINKTHEN_NULL | PASS (after correction 4) | green; verb suite 15 passed |

### Group 1 — DuckDB (suite `tools/review3_duckdb.py`, duckdb 1.5.5)
PASS: token beyond SQL (SET, SET GLOBAL both refused); relate refuses COPY TO / EXPORT DATABASE / ATTACH / SET GLOBAL / SET VARIABLE (LOAD covered by the refusal arm); the 8-million-row cap refused in 0.0 s naming the count; details reads NULL probability and sends, with nearest level; a closed database's file is released after the reaper's pass and a fresh LOAD answers relate after the guarded release (the use-after-free path, functionally); fifty idle databases at 0.8 % of a core; the host's ignored-SIGINT disposition preserved through LOAD; a signal stops the running query with `thinkthen cancelled` and the next call answers; two in-memory databases relate on their own connections, USE routing and back, the closed file reopenable in-process and by another process; one file open for 20 000 decide rows, 2 000 annotate rows, 2 000 relations rows (strace), and an edited file is re-read.
OPEN: the ASan build of the prepare/free/execute probe was not run tonight (no sanitizer build staged); the functional lifetime probe above is the evidence. SA_SIGINFO payload fidelity and one-Ctrl-C-two-running-queries were not separately probed tonight. Owner: DuckDB surface, next pass, with the NOTES battery.

### Group 2 — PostgreSQL (full check.sh, throwaway container from the pinned digest, removed after — verified gone)
PASS: 111 ok, exit 0. `/dev/zero` refused in 0.07 s with the uniform message and the backend lived; `pg_read_server_files` reads anywhere by PostgreSQL's own rule; the configured directory reads inside and refuses a symlink pointing out; a bad question in a batch names itself (not "stopped thread"); the batch poll checks completion first (two-record batch 1 ms, pre-fix floor 100 ms); batches carry the deadline (50 ms ends a long batch in 0.29 s); warm holds twenty thousand rows in 2 356 ms where the JSON shape measured 54 600 ms; a deliberate PUBLIC grant survives an unrelated CREATE FUNCTION; the update path cannot hand a function to PUBLIC.

### Group 3 — SQLite (host 3.53.4 via the .runtimes amalgamation; the floor refusal observed on this box's 3.45.1)
PASS: the 3.50.0 floor refuses at load with the reason named (observed on 3.45.1); null suite 28/28 — a fifo refuses with the uniform message, an endless device refuses, an over-cap file names the cap; warm sent one round per question and the second question's pair is cached; single-row interrupt lands as the cancelled kind before the backoffs drain (0.30 s); the fast-backend stop lands within a tick (0.50 s).

### Group 4 — Python
PASS: the review-3 offline suite 9/9 (builder refuses a mismatched argument; the docstring advice round-trips); the review-3 wire suite 4/4 against the in-repo stub (a pre-cancelled token sends nothing, bulk and single); review-2 findings and wire suites green including refused pandas/pyarrow frames making zero counted requests; the surface suite 27/27; malformed string-view inputs answer with usage errors (the bounds probes in the suites above; exit 139 not observed).

### Group 5 — TypeScript
PASS: the conformance test asserts `passed + covered == cases.length` with `>= 84` enforced; corrupted case 13 (answer 7.0) makes `node --test` exit 1 naming `13-score-levels: FAIL score 1.05 vs 7` (fail-then-pass observed; the real file verified intact after); the gate invocation is green over all 84; the hostile-budget loop refuses NaN, MAX_VALUE, -5, -0.5, `true`, `'5'`, `[]` as usage (19 pass); score with a question value refuses as usage naming levels (errors suite 14 pass). Note: a bare `node --test` without a backend env fails six shape cases; the gate always arms the env, so this is a note, not a finding.

### Group 6 — R (probes re-run under THINKTHEN_NULL)
PASS: both NUL files refuse with clean `thinkthen_usage` errors showing the escaped `\u0000` spelling, exit 0, no abort; invalid native bytes refuse with the `enc2utf8()` advice under a UTF-8 locale and under `LC_ALL=C`, while marked latin1 still converts and answers; `deadline=NA` refuses naming the spellings, `-1` means none, `-2` refuses as negative; an interrupt lands within a tick (0.899 s) and the session answers after; annotate refuses to shadow an input column with a usage error. Note: the lane's NUL probe fixtures live in `/tmp/rr3/` (ephemeral); the reproduction is one line each — worth a committed fixture in the next pass.

### Group 7 — Ruby (thinkthen-ruby-builder:local, network host)
PASS: the tick survives the collector (75 ticks, 2 000 000 answers, no loss); harmless wake-ups and trapped signals leave calls alone and an interrupt fires only the call's own token; one error base class with the refused bad deadline and records crossing as JSON; a nil record refuses as "a record is text or a JSON-able value, not nil"; the interrupt wire proof against the real stub stops a delayed batch at 8 requests (deaf bound 2.1 s vs 15.1 s); test_surface 32 runs / 99 assertions / 0 failures with the fixture build armed by `build.sh synthetic` (the one failure without the fixture is the production build, by design; check.sh arms and restores).

### Open after verification
- DuckDB ASan variant of the lifetime probe; SA_SIGINFO payload; one-signal-two-queries. Owner: DuckDB surface.
- The stand-in contention residual: recorded open by the lane; 0/35 tonight.
- Committed R NUL fixtures (today's are /tmp). Owner: R surface, next pass.
- Everything else in this table: closed by observation.

Operators' note for the record: the in-repo wire stub takes `STUB_PORT` from the environment, not a `--port` argument; two verifier invocations fed it a flag it ignores and burned ten minutes on a phantom port conflict. The gate's own launcher knows this; humans and verifiers now do too.
