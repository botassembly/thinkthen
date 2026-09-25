# Surfaces branch: second review — new defects and leftovers

Status: Closed on 2026-09-25 as superseded. The surfaces branch was never merged. Its rows live in the error index, sorted by sdlc/planning/surfaces-port-guide.md; the port tickets re-proved them.

Date: 2026-09-22. Source: external review of pushed tip `50c6327` (three conformance commits landed after the snapshot and got a light read; phase 3 was live in the worktree during review; phases 4–5 had not started). Companion to `2026-09-22-surfaces-branch-review-the-full-findings.md`. The reviewers probed offline copies and confirmed the first wave's headline fixes hold and its new tests catch the old bugs.

## Confirmed fixed and holding

DuckDB row mapping (10,000 mixed NULL/two-text rows → 3333|3333|3334, probability and score exact); the DuckDB driver compares exactly and exits nonzero; Python piecewise/sliced/empty/view tables, cleared release pointers, category/struct/list/tz round-trips; C two-thread use-after-free gone under ASan with 19 symbols guarded; one checked deadline conversion contract-wide (huge/NaN/infinite refused in Node, Ruby, Python, C); Node 2,000 calls leave zero listeners; Ruby Ctrl-C/Thread#raise/Timeout stop calls; R columns typed by kind; SQLite per-call handle, no second SQLite load, views/triggers blocked; PostgreSQL 15 revokes, real batch errors, a deadline setting; stand-in fixture off by default, retry sleeps honor cancel/deadline; conformance score 1→3 and tag 1→4 cases, one shared skip table.

## New defects — security (fix first)

