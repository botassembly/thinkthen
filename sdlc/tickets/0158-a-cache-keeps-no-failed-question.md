---
flow: build
priority: 158
opens: crates/thinkthen/src/engine/request.rs crates/thinkthen/src/engine/recorder.rs crates/thinkthen/src/core/reply.rs crates/thinkthen/tests/backend/cache_partial.rs crates/thinkthen/tests/backend/main.rs specification/records.md specification/recording.md sdlc/ratchet.json sdlc/records sdlc/tickets sdlc/issues
---

# 0158: A cache keeps no reply that failed a question

Status: landed 2026-09-26 (`sdlc/records/0158-build-a-cache-keeps-no-failed-question.md`). A fresh read-only code review accepted it. The coordinator accepted the ticket on 2026-09-26 after three fresh read-only reviews. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

A user runs `annotate`, or after ticket 0146 a batched `filter`, and the backend's reply leaves one question unanswered. Today the default cache installs that reply. Every rerun reads it and fails the same question without asking again. After this ticket a cache installs only a reply that answered every question, and the next run asks again. `--record DIR` alone still writes the reply, so `--replay` gives the live run's result.

ADR 0053 item 6 is the ruling. `specification/records.md` already says each digest keeps "the first complete response". The coordinator ruled the order on 2026-09-26: this ticket lands before ticket 0146's build starts. Batching makes a partial reply common, because every record of a batch is a question and the default cache is on. Ian can overturn each ruling.

## What happens today

Read from `origin/main` `c490f082`.

- `engine/request.rs::ask_prepared` installs the reply through `permit.finish` whenever it decodes (lines 147 to 160). A reply that fails to decode cancels the permit and writes nothing.
- `core/adapters/systemone/response.rs` decodes a reply when at least one question succeeds (lines 122 to 128). A failed question becomes `AnswerOutcome::Failed` in `core/reply.rs`. A reply where every question failed is a whole-reply decode error.
- `engine/recorder.rs` treats `--cache DIR` as recording and replaying on one folder. The default cache and `THINKTHEN_CACHE` are the same with a private folder. `--record DIR` alone records and does not replay.
- `specification/recording.md` line 84 says "A recorded partial reply replays the same good answers, failed markers, failure count, and exit 6." That rule is about `--record` and `--replay`.
- Local experiment 273 reproduced the defect with `annotate` and a hand-built cache entry whose reply answered `q1` and omitted `q2`. Two runs printed the same failure and exited 6. The loopback received 0 requests.
- The conformance backend's `/arm/malformed/missing_answer/v1` fails the last question of a reply and answers the rest (`tests/backend/loopback_arms.rs`).

## Design

`Recorder` gains `caches()`, true when it both records and replays. That is `--cache DIR`, `--record DIR --replay DIR` on one folder, `THINKTHEN_CACHE` and the default cache. `recording.md` names the two options on one folder a cache, so they follow the cache rule. `Reply` gains `failed_any()`, true when any outcome is `AnswerOutcome::Failed`.

In `ask_prepared`, after a live reply decodes, the engine checks both. When the recorder caches and the reply failed a question, it cancels the permit in place of finishing it. It still returns the reply, so the run prints and exits exactly as today. Every other case finishes the permit as today. A failed `permit.cancel()` propagates as a recording storage failure at exit 5, as a failed `permit.finish` does today and as the decode-error path's `cancel` already does. A cancelled permit releases the digest's lock, so a waiter on the same digest sends its own request, as it does after a failed owner today.

