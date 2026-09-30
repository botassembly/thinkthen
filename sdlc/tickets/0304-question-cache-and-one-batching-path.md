# 0304: One question cache and one batching path

Status: slice 3a fixed, awaiting re-review. Plan: `sdlc/planning/cleanup-2026-09-30.md`, rulings 2 to 6.

## Outcome

One engine path serves all ten functions and every surface. It builds backend questions, looks each one up in the cache, packs only the misses into requests, sends them, splits the replies, stores each answer alone with a taken-at time, and returns results in input order with bounded memory.

## Design first

Write ADR 0111. It replaces ADR 0048 item 5 and settles:

- The question key: model, backend address, shared context, and one question as sent. What else changes an answer and so belongs in it.
- How `choose`, `tag`, and `score` quote each record in its own question.
- How `find`, `recognize`, and `relate` map onto question entries.
- The store: one SQLite file or another format. Locking between processes. What happens to per-digest lock files and coalescing.
- Test recordings: the simpler of one store or two, and a converter for old request-level recordings.
- Token and request accounting across only the sent questions.
- Partial replies, refusals that halve a batch, and retries.
- Where the requests-per-minute limit sits.
- Which current code the path deletes: content-cut batching, duplicate schedulers, and the batch paths in `cli`, `public`, and the facade.
- The build order in slices that each land green.

A fresh reviewer returns ACCEPT or findings before any build.

## Slices

ADR 0111 names each slice's proof. Each lands green with `cargo test --workspace`, `policy.py` and a fresh code review.

1. The quoted wire form on every surface. The shared `conformance/` cases and committed recordings are rewritten at the request level. The `spec` gate stops replaying probes. No storage or scheduler change.
2. Question key, SQLite store, sorted JSON Lines fixtures, `cache convert`, and the pipeline for the seven record functions on the command. Proof includes 100 records, then 120, sending only the 20 new questions on the loopback backend.
3. Public Rust API, Polars eager and lazy, the C door and the SQL hosts on `ask_all`.
4. `find`, `recognize` and `relate` on `ask_all`. Their fixtures convert without loss.
5. Remove the old store, locks, marker, request-level prune, both old schedulers and every committed `DIGEST.json` outside probe history.

## Retained behavior

Output order, exit codes, the null-versus-failure rule, key secrecy, no key read on replay.

## Proof

Named in the ADR. At least: a batch of 100 records followed by a run of 120 that includes them sends only the 20 new questions, counted on a loopback backend.

## Deferred gaps

Cache clearing and expiry.

## Evidence

- Starts from: ADR 0048, ADR 0055, and the 2026-09-29 mapping of System One calls.
- Keeps: output order, exit codes, null versus failure, key secrecy, no key on replay.
- Changes: the cache key, the store, and one batching path, per ADR 0111.
- Proof: named per slice in ADR 0111.
- Defers: cache clearing and expiry.

### Slice 2 evidence

- Starts from: slice 1 (d8ac2c30b), ADR 0111 sections 2 to 7 and 9, and rulings 2 to 6 and 8 of `sdlc/planning/cleanup-2026-09-30.md`.
- Keeps: output order, exit codes, null versus failure, key secrecy, no key read on replay, the wire bytes of slice 1, and every demo's printed output. Demos 14, 16 and 27 change only where they show request digests, because `answers[].request` now names a question key. Demo 12 now caches in a scratch copy of its fixture.
- Changes:
  - `core/pack.rs` holds the question key, the packer, the splitter and the even token share.
  - `engine/store.rs` holds the bundled rusqlite store. `engine/store/fixture.rs` reads and writes `thinkthen.jsonl`. `engine/store/convert.rs` and `thinkthen cache convert DIR [--quote]` merge a folder's sources.
  - `engine/pipeline.rs` runs lookup, pack misses, send, split, store and in-order emit for `decide`, `filter`, `rank`, `choose`, `tag`, `score` and `annotate` on the command.
  - The annotate group scheduler, its slice batcher and the per-command batch metadata are deleted. `meta.batch` and `meta.batches` leave these rows, and `meta.requests` lists question keys.
  - `policy.py` pins rusqlite and refuses a committed `thinkthen.sqlite`. 42 committed folders gained `thinkthen.jsonl`; their old files stay until slice 5.
  - The loopback listener counts the questions it receives.
  - The recording, result, annotate and backends pages and the changelog describe the store.
  - The crate ratchet rises by 2,615 to 108,286 lines over main at 0324: the key, store, fixture reader, converter and pipeline arrive while the old recorder and schedulers stay for slices 3 to 5. The C library ratchet rises by 7 to 4,293 for the `nm` check.
