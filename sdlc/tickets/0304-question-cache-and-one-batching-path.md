# 0304: One question cache and one batching path

Status: slice 1 landed; slice 2 built, awaiting code review. Plan: `sdlc/planning/cleanup-2026-09-30.md`, rulings 2 to 6.

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
  - The crate ratchet rises by 2,409 to 108,326 lines: the key, store, fixture reader, converter and pipeline arrive while the old recorder and schedulers stay for slices 3 to 5. The C library ratchet rises by 7 to 4,096 for the `nm` check.
- Proof:
  - `tests/backend/question_cache.rs`: 100 records, then 120 that include them, send one request holding exactly records 101 to 120, and the second run reports 100 cache answers. A partial reply stores its good answers, and the rerun asks only the failed question. A new tag label sends only its three questions. Two child processes write one store at once, and a third run sends nothing.
  - `engine/store/tests.rs`: hit, miss, replace, a private new file, a read-only replay that writes nothing, a hot journal under replay, the busy limit and a stop during a wait, a lookup that waits through another writer's commit, a folder holding both files, a hand-edited fixture, and the merge rule.
  - `tests/cache_convert.rs`: converting twice writes identical bytes, each old form converts to its keys and origins, and demo 14's converted bytes equal its committed fixture.
  - `tests/backend/batching/too_large.rs`: a 413 makes three attempts and stores both halves; a refused first half sends no second half.
  - `tests/backend/batching.rs` `a_pause_sends_the_open_batch` and `tests/backend/scheduling.rs`: a slow pipe closes a request at the pause, and the window never passes W.
  - `engine/pipeline/tests.rs`: a spent deadline or fired cancel reads no input and sends nothing, and a deadline starts no waiting request. These replace the deleted annotate scheduler's deadline tests.
  - `libraries/c/tests/door/main.rs`: `nm` finds no exported `sqlite3_` symbol. The SQLite extension's check already requires exactly one export.
  - Every green demo replays unchanged apart from the three digest demos named above.
- Defers:
  - Slice 3: the public Rust API, Polars, the C door and the SQL hosts. They keep the request cache, `meta.batch` and the old schedulers. `tests/backend/public_json.rs` leaves `meta.requests` out of its comparison until then. Conformance case 18, annotate with two groups, still holds two requests, so the command's loopback runner skips it until slice 3 rewrites the shared cases.
  - Slice 4: `find`, `recognize` and `relate` keep the old store. The digest-lock and prune tests now run on `find`.
  - Slice 5: the old recorder, locks, marker, request-level prune and every old `DIGEST.json`.
  - `audit` and `diff` read `meta.batch.setting`. Command rows no longer carry it, so both treat such a run as setting 1. A later ticket should give them the setting another way.
  - Cache clearing and expiry.

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

- rusqlite sits behind the `cli` feature, so a library build without the command carries no SQLite yet.
- `cache convert` removes `thinkthen.sqlite` after it writes the fixture, because `--replay` refuses a folder holding both.
- A cache on a folder that holds only the fixture imports it into a new `thinkthen.sqlite` on first use. A run that stores nothing creates nothing, and the folder is checked before any key is read.
- The input reader runs on its own thread, so the coordinator never blocks on input.
- Each path keeps its own quote form: an annotate root group quotes the record's compact text as a JSON string, and the record functions quote the object.
- The dry run packs every record's questions and coalesces equal keys, with no lookup.
- A refused first half fails the second without sending it. Both halves show the parent's attempts, and only the first counts the parent's send.
- Coalescing covers keys already on their way within one call. The first waiting row counts the send; a row that joined it shares the answer and its usage share and counts no attempt.
- One document whose answer fails exits 4 with the reply failure. A stream gives the partial-reply stop. A stop that arrives after a document's row finished is ignored.
- The earliest ask's failure decides a row's failure, whatever order the replies arrive in.
- Annotate wraps a transport or status failure as a batch failure only when it spans several records or the input is a stream.
- A cached answer's model counts toward the run's `model` fact but takes no part in the live model check, by ADR 0111 section 4.
- A reply may name any nonblank model, so a hostile name reaches the safe mismatch message instead of a defect.
- A stored answer that no longer decodes is a miss under a cache and names its entry under `--replay`.
- The window test uses W = (jobs + 1) × batch, as section 4 gives it.
- The annotate plan's `request_count` counts packed requests once; `group_requests` counts the requests each group joins.

What the build found:

- The old annotate test `a_model_mismatch_cancels_groups_that_have_not_started` no longer applies: a record's model check runs after all its questions are answered, so no group of the record waits to be cancelled. Two tests still cover the mismatch and the stop of later work.
- `listener.requests()` drains its list, so a test reads the bodies once.
- A rerun test must reuse the same loopback listener, because the address is part of every key.
- Running demo 12 in place left a `thinkthen.sqlite` in its fixture folder, and one was committed by mistake. The demo now caches in a scratch copy, and `policy.py` refuses the file.