**A partial entry reads as a miss under a cache.** The coordinator ruled this on 2026-09-26. An entry already in a folder can hold a partial reply, one that failed a question beside a good answer: one written before this ticket by a default cache, or one written by `--record` alone. `Recorder` gains a sibling of `prepare_cancelled`, `prepare_checked(exchange, digest, cancel, complete)`, where `complete` takes an existing entry's response and returns false when it decodes against the plan with a failed question. `ask_prepared` is its only caller, and it passes the check only when the recorder caches. `prepare_cancelled` keeps its signature, so its test callers in `engine/request.rs` and `engine/deadline_tests.rs` stay as they are, and so does ticket 0155. A cache treats such an entry as it treats a damaged one today: it takes the digest's lock, checks again, sends, and replaces the entry atomically when the new reply answered every question. When the new reply fails a question too, the old entry stays and the next cached run asks again. `--replay` alone never runs the check. It replays a partial entry byte for byte, with the recorded failure markers and exit code. `--record` alone keeps today's rule for an existing entry.

Once `ask_prepared` calls `prepare_checked`, only tests call `prepare_cancelled`. The builder may then mark it `#[cfg(test)]` so the build has no dead code.

**An entry that fails to decode as a whole keeps today's behavior.** Under a cache, `complete` returns true for such an entry, so the recorder replays it. `ask_prepared`'s replay branch calls `built_in::decode`, which fails, and the run stops with the error that a live reply of those bytes gives. The entry stays in the folder. That covers an entry whose reply fails every question. This ticket leaves that path unchanged, and the coordinator ruled that it stays.

The engine is shared, so the libraries and SQL extensions follow the same rule under their caches. `relate` edges and `annotate` question entries follow it too.

### Pages

Every page sentence below follows both halves of the rule: a cache installs no partial reply, and a cache reads a partial entry as a miss.

- `recording.md`, line 13, the row "Both, with the same `DIR`": its cell becomes "A cache. An entry that exists is replayed, unless it holds a partial reply, one that failed a question beside a good answer. Such an entry counts as a miss. A request that is absent or missed goes to the backend, and its reply is recorded when it is complete. A complete reply answers every question."
- `recording.md`, line 59: replace "Only an exchange that succeeded and decoded is recorded. A failure is never recorded." with "A reply that fails as a whole is never recorded. That covers a transport failure, a refused or failed status, and a reply that does not decode or fails every question. A reply that fails some questions beside a good answer decodes. `--record` alone writes it, so `--replay` reproduces it. A cache does not install it."
- `recording.md`, line 64: "After a reply succeeds and decodes, the tool writes and syncs the complete entry." becomes "After a reply succeeds and decodes, the tool writes and syncs the complete entry. Under a cache it writes nothing for a partial reply." "It replaces a damaged final name atomically while it owns that digest's lock." becomes "It replaces a damaged final name atomically while it owns that digest's lock. Under a cache it replaces a final name that holds a partial reply the same way."
- `recording.md`, line 66: after "A successful answer through `--record` or `--cache` replaces a damaged entry atomically, and a later replay reads the repair." add "A complete answer through a cache also replaces a partial entry atomically. `--record` alone and `--replay` alone leave a partial entry as it is."
- `recording.md`, line 68: "A missing or damaged entry takes an exclusive operating-system lock for its digest, then checks the entry again." becomes "A missing or damaged entry, and under a cache an entry that holds a partial reply, takes an exclusive operating-system lock for its digest, then checks the entry again."
- `recording.md`, line 84: after the first sentence add "A cache, typed or default, installs no partial reply and reads a partial entry as a miss, so a cached run asks again. ADR 0053 item 6 and its amendment rule it. To retry a failed question, run the same command again under a cache. For a folder written by `--record` alone, run it again with `--cache` on that folder."
- `recording.md`, line 42, after "Both options on one folder are also the resume for a record run.": add "A resumed run asks again for records whose reply was partial, because a cache keeps no partial reply."
- `records.md`, line 115, after "A rerun with `--record DIR --replay DIR` on one folder answers the finished records from disk and pays only for the rest.": add "A record whose reply was partial counts as unfinished, so the rerun asks for it again."
- `records.md`, line 123, after "The same command run again replays those 399 and pays for the rest.": add "A record whose reply was partial has no entry, so the resumed run asks for it again."
- `records.md`, line 125: after "Each digest keeps the first complete response installed in the folder." add "A partial reply, one that failed a question beside a good answer, is not complete. Under ADR 0053 item 6 and its amendment, a cache does not install it, and reads an entry that holds one as a miss." "The owner checks again, sends only if the entry remains absent, and installs the complete response." becomes "The owner checks again, sends unless a complete entry now exists, and installs the response only when it is complete." An entry whose reply failed every question is not partial. A cache replays it and the run exits 4, as edge row 11 says.
- `relate.md`, line 70, stays as it is. It speaks of record and replay, and a recorded partial reply still reproduces there.