- Proof:
  - `tests/backend/question_cache.rs`: 100 records, then 120 that include them, send one request holding exactly records 101 to 120, and the second run reports 100 cache answers. A partial reply stores its good answers, and the rerun asks only the failed question. A new tag label sends only its three questions. Two child processes write one store at once, and a third run sends nothing.
  - `tests/backend/question_cache.rs` also: a stored answer that no longer decodes is a miss that re-sends under a cache and a named exit 5 under `--replay`.
  - `tests/audit_write.rs` and `tests/diff.rs`: rows that name no batch setting leave a tuned `batch` alone and warn nothing.
  - `engine/store/tests.rs`: hit, miss, replace, a private new file, a read-only replay that writes nothing, a hot journal under replay, the busy limit and a stop during a wait each ending in under a second, a lookup that waits through another writer's commit, a folder holding both files, a hand-edited fixture, and the merge rule.
  - `tests/cache_convert.rs`: converting twice writes identical bytes, each old form converts to its keys and origins, and demo 14's converted bytes equal its committed fixture. A writer holding the live file makes convert wait, and its committed row reaches the fixture.
  - `tests/backend/batching/too_large.rs`: a 413 makes three attempts and stores both halves; a refused first half sends no second half.
  - `tests/backend/batching.rs` `a_pause_sends_the_open_batch` and `tests/backend/scheduling.rs`: a slow pipe closes a request at the pause, and the window never passes W.
  - `engine/pipeline/tests.rs`: a spent deadline or fired cancel reads no input and sends nothing, and a deadline starts no waiting request. These replace the deleted annotate scheduler's deadline tests.
  - `libraries/c/tests/door/main.rs`: `nm` finds no exported `sqlite3_` symbol. The SQLite extension's check already requires exactly one export.
  - Every green demo replays unchanged apart from the three digest demos named above.
- Defers:
  - Slice 3: the public Rust API, Polars, the C door and the SQL hosts. They keep the request cache, `meta.batch` and the old schedulers. `tests/backend/public_json.rs` leaves `meta.requests` out of its comparison until then. Conformance case 18, annotate with two groups, still holds two requests, so the command's loopback runner skips it until slice 3 rewrites the shared cases.
  - Slice 4: `find`, `recognize` and `relate` keep the old store. The digest-lock and prune tests now run on `find`.
  - Slice 5: the old recorder, locks, marker, request-level prune and every old `DIGEST.json`.
  - `audit` and `diff` read `meta.batch.setting`. Command rows no longer carry it, so both treat such a run's setting as unknown: `audit --write` leaves `batch` alone and neither warns. A later ticket should give them the setting another way.
  - Slice 3: a refused first half fails the untried second half (`engine/pipeline/send.rs`). Revisit when the SQL hosts continue past failures.
  - Slice 3: rusqlite moves out of the `cli` feature.
  - Slice 5: the store's folder-is-a-file check repeats `Recorder::of_private`'s; one goes.
  - Cache clearing and expiry.

### Slice 3 split

Slice 3 lands in four parts, one at a time. 3b, 3c and 3d touch disjoint files once 3a lands, so they can build in parallel lanes.