1. SQLite: on 3.45.1 (Ubuntu 24.04 Python's build) a CHECK constraint in an untrusted database still calls the functions; read /etc/os-release with trusted_schema=0. The extension accepts 3.41+. No CHECK test. Fix: make every schema object (views, triggers, CHECK, defaults, indexes) unable to reach the functions on every accepted version, or raise the minimum — evidence first, then the guard and its test.
2. DuckDB relate runs outside the caller's transaction: the relate query can hold any SQL including several statements and commits on its own; a DELETE inside relate survived the caller's ROLLBACK (relate.rs:448). Fix: read-only, single statement.
3. DuckDB file-access holes: thinkthen_relations reads files with access off and its errors reveal existence (relations.rs:117); allowed_directories and disabled_filesystems ignored everywhere; a prepared relate keeps the bind-time switch. Fix with the same config-check door the others got.
4. PostgreSQL: any non-JSON text is treated as a file path with no `@` (lib.rs:230,289) — a granted role reads files as postgres; the documented GRANT ON ALL FUNCTIONS re-grants other extensions; the revoke runs only at install so ALTER EXTENSION UPDATE re-grants PUBLIC. Fix: require `@`, narrow the grant to named functions, re-revoke in the update path.

## New defects — crashes and hangs

5. R crashes on `%` in any text quoted back in an error (extendr passes it as a printf format): tt_recognize("100% sure…") segfaults. Escape, and convert latin1; invalid UTF-8 must error, not answer FALSE.
6. DuckDB nested relate hangs forever, Ctrl-C cannot stop it (connections.rs:64). Detect and error, or make the lock reentrant.
7. C: the panic guard's own error path panics once thread-local storage tears down — thinkthen_engine_free from atexit exits 134. The guard must be unable to panic at teardown.
8. Ruby with_tick: a proc still held in Rust memory is unprotected from GC — likely use-after-free with two threads.

## New defects — wrong results without error

9. Stand-in keyed wrongly: two engines with differing settings share state — width 1 allowed 8 concurrent; FDs rose 55→133 over 40 calls (standin/src/lib.rs:1149). One state per settings value.
10. Deadlines that silently mean none: any negative in C (lib.rs:414); exactly -1 in Python and Node (deadlineMs: end - Date.now() can land there by chance); Node null reads as spent. Only an explicit no-deadline spelling means none; everything else refuses.
11. Python runs the paid batch before refusing pandas/pyarrow input (__init__.py:139-150) — 18 requests in one refusal; the test that claims otherwise does not count requests. Refuse before any request; count requests in the test.
12. Python sliced struct stream puts answers on wrong rows (arrow.rs:430-437).
13. DuckDB relate routing: two in-memory databases both register as "memory" so relate fails in both; ATTACH … USE counts the main database's table; the saved connection keeps a closed file locked. Key by attached database identity, close with the database.
14. DuckDB: an interrupt within 250 ms of an engine call still poisons the next query (4 of 5 runs); DuckDB's own cancel (con.interrupt, timeouts) never reaches the engine.
15. Ruby: Thread#wakeup and trapped signals like USR1 now cancel calls; an interrupt fires the caller's shared token, cancelling every call sharing it. Distinguish spurious wake-ups; fire only a per-call token.
16. C error slot: one engine per thread — a failure on engine B makes engine A report "no failure"; a freed engine's error appears on a new engine reusing its address.

## Memory and performance

17. Python leaks ~1 MB per refused Arrow input — refused inputs never released.
18. DuckDB re-reads an @file question every row: 20,000 rows → 20,022 file opens, 5x slower. Read once per query.
19. PostgreSQL warm rewrites its JSON state every row and saves nothing under the stand-in.

## First-review items still open

- The weaker contract parser (accepts version: 99, unknown keys, sorts names) — adopt core's strict one or fix the deltas; libraries accept files the CLI rejects and order annotate columns differently.
- The fixture is armed by environment variable — a shipped libthinkthen.so still honors ENGINE_SYNTHETIC_PARTIAL=1 on the live path. Compile-time only.
- Transport errors misclassified retryable (a refused connection wastes 3 s); unprefixed ENGINE_* variables remain.
- Connector incomplete: C and Rust surfaces still name BlockingEngine; DuckDB calls standin::request_digest — the one-dependency-line swap is still false for three surfaces.
- Warm judges under the wrong question (DuckDB: last seen; SQLite: first) and DuckDB swallows errors as 0.
- Row-at-a-time requests (DuckDB choose/score/tag/details/annotate; R choose/score/tag) — host-side batching waits on the packing encode the wire probe specified; build team owns the engine side.
- Cancel gaps: SQLite single-row has no deadline or cancel; PostgreSQL batches ignore the deadline; Python has no cancel token and Polars never polls within a column.
- Error classes: Python Cancelled is not a ThinkThenError and SystemExit becomes Cancelled; Ruby has no shared base, drops a bad deadline silently, sends records via to_s.
- C door: details:false still enables the audit view; header still says unsigned long, promises a .pc file, no SONAME.
- Shapes: Python and TypeScript differ; Node holds one worker thread per call.
- plain cargo test in libraries/rust still fails 17 tests (phase 3 was mid-fix at review time).

## Phases 4 and 5 gaps (not yet started at review time)

Private repo name on 47 lines in 30 files; /home/ian on 20 lines; the built Python .so embeds 157 home paths; nothing checks either. Nine notes files at the root. Licenses missing in six manifests; gem UNLICENSED, no platform; R DESCRIPTION names a LICENSE file that does not exist. Six manifests lack strict lints; four toolchain versions. Gate runs npm install, unpinned uv pip install, a PyPI fetch, floating Docker tags; wire stub still in experiments/. x86_64 hardcoded in four build scripts; no Mac evidence committed; Python .so needs glibc 2.39; no wheel or napi build path. The 30 vendored DuckDB tool files lack license and upstream version. Documents carry false claims: HANDOFF "all complete and green" and "one dependency line"; HANDOFF/MERGE-NOTE case counts 74/72 versus the real 84; FINDINGS says pg_cancel_backend works; SURFACES says "ten surfaces".

## Merge, records, process

- The trial merge cannot build: main deleted crates/thinkthen-core in 5e1dafd (main now 203 commits ahead); contract/Cargo.toml:19 and standin/Cargo.toml:19 still point at ../crates/thinkthen-core. Git shows two text conflicts but the merged tree does not build. The merge-first recommendation relied on the text view; the build team needs: the dependency retarget decided, then the trial merge run as a build.
- Rulings lack durable records (NOTES-only or nowhere): DuckDB relate option A; the R residual window; PostgreSQL deadline in place of cancel; the C lane's header edit permission; the phase-3 driver permission. The C DESIGN.md "under Ian's ruling" provenance checks out (session of 2026-09-22: C supports everything through the C door; C ABI design assigned to the library team) but needs the ADR.
- No commit references the findings issue; the issue has no progress notes. Future commits reference both issues; both get progress updates.
- Duplication grew: the panic guard written seven ways, panic-to-text five, skip-table reader nine. The contract should own the first two as it owns the deadline.
- The wave added 3,632 lines of Rust that main's ratchet does not count; the gate does not cover the new workspaces. The checker's own test, package.sh files, and benches run from no gate step.
- Dependencies: no vulnerabilities in 12 lockfiles; two unmaintained crates flagged (serde_cbor in PostgreSQL, paste in R) — record and plan replacements.

## Fix order

1. Security items 1–4. 2. Crashes 5–8. 3. Wrong answers 9–16 plus memory/perf 17–19. 4. Merge: dependency retarget decided and the trial merge run as a build (build team, with our enumerated break list). 5. Phases 4–5 as announced, plus ADRs for the rulings, the shared panic guard in the contract, and the gate over the new workspaces.

## Progress: wave 2 landed, wave 3 queued (2026-09-22)

As of branch `surfaces` tip `6a829b8`. Wave 2 (conformance honesty) landed after the review's snapshot; wave 3 (this issue's fixes) has not landed.