## Decisions

Each is the ticket author's call unless marked. Ian can overturn any of them.

1. **A cache never installs a reply that failed a question.** ADR 0053 item 6 ruled it. The coordinator ruled it.
2. **`--record` alone still writes a partial reply.** A recording exists to replay a run as it happened, and `recording.md` line 84 promises that.
3. **A first run's output does not change.** The fix touches only what the folder keeps. The same reply prints the same rows, markers and exit code. A cached rerun of a partial reply sends again, so its `meta.cached`, `meta.requests_sent` and `meta.usage` report a live request. That change is the point of the ticket.
4. **The check lives in `ask_prepared`, the one place a live reply is installed.** No second write path exists.
5. **A cache reads a partial entry as a miss, and `--replay` alone replays it.** The coordinator ruled it. It clears partial entries left in default caches before this ticket and in folders written by `--record` alone. A replay stays byte for byte.

## Edge cases

| Input | Expected |
| --- | --- |
| `annotate` under the default cache, a reply missing its last answer, run twice | Both runs print the same failure marker and exit 6. The backend sees 2 requests. The cache folder under the private `XDG_CACHE_HOME` holds no entry for that digest |
| The same under `--cache DIR`, and under `--record DIR --replay DIR` on one folder | The same. The folder holds no entry for that digest |
| The same under `--record DIR`, then `--replay DIR` | The replay prints the recorded failure marker and exits 6, with no request |
| A reply that answers every question under the default cache, run twice | The second run sends nothing, as today |
| A reply that fails every question | A whole-reply decode error at exit 4, as today. No entry |
| Two concurrent runs on one `--cache DIR` with the same partial reply | Each sends once. The folder holds no entry |
| A folder holding a partial entry, run under `--cache DIR` against a backend that now answers every question | One request. The complete reply replaces the entry. A second cached run sends nothing |
| A folder holding a partial entry, run under `--cache DIR` against a backend that still fails the question | One request each run. The old entry stays |
| The same folder under `--replay DIR` alone | The recorded failure marker and exit 6, byte for byte. No request |
| A default cache holding a partial entry written before this ticket | Read as a miss, as the `--cache DIR` rows say |
| A cache holding an entry that fails to decode as a whole | Unchanged. The replay fails to decode and the run exits 4, as edge row 5's live reply does. No request. The entry stays |

## Proof

One route serves every row. The test starts the conformance backend with `conformance_backend::Backend::start()`, as `tests/backend/loopback_arms.rs` does. `/arm/malformed/missing_answer/v1` fails the last question of a reply and answers the rest, and `/generic/v1` answers every question. `annotate` asks that file's two-question set, so a missing last answer is a partial reply. `decide` asks one question, so the same arm fails every question. `backend.count()` counts requests. The default-cache rows set `XDG_CACHE_HOME` and `HOME` to a private temporary folder, as `tests/backend/default_cache.rs` does, so no row touches the user's cache.

