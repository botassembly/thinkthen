# Surfaces branch: third review — the fixes that did not hold, and the new defects

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