**Landed since the review snapshot**, each closing a gap this review listed:

- `50c6327` the checker honors its file argument and runs from the repository root.
- `c83add5` every runner reads its skips from one table in the conformance file; the per-runner hardcoded lists are gone.
- `bcaa5f7` score and tag coverage raised (score 1→3, tag 1→4 cases).
- `2143d7e` the shared repeated-text, NULL-row, and annotate cases, with the DuckDB driver's `decide_many` and `annotate` branches extended under a narrow permission (record 0071) and the selftest extended so the new shapes fail when corrupted; DIVERGENCES carries the reasons.
- `6a829b8` the gate summary counts skips beside greens, and a bare `cargo test` in `libraries/rust` passes, closing the "17 tests" leftover. The conformance file is 84 cases.

**Records landed:** ADR 0037 (the C door serves every language that can call C) and ADR 0038 (DuckDB relate on the caller's database) at `79a7efd`; records 0069 (the R interrupt window), 0070 (the PostgreSQL deadline), 0071 (the two granted exceptions) at `aaccc63`. The merge-build fact has its own record at `sdlc/issues/2026-09-22-merge-trial-the-build-break-list.md` (`4b4f85b`).

**Open as of tip `6a829b8`:** the new defects 1–19 and the first-review leftovers above, plus the merge-build item. Commits from here on end with `refs surfaces-review-2` or `refs surfaces-review-1`, so this issue can be read against the branch by search.

## Progress note (2026-09-22, the fix wave)

The fixes for the groups above landed on the `surfaces` branch in 36 commits, `2143d7e..ce177c1`:

```
$ git rev-list --count 2143d7e..ce177c1        # 36
$ git diff --shortstat 2143d7e..ce177c1        # 150 files changed, 7971 insertions(+), 1481 deletions(-)
$ node sdlc/scripts/surfaces-ratchet.mjs       # 22039/22039 non-blank .rs lines
```

The four new trees (contract, standin, libraries, databases) grew 2316 net non-blank Rust lines in the wave — 19723 at `2143d7e` to 22039 at `ce177c1`, measured by the surfaces ratchet added this wave; the 7971/1481 figures are every file type.

Landed by the lanes, each with a test that fails against its pre-fix code: the security group (SQLite's schema-object reach, DuckDB's read-only single-statement relate with per-database routing and the file-access door, PostgreSQL's `@` requirement, narrowed grants, and the update-path revoke); the crash group (R's percent and encoding handling, the DuckDB nested-relate refusal, the C guard's teardown path, Ruby's tick rooting); the wrong-answer group (settings-keyed stand-in state, the deadline sentinels, the pre-request refusals, the sliced struct stream, the interrupt re-arm); the memory and performance items; the host-side leftovers (compile-time fixture, connector completion for C, Rust, and DuckDB, the cancel gaps, the error classes); the ADRs for the rulings; the shared panic guard in the contract; and this wave's process work (the workspace lint pass over every standalone workspace, the surfaces ratchet, the benches and package dry-runs wired into the gate, the Python artifact's home-path remap). The two unmaintained crates the audit found are recorded with replacement plans in the ticket beside this issue.

Still in flight or fenced when this note was written: the final commits of the DuckDB, Ruby, and TypeScript lanes, and the final verification pass that re-runs both reviews' repros as commands and corrects the documents.

## Final verification: the per-finding status (2026-09-22, branch tip `ed5a819`)

The full gate ran with the stub up on all ten ports and no key in the environment: `bash scripts/check_surfaces.sh` exit 0, `all landed checks green: green=927 skipped=73 diverged=18 failed=0 (wire: stub up)`. Every item below was re-run as a command.