| Test | What it proves | Planted faults that turn it red |
| --- | --- | --- |
| `a_cache_keeps_no_failed_question`, new in `tests/backend/cache_partial.rs` | Edge rows 1 to 5. Each pins standard output, exit code and `backend.count()`. Rows 1 and 2 also pin that the cache folder holds no entry file for the digest after the first run. Row 1 checks the default cache folder under the private `XDG_CACHE_HOME`. The `--cache DIR` row runs once more as `--record DIR --replay DIR` on one folder. Row 5 runs `decide` twice under `--cache DIR` against the missing-answer arm: exit 4 both times, 2 requests, no entry | (a) Install every decoded reply: rows 1 and 2 go red on the no-entry assertion, because an entry file exists after the first run. The request count alone would not catch it, because the miss rule still sends on the second run. (b) Skip the write under `--record` too: the replay row exits 5. (c) Skip the write for every reply: the complete-reply row sends twice. (d) Leave `caches()` false for two options on one folder: that run's second pass sends nothing |
| `a_cache_reads_a_partial_entry_as_a_miss`, new in the same file | Edge rows 7 to 11. A partial entry is made two ways. The first records the missing-answer arm under `--record DIR`. The second records `/generic/v1` under `--record DIR`, then rewrites that entry's `response` line to the missing-answer arm's reply for the same request, keeping the request line. The arm row records the arm under `--record DIR`, then changes the first answer's text in that entry's `response` line, keeping the request line. The planted partial reply then differs from any reply the arm sends. A cached run against `/generic/v1` sends 1 request and replaces the entry, and a second sends none. A cached run against the arm sends 1 each time, and the entry file stays byte for byte equal to the planted one. `--replay DIR` alone prints the failure marker, exits 6 and sends none. The default-cache row plants the rewritten entry under the private `XDG_CACHE_HOME`. Row 11 records `decide` against `/generic/v1` under `--record DIR`, then rewrites the `response` line by hand to `{"model":"jev-latest","answers":{}}`, or to the conformance backend's pinned model, which fails `decide`'s one question. Recording against the arm writes nothing, because that reply fails to decode. A cached run against `/generic/v1` exits 4, sends none, and leaves the entry byte for byte | (e) Replay a partial entry under a cache: the rewritten-entry row sends nothing. (f) Run the check under `--replay` alone: the replay row exits 5 or sends. (g) Replace the entry with a reply that also failed: the arm's reply overwrites the changed first answer, so the arm row's byte comparison fails. (h) Read an entry that fails to decode as a whole as a miss: row 11 sends 1 request and exits 0 |

Edge row 6, two concurrent runs, is the one exception to that route. It rests on the existing `tests/backend/cache_locking.rs::backend_and_decode_failures_release_the_digest_lock`, which serves its replies from the in-process `Listener` harness, not the conformance backend. Its decode-failure case shows that an owner that cancels its permit releases the digest's lock, and that the waiter then sends its own request. This ticket's partial reply takes that same cancel path, so the row needs no new test. The same test pins, for an undecodable reply, the path edge row 5 takes: a decode error cancels the permit and writes no entry. No existing test sends a reply that fails every question through a cache, so row 5 joins the new test.

The four questions:

- **What behavior does it protect?** `records.md`'s "first complete response" under every cache, the miss rule for partial entries, and `recording.md` line 84 under `--record` and `--replay`.
- **What credible regression fails it?** Installing a partial reply, replaying a partial entry from a cache, dropping the recorded one, or checking under `--replay` alone.
- **Why does no existing test catch it?** No test sends a partial reply twice through a cache, or reads a partial entry through one, today.
- **Does it need a test-only hook?** No. The conformance backend's arms are ordinary replies, the rewritten entry is an ordinary file in a real folder, and `XDG_CACHE_HOME` is where the default cache really lives.

## Budgets

Nonblank lines, measured with `grep -c .`.

- `crates/thinkthen/src/engine/request.rs`, `engine/recorder.rs` and `core/reply.rs`: at most 35 net together.
- `crates/thinkthen/tests/backend/cache_partial.rs`: at most 150, new, and one `mod` line.
- Pages under `specification/`: at most 8 net.
- `sdlc/ratchet.json` moves to the measured total, at most 195 above main. The commit says what grew.
- No dependency.
- The `surfaces` rung runs, because the engine that the libraries share changes.

## Stop rules