- **3a. The public Rust API on `ask_all`.** Detailed below.
- **3b. The C door and the SQL hosts.** They already call the public `*_with` methods, so 3a moves their batching. 3b removes the SQL host statics that 0323 named (`ACTIVE_THROTTLE`, `SEND_BUDGET`, the engine maps, SQLite's `STORED`, `ENGINE` and budget registry), proves an SQL row with a missing pointer fails alone while its neighbours answer, and runs every SQL and C conformance case on the question store. It also fixes the two facts of ADR 0111's 2026-09-30 amendment:
  - The SQLite extension's store runs on the host's SQLite, because `loadable_extension` skips the bundled build. On load, 3b checks the host SQLite's version (3.24 or later, for `ON CONFLICT DO UPDATE`) and thread mode (not single-thread, because the store opens connections on the pipeline's background thread), and refuses a host that fails either.
  - `libthinkthen.a` and R's static library export about 285 global `sqlite3_` symbols. 3b localizes them with a partial link before archiving, flips `the_static_library_still_exports_sqlite_symbols_until_slice_3b` to require zero, and closes `sdlc/issues/2026-09-30-static-library-exports-sqlite-symbols.md`.
- **3c. Polars eager and lazy.** Polars calls the public batch methods, so 3a moves its batching. 3c proves a lazy frame collected twice sends nothing the second time and removes any frame-only batching left.
- **3d. One owner for the send limits.** It waits for Ian's ruling on the process-wide throttle, below.

The process-wide throttle is an Ian ruling. ADR 0017's amendment from ticket 0077 says one process has one throttle, and a second engine with another throttle is refused. `PROCESS_WIDTH`, `WIDTH_CHILD` and the 1 ms `REBUILD_POLL` exist to keep that ruling safe across a fork. `PROCESS_GATES` (the 429 gate and the pacer) and `process_budget` (the request total) follow the same per-process rule. Cleanup step 4 lists process-wide statics as hacks. The two conflict, so 3a keeps them and 3d waits. Options for Ian:

1. Keep one process throttle, pacer, gate and request total. The statics stay as honest process state, as signals do. 3d only replaces the 1 ms poll with the 50 ms stop check. Cost: small. Recommended, because every engine in a process sends against one account's rate limit.
2. Give each engine its own throttle, pacer, gate and request total, and drop the "already active for this process" refusal. Every static and the fork rebuild for them go. Cost: the settings, records and backends pages, and the Python, TypeScript, PostgreSQL and DuckDB tests that pin the refusal. Two engines in one process could then send twice the rate.

Decision for 3d, 2026-09-30: option 1. One throttle per process stays, per Ian's ruling in ticket 0077, and 3d replaces the 1 ms `REBUILD_POLL` with the pipeline's 50 ms stop check. This is a coordinator default. Ian can overturn it.

### Slice 3a evidence

- Starts from: slice 2 (`1fbbe08e0`) and its Defers; ADR 0111 section 4 (the public Rust API row and the host demand bridge) and section 10; ticket 0323's inventory of engine imports of `public`; `sdlc/issues/closed/2026-09-30-public-batch-holds-one-send-under-a-throttle.md`.
- Keeps: every public name and signature; lazy `Batch` pulls, input order, the stop at the first failed record, `facts()`; `BatchSetting::Records(1)` returning each row before the next pull, as `specification/records.md` and `libraries/rust/README.md` promise; recoverable native rows and the spent rows after a send-budget denial; the wire bytes of slice 1; key secrecy and no key read on replay.
- Changes:
  - `filter`, `decide_many`, `choose_many`, `score_many`, `tag_many`, `rank`, `details_many`, `annotate` and `details_many_recoverable_with` run on `Engine::ask_all`. `ask_all` runs on a background thread. The calling thread is the host: it pulls a record on each ask and returns on each row.
  - Public rows lose `meta.batch` and list question keys in `meta.requests`, as the command's rows did in slice 2. A cached rerun answers from the store.
  - `public/batch/planned*`, `public/batch/annotation.rs`, the planning in `public/native_batch.rs`, `engine/facade/native_batch.rs`, `engine/facade/annotate/batch.rs`, `engine/facade/split.rs` and `engine/annotate_batching.rs` go, unless slice 4's functions still need a piece.
  - The pipeline and the store build without the `cli` feature, so rusqlite leaves it.
  - `SendBudget` and its reservations move from `public` into `engine`, with `public` re-exporting them. `engine` then names nothing from `public`, and `policy.py` refuses an engine reference to `public` as it refuses a core one.