| # | Finding | Status | Proof (command → observed) |
| --- | --- | --- | --- |
| 1 | SQLite CHECK reaches the functions | fixed | floor raised to 3.50.0; the schema suite shows `a CHECK constraint refuses (trusted OFF)` and `(trusted ON)`, with DEFAULT, view, trigger, generated column, and index expressions refused too; the stock 3.45.1 host refuses to load and the refusal names the floor |
| 2 | relate outside the caller's transaction | fixed | `ok the DELETE never happened, the caller's ROLLBACK included`; `ok relate refuses a second statement` |
| 3 | DuckDB file-access holes | fixed | `ok decide @file refuses with access off`, `ok annotate @set refuses with access off`, `ok relations @file refuses with access off` (no existence leak), `ok allowed_directories refuses the file it does not name`, `ok disabled_filesystems refuses a local read`, `ok a prepared relate re-checks file access at execution` |
| 4 | PostgreSQL bare paths, grants, update path | fixed | `ok bare text is refused with the @ form named, and never read`; `ok PUBLIC holds EXECUTE on none of the 14 functions`; `ok the update path's new function is extension-owned and out of PUBLIC's hands` |
| 5 | R percent crash and encodings | fixed | `r surface: the text crossing` — 9 checks: a percent text errors instead of crashing, the quoted text keeps its percents, valid UTF-8, latin1 converts, invalid bytes error |
| 6 | nested relate hangs | fixed | `ok a relate inside a relate refuses instead of hanging` |
| 7 | C guard aborts at teardown | fixed | `ENGINE_NULL=1 ./build/atexit_free` → exit 0 |
| 8 | Ruby tick proc unprotected | fixed | `the tick survived the collector: 71 ticks, 2000000 answers, no collection` (the honest limit is recorded in NOTES: no collection ran) |
| 9 | stand-in settings-keyed state | fixed | `cargo test --test settings_state` 2 passed; width 1 holds beside a wide engine and the descriptors stay flat |
| 10 | deadlines that silently mean none | fixed | Ruby: `the deadline bounds hold: usage for hostile budgets, minus one means none, zero stays spent`; Python: `test_a_computed_budget_that_landed_below_zero_is_refused`, `test_zero_stays_a_spent_deadline`, `test_no_deadline_is_the_explicit_none` PASSED; C: the negative-below-sentinel refusal; Node: the negative rule pinned in its suite |
| 11 | pandas refusal spends | fixed | `test_a_refused_pandas_frame_makes_no_request` and its pyarrow twin PASSED — zero requests, counted |
| 12 | sliced struct stream | fixed | `test_a_sliced_struct_stream_answers_the_callers_rows` PASSED; the whole-stream case beside it |
| 13 | DuckDB relate routing | fixed | `ok leaving USE behind reads the main table again`; `ok the closed database's file reopened in-process`; another process opened the closed database's file |
| 14 | 250 ms interrupt poisoning | fixed | `ok the interrupt stopped one query and the next query answered`; `ok ten interrupts near calls left the next query answering`; `ok DuckDB's own interrupt ended the query (InterruptException) and the next answered` |
| 15 | Ruby spurious wake-ups | fixed | `wake-ups and trapped signals leave calls alone; an interrupt fires only the call's own token` |
| 16 | C engine-scoped error slots | fixed | `ok two threads read their own messages, none crossed`; `failures stay on their own engine, and a reused address starts clean` (2 tests) |
| 17 | Python leak on refusal | fixed | `test_a_refused_frame_batch_is_released`, `test_a_refused_frame_schema_is_released` PASSED |
| 18 | @file re-read per row | fixed | `ok one question file open for twenty thousand rows`; `ok an edited question file is read again` |
| 19 | PostgreSQL warm rewrite | fixed | `ok warm runs at the width (about 1.2 s)`; the batch deadline shape: `ok a spent budget sends nothing on both batch shapes; a 50 ms budget ends a long batch in 0.37s` and the wire batch `ok a 100 ms budget ended a 64-record wire batch in 0.28s (two 300 ms rounds without it)` |

