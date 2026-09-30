# 0304: One question cache and one batching path

Status: slice 1 landed; slice 2 ready. Plan: `sdlc/planning/cleanup-2026-09-30.md`, rulings 2 to 6.

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