1. Stop before crossing any budget by more than a tenth, or before adding a dependency.
2. Stop if the change needs a file outside this ticket's opens that ticket 0146, 0154 or 0155 opens. None of those tickets opens `engine/request.rs`, `engine/recorder.rs` or `core/reply.rs`. The expected merge points are these files, which this ticket shares with them:
   - Ticket 0146: `specification/records.md`, `crates/thinkthen/tests` (so `tests/backend/main.rs` and the new `cache_partial.rs`), `sdlc/issues`, `sdlc/ratchet.json`, `sdlc/records` and `sdlc/tickets`. Ticket 0146's build starts after this ticket lands, so ticket 0146 merges these.
   - Ticket 0154: `specification/records.md`, `sdlc/ratchet.json`, `sdlc/records` and `sdlc/tickets`.
   - Ticket 0155: `specification/recording.md`, `tests/backend/main.rs`, `sdlc/ratchet.json`, `sdlc/records` and `sdlc/tickets`. Ticket 0155 also opens `engine/deadline_tests.rs`, which this ticket leaves untouched.
   Each merge point is a page sentence, one `mod` line or the ratchet total. Whichever ticket lands second rebases and remeasures the ratchet.
3. Stop if `--record` alone stops writing a partial reply.
4. Stop if a first run's standard output, standard error or exit code changes, or if any run's does under `--record` alone or `--replay` alone. A cached rerun of a reply that failed a question changes by design: it sends again, so its rows carry `meta.cached` false, a fresh `meta.requests_sent` and `meta.usage`, and `status` counts one more request and no cache answer.
5. Stop if any plant stays green.
6. Stop if the build needs a live call. None is authorized. Never run `sdlc/scripts/live`.

## Scope and exclusions

Excluded: a recorded refusal entry. Any change to how a partial reply prints or exits. Batching itself, which ticket 0146 builds. `site/`.

## Routing

Builder: Claude (Opus subagent) in the lane the coordinator names. Reviewer: a fresh read-only Claude session for the design and for the code.

## Complexity

Contract 1; state and timing 1; reach 2; proof 1; cost of error 1; total 6. Final level: 2. The risk is a cache that drops complete replies too, which the complete-reply row guards.

## Deferred gaps

None.

## What Ian can overturn

- Decision 1: a cache never installs a reply that failed a question.
- Decision 2: `--record` alone still writes it.
- Decision 5, the coordinator's: a cache reads a partial entry as a miss and replaces it, and `--replay` alone replays it.
- The coordinator's order: this ticket lands before ticket 0146's build starts.

## Closes

Finding 4 of `sdlc/issues/2026-09-26-batching-design-review-before-0146.md`.

`sdlc/issues/2026-09-26-recording-page-says-a-failure-is-never-recorded.md`. The line 59 rewrite says which failures are never recorded and that a partial reply is. The line 84 sentences say how a user retries a failed question. The build moves the issue to `closed/`.

## Evidence

- Starts from: ADR 0053 item 6. Finding 4 of `sdlc/issues/2026-09-26-batching-design-review-before-0146.md`, reproduced in local experiment 273 with `annotate` and a hand-built cache entry. The code at `origin/main` `c490f082`: `engine/request.rs::ask_prepared`, `core/adapters/systemone/response.rs`, `core/reply.rs` and `engine/recorder.rs`. `specification/records.md`'s "first complete response" and `recording.md` line 84.
- Keeps: Every complete reply cached as today. `--record` alone and `--replay` alone as today, partial replies included. A first run's output, standard error and exit code, and those of every run under `--record` alone or `--replay` alone. A cached entry that fails to decode as a whole, which stops the run and stays in the folder. A cached rerun of a partial reply changes by design: it asks again, so `meta.cached`, `meta.requests_sent`, `meta.usage` and the usage totals show a sent request.
- Changes: A cache, typed or default, no longer installs a reply that failed a question, and reads an existing entry that holds one as a miss. Page sentences in `records.md` and `recording.md`.
- Proof: Two outside-in tests with eight plants, the existing cache-locking test for concurrent runs, and the `install`, `lint`, `test`, `spec` and `surfaces` rungs.
- Defers: Nothing.