**First-review leftovers:** the strict question-set parser (the core's own parser, file order kept) — fixed, `49c3b78`; the fixture compile-time only — fixed, `a06b70d`; transport errors no longer retryable and the `ENGINE_*` names ruled — fixed, `a06b70d`, `c81679b`; the connector completed for C, Rust, and DuckDB (`request_digest` gone) — fixed, `5306d2c`, `361659e`, `22d7eb9`, `438521c` (grep proof in HANDOFF); warm judged under its own question — fixed; cancel gaps — SQLite single-row deadline/interrupt (`b40615b`), PostgreSQL batch deadline (`28c6dc4`), Python token and the Polars column poll (`177e6cd`, `d9f7a55`); error classes — Python `Cancelled` base and `SystemExit` (`177e6cd`), Ruby one base and a refused bad deadline (`832f4fc`); the C door's `details: false` audit arm, the header's `unsigned long` wording, and the SONAME decision — recorded in `libraries/c/DESIGN.md`; `cargo test` in `libraries/rust` passes bare (`6a829b8`).

**Phases 4 and 5 gaps:** the case counts, the surface count, the pg_cancel claim, and the "all complete and green" and "one dependency line" claims are corrected in HANDOFF, SURFACES, FINDINGS, and MERGE-NOTE (`ed5a819`); the merge-build fact, the retarget decision points, and the ratchet-and-gate items are MERGE-NOTE §8 and §4. **Still open, measured at `ed5a819`:** the private-name and home-path sweep (31 files name the deck's repository, 7 files carry absolute home paths, the built Python artifact's remap landed at `30f1c9d` but no check enforces either rule yet); the vendored DuckDB tool trim and its license note; the gate's remaining non-hermetic installs (npm, uv, Docker tags); the Mac-capable check scripts. Owner: the merge ticket, with the counts above.

**Merge and records:** the three rulings now have their ADRs (`79a7efd`, `aaccc63`) and the merge trial has its own issue (`4b4f85b`); commits from the wave on end with `refs surfaces-review-1` or `refs surfaces-review-2`; the panic guard and its message are the contract's (`4a62fdb`); the surfaces ratchet counts the new trees (`3025378`, pinned `ce177c1`); the two unmaintained crates are recorded with their plans (`f0e9126`).

**One new blocker found by this verification and fixed:** the Ruby wire interrupt proof segfaulted 3 of 3 pre-fix runs (exit 139; eight `ttb-worker` threads faulting in `Error::guard` over freed memory after the call returned — the poll's caller-token path returned without hearing the raise, and MRI delivered it inside the batch, skipping the scope that joins the workers). `hear_interrupts` now runs on every poll path and `without_gvl` drains after the call (`3238ed0`); the proof passes 4 of 4 and the whole Ruby surface check exits 0 with 84 ok-lines.

## Closure: phase 4 (hygiene, licenses, stub, hermeticity) — fixed 2026-09-22 evening, `37240fd` (refs surfaces-review-2)

The final verification had marked this group open with the merge ticket as owner; the orphaned phase-4 lane reclaimed and closed it. Proof commands run against the committed tree:

- Private names and home paths: 29 tracked files outside sdlc/ swept to generic wording; five home paths made relative or environment-based. `python3 scripts/check_no_private_refs.py` → `ok: no private repository names or home paths outside sdlc/`, exit 0; planted violation → exit 1; removed → exit 0. The check is wired into the gate.
- Root notes: the four NOTES-*.md files live under `sdlc/records/surfaces-notes/`; every reference updated (HANDOFF, FINDINGS, SURFACES, MERGE-NOTE, scripts).
- Licenses: MIT on all seven missing manifests (Python Cargo+pyproject, Ruby Cargo+gemspec with `Gem::Platform::CURRENT`, TypeScript addon Cargo+package.json, R rust manifest) plus the R `LICENSE` file; R's five-folders-up reference replaced by `tools/make-tarball.sh` (stages contract/standin/thinkthen-core, check runs `--stage-only`).
- Vendored DuckDB tools: 30 files down to the 5 the build reads, with a README recording upstream project, pin, license, and re-pin instructions.
- Wire stub in-repo: `tools/wire-stub` builds offline in 7 s; the gate builds it when missing, starts one stub per port (300 ms — a zero delay let batches finish before cancel proofs tripped, caught on the first gate run), stops only what it started; no experiments/ dependency.
- Hermeticity and Mac: `npm ci` on the lockfile; pinned requirements-dev.txt; postgres image by digest; Ruby toolchain by glob; `scripts/gate-hermeticity.md` records what cannot pin and why; Darwin spellings behind experimental headers; Mac evidence committed under `sdlc/records/2026-09-22-macos-artifacts/`.
- Gates: full gate green twice on the committed content (929 green / 73 skipped / 18 diverged / 0 failed, wire stub up on all ten ports); the gate's counter now counts TAP `not ok` lines, closing the hole where Node failures were invisible in totals.

With this, every row in both review tables reads fixed-with-proof except the items explicitly owned by the build team (the merge-as-a-build with the dependency retarget, the packing encode, row-at-a-time batching behind it) and the standing waiting-on-capability list.