- Proof, offline and counted on the loopback backend:
  - Through the public Rust API, 100 records and then 120 that include them send one request holding exactly records 101 to 120, and the second run reports 100 cache answers.
  - The throttle issue's test passes. Its batch-1 form contradicts the interactive promise above, so the test runs at batch 2 and fills throttle 2, and the interactive test keeps batch 1 at one send.
  - A stop during a pulled batch sends nothing new and joins every worker; rows keep input order with slow and fast replies mixed.
  - `policy.py` refuses a planted engine reference to `public`.
  - `sdlc/scripts/test`, `spec`, workspace clippy, `policy.py`, `tickets` and `lint` pass, with any binding whose build breaks fixed at its call site.
- Checks not run: the Ruby binding check. Its rb-sys probe fails because the pinned Ruby 3.4.11 is absent on this machine. The failure predates slice 3a.
- Defers: 3b, 3c and 3d above; `default_engine`, which holds an engine rather than a limit and which the Python, TypeScript, Ruby and R module functions call, to the binding pass of ticket 0314; slice 4's `find`, `recognize` and `relate`; slice 5's old store.

## What the build taught us

### Slice 1

- `max_evidence_bytes` bounds each quoted record's evidence bytes, as before. It now also counts the shared state, which is the 44-byte fixed sentence or the context. A value below 44 therefore refuses every quoted request. The batcher and the single-record plan each check the record.
- A question written as JSON cannot carry a quote. That record still goes as the state with its question unquoted, and a context with such a question still refuses, as ADR 0111 section 1 says.
- A structured tag description puts its questions in an array. The quote goes at the head of array element 0. The ADR does not cover this, and `specification/tag.md` now says it.
- Two paths quote a JSON record differently. An annotate root group quotes the record's compact text as a JSON string. The batcher quotes the object itself. Slice 2 must pick one form for its question key.
- The annotate slice path hands its questions to a group batcher, which quotes them. Quoting them earlier quoted them twice. The conformance runner hit the same trap through the facade. One quoting point per path avoids it.
- `recognize`, `find` and `relate` keep their own states. The requote script skipped them by their state shape.
- The requote script needed three rules. It parses a string state that looks like JSON as the object the batcher would quote. It treats an instruction as quoted only when a JSON value and ". " follow "The text is ", because some questions start with those words. It merges recordings that collide once the string and structured forms of one record meet, as in demo 06.
- 468 recordings were requoted and each carries `"quoted": true`. Recordings under `site/examples` and `site/recordings` were requoted mechanically. Marketing owns the prose under `site/`; these fixtures changed only by the script.
- The two `portable-frame` batching fixtures became copies of `portable-1` and `portable-2`, so they were removed.
- The `--plan` hint now says each question quotes the evidence it asks about.
- Every surface fake that counted per-record arrivals had to read a one-record quoted request as that record. The fakes still log and compare the true body.
- The full test rung reaches two readers of the state that the shared cases miss: the consumer's annotate parts test and the find-0040 probe self-test. Both now read the quoted form.
- Surface checks not run for slice 1: Dart and the sqlite3 tool are absent, and the Python pandas 2 pin is not in uv's offline cache. The main Python suite passes.

### Slice 2

Where ADR 0111 was silent, the build took the simpler option:

- `cache convert` removes `thinkthen.sqlite` after it writes the fixture, because `--replay` refuses a folder holding both. It holds the file's write lock from read to removal. It must not run beside a live writer, because a process holding the file open keeps writing to the removed file.
- A cache on a folder that holds only the fixture imports it in the new file's schema transaction. A run that stores nothing creates nothing.
- Each path keeps its own quote form: an annotate root group quotes the record's compact text as a JSON string, and the record functions quote the object.
- Coalescing covers keys already on their way within one call. The first waiting row counts the send; a row that joined it shares the answer and counts no attempt.
- A cached answer's model counts toward the run's `model` fact but takes no part in the live model check, by ADR 0111 section 4.
- A stored answer that no longer decodes is a miss under a cache and names its entry under `--replay`.

What the build found:

- rusqlite sets a 5-second busy timeout on every connection. The store turns it off, so its own wait, which checks the stop, is the only one.
- `listener.requests()` drains its list, and a rerun test must reuse the same listener, because the address is part of every key.
- Running demo 12 in place committed a `thinkthen.sqlite`. The demo now caches in a scratch copy, and `policy.py` refuses the file.

### Slice 3a

What the build changed beyond the evidence list:

- An annotate record's groups share one state, so its questions pack into one request, by ADR 0111 section 5. The consumer's parts test and the shared case `18-annotate-two-groups` recorded one request per group. The consumer now reads every question of a request, and it skips case 18 as the command's wire run does.
- A 413 on a stopping stream now sends both halves, by ADR 0111 section 6. The old path stopped after the first half.
- `thinkthen decide --plan` and the public plan count real option counts through `pipeline::options`. The command plan passed 0 before.
- A single public call listed only its first question key in its details. A tag call has one key per label, and `public_json` caught the gap. `keyed` now passes every key through.
- The library now needs rusqlite, so every consumer lock gained it. The SQLite extension also pins rusqlite, so Cargo resolves one rusqlite with both crates' features. `policy.py` now exempts the packages beneath a crate that a binding pins to the root's exact version. Otherwise its lock check read the extension's own packages as drift in thinkthen's tree.
- That unification also joins `bundled` and `loadable_extension` in one `libsqlite3-sys` inside the SQLite extension. ADR 0111 expected two SQLite copies there. `loadable_extension` skips the bundled build, so the store runs on the host's SQLite; ADR 0111's 2026-09-30 amendment records it, and 3b checks the host SQLite on load.
- ADR 0111 coalesces misses only within one call. The fork probe's digest-lock test therefore counts two sends for two concurrent single calls, and it keeps its proof that the parent's call never waits for a forked child.
- A cache on a folder that holds only `thinkthen.jsonl` imports it into `thinkthen.sqlite` beside it. The old folder binding refusal no longer applies to the record functions, because each key names its URL.
- The type corpus lost its three `meta.batch` and `meta.batches` shape cases with the schema definitions they checked.
- The ratchet fell from 108,545 to 105,563 lines, because the old batchers, their tests and the batch receipts went.

What the review fixes found:

- The Polars tests and the C door tests run outside `sdlc/scripts/test`, and both still pinned content cuts, request digests and case 18. They now read question keys, skip case 18 as the consumer does, and the portable fixture's five questions ride one request. The C door's settings case counts answer rows in `thinkthen.sqlite`.
- A call answered wholly from the store lost its `model` fact, because a stored answer told the process usage but not the call's facts. The C door's typed facts test caught it; the stored path now tells both.
- `policy.py`'s lock exemption skipped every package beneath a shared exact pin, so a changed `libsqlite3-sys` would have passed. It now skips only packages whose name the root tree never holds, and what only they reach. Planted locks prove a changed `libsqlite3-sys` or `cc` fails and a changed `hashlink` or an extension-only package passes. `hashlink` is absent from the root tree, so its version has nothing to drift from.
- The batch-1 read-ahead test now pins the real figure: no record read ahead of its rows, one read while the send is held.
- The throttle issue moved to `closed/` with a note that its batch-1 premise was wrong.
